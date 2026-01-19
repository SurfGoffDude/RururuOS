use std::path::Path;
use thiserror::Error;
use tracing::debug;

#[derive(Error, Debug)]
pub enum Model3DError {
    #[error("Failed to load model: {0}")]
    LoadError(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("No meshes in model")]
    NoMeshes,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub mesh_count: usize,
    pub material_count: usize,
    pub animation_count: usize,
    pub texture_count: usize,
    pub total_vertices: usize,
    pub total_faces: usize,
    pub has_normals: bool,
    pub has_uvs: bool,
    pub has_tangents: bool,
    pub has_colors: bool,
    pub has_bones: bool,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub tangents: Vec<[f32; 4]>,
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
    pub material_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub diffuse_color: [f32; 4],
    pub specular_color: [f32; 3],
    pub ambient_color: [f32; 3],
    pub emissive_color: [f32; 3],
    pub shininess: f32,
    pub opacity: f32,
    pub diffuse_texture: Option<String>,
    pub normal_texture: Option<String>,
    pub specular_texture: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub name: String,
    pub duration: f64,
    pub ticks_per_second: f64,
}

pub struct Model3D {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub animations: Vec<Animation>,
    pub info: ModelInfo,
}

impl Model3D {
    #[cfg(feature = "assimp")]
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Model3DError> {
        use russimp::scene::{PostProcess, Scene};

        let path = path.as_ref();
        debug!("Loading 3D model: {:?}", path);

        let scene = Scene::from_file(
            path.to_str().ok_or_else(|| Model3DError::LoadError("Invalid path".into()))?,
            vec![
                PostProcess::Triangulate,
                PostProcess::GenerateNormals,
                PostProcess::CalculateTangentSpace,
                PostProcess::JoinIdenticalVertices,
                PostProcess::OptimizeMeshes,
            ],
        )
        .map_err(|e| Model3DError::LoadError(e.to_string()))?;

        if scene.meshes.is_empty() {
            return Err(Model3DError::NoMeshes);
        }

        let mut meshes = Vec::new();
        let mut total_vertices = 0;
        let mut total_faces = 0;
        let mut has_normals = false;
        let mut has_uvs = false;
        let mut has_tangents = false;
        let mut has_colors = false;
        let mut has_bones = false;

        for mesh in &scene.meshes {
            let mut vertices = Vec::new();
            let mut normals = Vec::new();
            let mut uvs = Vec::new();
            let mut tangents = Vec::new();
            let mut colors = Vec::new();
            let mut indices = Vec::new();

            for vertex in &mesh.vertices {
                vertices.push([vertex.x, vertex.y, vertex.z]);
            }

            for normal in &mesh.normals {
                normals.push([normal.x, normal.y, normal.z]);
                has_normals = true;
            }

            if let Some(tex_coords) = mesh.texture_coords.first() {
                if let Some(coords) = tex_coords {
                    for coord in coords {
                        uvs.push([coord.x, coord.y]);
                    }
                    has_uvs = true;
                }
            }

            for tangent in &mesh.tangents {
                tangents.push([tangent.x, tangent.y, tangent.z, 1.0]);
                has_tangents = true;
            }

            if let Some(color_set) = mesh.colors.first() {
                if let Some(colors_data) = color_set {
                    for color in colors_data {
                        colors.push([color.r, color.g, color.b, color.a]);
                    }
                    has_colors = true;
                }
            }

            for face in &mesh.faces {
                for idx in &face.0 {
                    indices.push(*idx);
                }
            }

            if !mesh.bones.is_empty() {
                has_bones = true;
            }

            total_vertices += vertices.len();
            total_faces += mesh.faces.len();

            meshes.push(Mesh {
                name: mesh.name.clone(),
                vertices,
                normals,
                uvs,
                tangents,
                colors,
                indices,
                material_index: Some(mesh.material_index as usize),
            });
        }

        let mut materials = Vec::new();
        for mat in &scene.materials {
            let mut material = Material {
                name: String::new(),
                diffuse_color: [0.8, 0.8, 0.8, 1.0],
                specular_color: [1.0, 1.0, 1.0],
                ambient_color: [0.2, 0.2, 0.2],
                emissive_color: [0.0, 0.0, 0.0],
                shininess: 32.0,
                opacity: 1.0,
                diffuse_texture: None,
                normal_texture: None,
                specular_texture: None,
            };

            for prop in &mat.properties {
                match prop.key.as_str() {
                    "?mat.name" => {
                        if let russimp::material::PropertyTypeInfo::String(s) = &prop.data {
                            material.name = s.clone();
                        }
                    }
                    "$clr.diffuse" => {
                        if let russimp::material::PropertyTypeInfo::FloatArray(arr) = &prop.data {
                            if arr.len() >= 3 {
                                material.diffuse_color = [
                                    arr[0],
                                    arr[1],
                                    arr[2],
                                    if arr.len() > 3 { arr[3] } else { 1.0 },
                                ];
                            }
                        }
                    }
                    "$mat.shininess" => {
                        if let russimp::material::PropertyTypeInfo::FloatArray(arr) = &prop.data {
                            if !arr.is_empty() {
                                material.shininess = arr[0];
                            }
                        }
                    }
                    "$mat.opacity" => {
                        if let russimp::material::PropertyTypeInfo::FloatArray(arr) = &prop.data {
                            if !arr.is_empty() {
                                material.opacity = arr[0];
                            }
                        }
                    }
                    _ => {}
                }
            }

            materials.push(material);
        }

        let animations: Vec<Animation> = scene
            .animations
            .iter()
            .map(|anim| Animation {
                name: anim.name.clone(),
                duration: anim.duration,
                ticks_per_second: anim.ticks_per_second,
            })
            .collect();

        let info = ModelInfo {
            mesh_count: meshes.len(),
            material_count: materials.len(),
            animation_count: animations.len(),
            texture_count: 0,
            total_vertices,
            total_faces,
            has_normals,
            has_uvs,
            has_tangents,
            has_colors,
            has_bones,
        };

        Ok(Self {
            meshes,
            materials,
            animations,
            info,
        })
    }

    #[cfg(not(feature = "assimp"))]
    pub fn load<P: AsRef<Path>>(_path: P) -> Result<Self, Model3DError> {
        Err(Model3DError::UnsupportedFormat("Assimp not enabled".into()))
    }

    pub fn supported_formats() -> &'static [&'static str] {
        &[
            "gltf", "glb", "obj", "fbx", "dae", "3ds", "blend", "stl", "ply", "x3d",
        ]
    }

    pub fn is_supported(path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|ext| Self::supported_formats().contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_formats() {
        let formats = Model3D::supported_formats();
        assert!(formats.contains(&"gltf"));
        assert!(formats.contains(&"obj"));
        assert!(formats.contains(&"fbx"));
    }

    #[test]
    fn test_is_supported() {
        assert!(Model3D::is_supported(Path::new("model.obj")));
        assert!(Model3D::is_supported(Path::new("model.gltf")));
        assert!(!Model3D::is_supported(Path::new("model.txt")));
    }
}
