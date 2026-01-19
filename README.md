# RururuOS

**Creative Linux distribution for designers, 3D artists, and content creators.**

Built on Arch Linux with universal codec support and Rust-based tooling.

## Features

- **Universal Codec Support** — All video, audio, image, and 3D formats
- **Low-Latency Audio** — PipeWire with JACK compatibility
- **Creative Workflow** — Optimized for Blender, DaVinci Resolve, Krita, etc.
- **Rust Components** — Modern, fast, and safe system utilities

## Architecture Support

| Architecture | Status |
|-------------|--------|
| x86_64 | ✅ Full support |
| ARM64 | 🚧 In development |

## Quick Start

### Development (macOS)

See [mac-develop.md](mac-develop.md) for detailed instructions.

```bash
# Install Lima for Linux VM
brew install lima

# Create Arch Linux VM
limactl create --name=rururu template://archlinux
limactl start rururu
limactl shell rururu
```

### Build from source

```bash
# Install dependencies (in Arch Linux)
./scripts/install-deps.sh

# Build Rust components
cargo build --release

# Build ISO (requires root)
sudo ./scripts/build-iso.sh
```

## Project Structure

```
RururuOS/
├── packages/
│   ├── rururu-codecs/       # Meta-package for all codecs
│   ├── rururu-file-handler/ # Universal file format daemon
│   └── rururu-desktop/      # Desktop environment components
├── scripts/
│   ├── build-iso.sh         # ISO builder
│   └── install-deps.sh      # Dependency installer
├── config/
│   ├── packages.x86_64      # Package list for ISO
│   ├── sysctl.conf          # Kernel parameters
│   └── limits.conf          # Resource limits
├── iso/
│   └── airootfs/            # Live ISO filesystem
├── idea.md                  # Project concept
├── todo.md                  # Development roadmap
└── mac-develop.md           # macOS development guide
```

## Supported Formats

### Video
H.264, H.265, AV1, VP9, ProRes, DNxHD, and 50+ more

### Audio
FLAC, MP3, AAC, Opus, DSD, and 40+ more

### Image
JPEG, PNG, WebP, AVIF, RAW (all cameras), EXR, JPEG XL, and more

### 3D
glTF, FBX, OBJ, USD, Blender, and 20+ more via Assimp

### Documents
PDF, DOCX, ODT, Markdown, and more

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License. See [LICENSE](LICENSE) for details.
