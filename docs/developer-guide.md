# RururuOS Developer Guide

## Architecture Overview

RururuOS is built with Rust and follows a modular architecture:

```
RururuOS/
├── packages/                 # Rust crates
│   ├── rururu-file-handler/  # Universal file handling
│   ├── rururu-files/         # File manager GUI
│   ├── rururu-settings/      # System settings GUI
│   ├── rururu-monitor/       # System monitor GUI
│   ├── rururu-colorcal/      # Color calibration GUI
│   ├── rururu-color/         # Color management daemon
│   └── rururu-workflows/     # Workflow profiles
├── installer/
│   ├── hardware-detect/      # Hardware detection
│   ├── post-install-wizard/  # Setup wizard GUI
│   └── calamares/            # Installer config
├── config/                   # DE configurations
├── iso/                      # ISO build scripts
└── docs/                     # Documentation
```

---

## Development Setup

### Prerequisites

```bash
# Arch Linux
sudo pacman -S base-devel git rust cargo

# Dependencies
sudo pacman -S wayland libxkbcommon dbus pkg-config

# Optional: cross-compilation
cargo install cross
```

### Clone and Build

```bash
git clone https://github.com/rururu/RururuOS.git
cd RururuOS

# Build all packages
cargo build --workspace

# Build release
cargo build --release --workspace

# Run tests
cargo test --workspace
```

### IDE Setup

**VS Code:**
```json
{
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.checkOnSave.command": "clippy"
}
```

**JetBrains (RustRover/CLion):**
- Install Rust plugin
- Open `Cargo.toml` as project

---

## Package Structure

### rururu-file-handler

Core file handling library with codec detection and thumbnail generation.

```rust
use rururu_file_handler::{FileHandler, FileType};

let handler = FileHandler::new();
let file_type = handler.detect_type("video.mp4")?;
let thumbnail = handler.generate_thumbnail("video.mp4", 256)?;
```

**Key modules:**
- `detection.rs` — MIME type detection
- `thumbnail.rs` — Thumbnail generation
- `metadata.rs` — File metadata extraction
- `codecs.rs` — Codec registry

### rururu-files

File manager built with Iced GUI framework.

```rust
// Entry point
fn main() -> iced::Result {
    RururuFiles::run(Settings::default())
}
```

**Key modules:**
- `app.rs` — Main application state
- `file_list.rs` — File list view
- `preview.rs` — File preview panel
- `sidebar.rs` — Navigation sidebar
- `tags.rs` — Tagging system
- `batch.rs` — Batch operations

### rururu-color

Color management daemon with D-Bus interface.

```rust
use rururu_color::{ColorConfig, IccManager, OcioManager};

// Load configuration
let config = ColorConfig::load()?;

// Manage ICC profiles
let icc = IccManager::new()?;
icc.apply_profile_to_monitor("profile.icc", "HDMI-1")?;

// OCIO integration
let ocio = OcioManager::new()?;
ocio.load_config("/path/to/config.ocio")?;
```

**D-Bus Interface:**
```
org.rururu.ColorManagement1
├── EnableColorManagement(enabled: bool)
├── ListMonitors() -> Vec<MonitorInfo>
├── SetMonitorProfile(monitor: str, profile: str)
├── ListIccProfiles() -> Vec<ProfileInfo>
├── EnableHdr(monitor: str, enabled: bool)
└── GetOcioConfig() -> String
```

### rururu-workflows

Workflow profile management.

```rust
use rururu_workflows::{WorkflowConfig, WorkflowProfile, WorkflowType};

// Get profile
let profile = WorkflowProfile::get_profile(WorkflowType::VideoEditor);

// Apply system settings
rururu_workflows::system::apply_system_settings(&profile.system_settings)?;
```

---

## GUI Development with Iced

RururuOS GUI apps use [Iced](https://iced.rs/), a cross-platform GUI library.

### Basic Structure

```rust
use iced::{Application, Command, Element, Settings, Theme};

struct MyApp {
    // State
}

#[derive(Debug, Clone)]
enum Message {
    // Events
}

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        "My App".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        // Handle messages
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // Build UI
        iced::widget::text("Hello").into()
    }
}
```

### Common Patterns

**Async commands:**
```rust
Command::perform(
    async { fetch_data().await },
    |result| Message::DataFetched(result),
)
```

**Subscriptions (timers, events):**
```rust
fn subscription(&self) -> Subscription<Message> {
    iced::time::every(Duration::from_secs(1))
        .map(|_| Message::Tick)
}
```

---

## D-Bus Integration

Use `zbus` for D-Bus services:

```rust
use zbus::{dbus_interface, ConnectionBuilder};

struct MyService;

#[dbus_interface(name = "org.rururu.MyService1")]
impl MyService {
    async fn do_something(&self, arg: String) -> String {
        format!("Result: {}", arg)
    }
}

async fn run_service() -> zbus::Result<()> {
    let service = MyService;
    let _conn = ConnectionBuilder::session()?
        .name("org.rururu.MyService")?
        .serve_at("/org/rururu/MyService", service)?
        .build()
        .await?;
    
    std::future::pending::<()>().await;
    Ok(())
}
```

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_detection() {
        let handler = FileHandler::new();
        let file_type = handler.detect_type("test.png").unwrap();
        assert_eq!(file_type, FileType::Image);
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
use rururu_file_handler::FileHandler;

#[test]
fn test_thumbnail_generation() {
    let handler = FileHandler::new();
    let thumb = handler.generate_thumbnail("tests/fixtures/test.jpg", 256);
    assert!(thumb.is_ok());
}
```

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific package
cargo test -p rururu-file-handler

# With output
cargo test -- --nocapture
```

---

## Contributing

### Code Style

- Follow `rustfmt` formatting
- Use `clippy` for linting
- Write documentation for public APIs
- Add tests for new functionality

```bash
# Format code
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings
```

### Git Workflow

1. Fork the repository
2. Create feature branch: `git checkout -b feature/my-feature`
3. Make changes and commit
4. Push and create Pull Request

### Commit Messages

```
type(scope): description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example:
```
feat(file-handler): add HEIC thumbnail support

Added libheif integration for HEIC/HEIF image thumbnails.
Closes #123
```

---

## Building Releases

### Local Build

```bash
cargo build --release --workspace
```

### Cross-Compilation

```bash
# ARM64
cross build --release --target aarch64-unknown-linux-gnu --workspace
```

### ISO Build

```bash
cd iso
make full
```

---

## API Reference

Full API documentation:
```bash
cargo doc --workspace --open
```

Online: https://docs.rururu.os/api/

---

## Resources

- **Iced Book**: https://book.iced.rs/
- **zbus Book**: https://dbus2.github.io/zbus/
- **Rust Book**: https://doc.rust-lang.org/book/
- **Wayland Book**: https://wayland-book.com/
