use rururu_workflows::{WorkflowConfig, WorkflowProfile, WorkflowType};
use rururu_workflows::apps::{is_app_installed, install_app, launch_app, list_installed_creative_apps};
use rururu_workflows::system::{apply_system_settings, get_system_info};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    match args[1].as_str() {
        "list" => list_workflows(),
        "info" => {
            if args.len() < 3 {
                println!("Usage: rururu-workflow info <workflow>");
                return;
            }
            show_workflow_info(&args[2]);
        }
        "activate" => {
            if args.len() < 3 {
                println!("Usage: rururu-workflow activate <workflow>");
                return;
            }
            activate_workflow(&args[2]);
        }
        "status" => show_status(),
        "apps" => list_apps(),
        "install" => {
            if args.len() < 3 {
                println!("Usage: rururu-workflow install <workflow>");
                return;
            }
            install_workflow_apps(&args[2]);
        }
        "system" => show_system_info(),
        _ => print_usage(),
    }
}

fn print_usage() {
    println!("RururuOS Workflow Manager");
    println!();
    println!("Usage: rururu-workflow <command> [args]");
    println!();
    println!("Commands:");
    println!("  list              List available workflows");
    println!("  info <workflow>   Show workflow details");
    println!("  activate <name>   Activate a workflow");
    println!("  status            Show current workflow status");
    println!("  apps              List installed creative apps");
    println!("  install <name>    Install workflow applications");
    println!("  system            Show system information");
}

fn list_workflows() {
    println!("Available Workflows:");
    println!();
    
    for workflow_type in WorkflowType::all() {
        let profile = WorkflowProfile::get_profile(*workflow_type);
        println!("  {} - {}", workflow_type.name(), profile.description);
    }
}

fn show_workflow_info(name: &str) {
    let workflow_type = match name.to_lowercase().as_str() {
        "video" | "videoeditor" => WorkflowType::VideoEditor,
        "3d" | "3dartist" => WorkflowType::ThreeDArtist,
        "2d" | "2ddesigner" => WorkflowType::TwoDDesigner,
        "audio" | "audioproducer" => WorkflowType::AudioProducer,
        "photo" | "photographer" => WorkflowType::Photographer,
        "dev" | "developer" => WorkflowType::Developer,
        _ => WorkflowType::General,
    };
    
    let profile = WorkflowProfile::get_profile(workflow_type);
    
    println!("Workflow: {}", profile.name);
    println!("Description: {}", profile.description);
    println!();
    println!("Applications:");
    for app in &profile.applications {
        let installed = if is_app_installed(app) { "✓" } else { "✗" };
        println!("  [{}] {} ({})", installed, app.name, app.executable);
    }
    println!();
    println!("System Settings:");
    println!("  CPU Governor: {:?}", profile.system_settings.cpu_governor);
    println!("  GPU Performance: {}", profile.system_settings.gpu_performance_mode);
    println!("  Realtime Audio: {}", profile.system_settings.realtime_audio);
    println!();
    println!("Color Config:");
    println!("  Working Space: {}", profile.color_config.working_space);
    if let Some(ref ocio) = profile.color_config.ocio_config {
        println!("  OCIO Config: {}", ocio.display());
    }
}

fn activate_workflow(name: &str) {
    let workflow_type = match name.to_lowercase().as_str() {
        "video" | "videoeditor" => WorkflowType::VideoEditor,
        "3d" | "3dartist" => WorkflowType::ThreeDArtist,
        "2d" | "2ddesigner" => WorkflowType::TwoDDesigner,
        "audio" | "audioproducer" => WorkflowType::AudioProducer,
        "photo" | "photographer" => WorkflowType::Photographer,
        "dev" | "developer" => WorkflowType::Developer,
        _ => WorkflowType::General,
    };
    
    let profile = WorkflowProfile::get_profile(workflow_type);
    
    println!("Activating workflow: {}", profile.name);
    
    // Apply system settings
    if let Err(e) = apply_system_settings(&profile.system_settings) {
        eprintln!("Warning: Failed to apply system settings: {}", e);
    }
    
    // Set environment variables
    for (key, value) in &profile.environment {
        println!("  Setting {} = {}", key, value);
        env::set_var(key, value);
    }
    
    // Set OCIO config if specified
    if let Some(ref ocio_path) = profile.color_config.ocio_config {
        if ocio_path.exists() {
            env::set_var("OCIO", ocio_path);
            println!("  OCIO config: {}", ocio_path.display());
        }
    }
    
    // Save config
    if let Ok(mut config) = WorkflowConfig::load() {
        config.set_active_workflow(workflow_type);
        if let Err(e) = config.save() {
            eprintln!("Warning: Failed to save config: {}", e);
        }
    }
    
    println!("Workflow activated successfully!");
}

fn show_status() {
    match WorkflowConfig::load() {
        Ok(config) => {
            println!("Current Workflow: {}", config.active_workflow.name());
            
            if let Some(profile) = config.get_active_profile() {
                println!("Description: {}", profile.description);
                println!();
                println!("Installed Apps:");
                for app in &profile.applications {
                    if is_app_installed(app) {
                        println!("  ✓ {}", app.name);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
        }
    }
}

fn list_apps() {
    let apps = list_installed_creative_apps();
    
    println!("Installed Creative Applications:");
    println!();
    
    if apps.is_empty() {
        println!("  No creative applications found.");
    } else {
        for app in apps {
            println!("  • {}", app);
        }
    }
}

fn install_workflow_apps(name: &str) {
    let workflow_type = match name.to_lowercase().as_str() {
        "video" | "videoeditor" => WorkflowType::VideoEditor,
        "3d" | "3dartist" => WorkflowType::ThreeDArtist,
        "2d" | "2ddesigner" => WorkflowType::TwoDDesigner,
        "audio" | "audioproducer" => WorkflowType::AudioProducer,
        "photo" | "photographer" => WorkflowType::Photographer,
        _ => {
            println!("Unknown workflow: {}", name);
            return;
        }
    };
    
    let profile = WorkflowProfile::get_profile(workflow_type);
    let config = WorkflowConfig::load().unwrap_or_default();
    
    println!("Installing applications for: {}", profile.name);
    println!();
    
    for app in &profile.applications {
        if is_app_installed(app) {
            println!("  ✓ {} already installed", app.name);
        } else {
            println!("  Installing {}...", app.name);
            match install_app(app, config.package_manager) {
                Ok(_) => println!("    ✓ Installed successfully"),
                Err(e) => println!("    ✗ Failed: {}", e),
            }
        }
    }
}

fn show_system_info() {
    let info = get_system_info();
    
    println!("System Information:");
    println!();
    println!("  CPU Cores: {}", info.cpu_count);
    println!("  Memory: {} GB", info.memory_total_gb);
    println!("  GPU: {}", info.gpu);
    println!("  NVIDIA Driver: {}", if info.has_nvidia { "Yes" } else { "No" });
    println!("  AMD GPU: {}", if info.has_amd { "Yes" } else { "No" });
}
