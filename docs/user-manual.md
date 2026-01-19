# RururuOS User Manual

## Table of Contents
1. [Getting Started](#getting-started)
2. [Desktop Environment](#desktop-environment)
3. [File Manager](#file-manager)
4. [System Settings](#system-settings)
5. [Workflows](#workflows)
6. [Color Management](#color-management)
7. [Audio Production](#audio-production)
8. [Keyboard Shortcuts](#keyboard-shortcuts)

---

## Getting Started

### First Login
After installation, log in with your created user account. The Sway desktop environment will start automatically.

### Desktop Overview
RururuOS uses **Sway**, a tiling Wayland compositor:
- **Waybar** — top panel with workspaces, system info
- **Wofi** — application launcher (Super key)
- **Mako** — notification daemon
- **Foot** — terminal emulator

### Quick Actions
| Action | Shortcut |
|--------|----------|
| Open launcher | `Super` |
| Open terminal | `Super + Enter` |
| Close window | `Super + Shift + Q` |
| Switch workspace | `Super + 1-9` |
| Move to workspace | `Super + Shift + 1-9` |
| Toggle fullscreen | `Super + F` |
| Screenshot | `Super + Shift + S` |

---

## Desktop Environment

### Sway Basics

#### Window Management
Sway is a tiling window manager — windows automatically arrange themselves:

```
┌─────────────────────────────────────┐
│              Window 1               │
├─────────────────┬───────────────────┤
│    Window 2     │     Window 3      │
└─────────────────┴───────────────────┘
```

**Layouts:**
- `Super + E` — Toggle split direction
- `Super + S` — Stacking layout
- `Super + W` — Tabbed layout
- `Super + Space` — Toggle floating

#### Focus and Movement
- `Super + Arrow keys` — Move focus
- `Super + Shift + Arrow keys` — Move window
- `Super + R` — Resize mode

### Workspaces
RururuOS comes with pre-configured creative workspaces:
1. **Main** — General work
2. **Edit** — Video/Photo editing
3. **3D** — Blender, FreeCAD
4. **Audio** — DAW, audio tools
5. **Web** — Browser, research
6-9. **Custom** — Your choice

### Themes
Switch themes in Settings or manually:
```bash
# Tokyo Night (default)
cp ~/.config/sway/themes/tokyo-night.conf ~/.config/sway/theme.conf

# Catppuccin
cp ~/.config/sway/themes/catppuccin.conf ~/.config/sway/theme.conf

swaymsg reload
```

---

## File Manager

### RururuFiles
Launch with `rururu-files` or click the file manager icon.

#### Navigation
- **Sidebar** — Quick access to common locations
- **Toolbar** — Navigation, view options, search
- **Preview** — Right-side preview panel

#### Features
- **Thumbnails** — Images, videos, documents
- **Quick Preview** — Spacebar to preview files
- **Tags** — Organize files with color-coded tags
- **Batch Operations** — Select multiple files for bulk actions

#### Tags System
1. Select files
2. Right-click → "Add Tag"
3. Choose color and name
4. Filter by tags in sidebar

#### Batch Operations
1. Select multiple files (Ctrl+Click or Shift+Click)
2. Right-click → "Batch Operations"
3. Choose: Copy, Move, Delete, Rename, Compress

---

## System Settings

Launch with `rururu-settings`.

### Pages

#### Appearance
- **Theme** — Light/Dark mode
- **Accent Color** — System-wide accent
- **Font** — UI font selection
- **Icons** — Icon theme

#### Displays
- **Resolution** — Output resolution
- **Refresh Rate** — Monitor refresh rate
- **Scale** — HiDPI scaling
- **Night Light** — Blue light filter
- **VRR** — Variable refresh rate

#### Audio
- **Output** — Speaker/headphone selection
- **Input** — Microphone selection
- **Volume** — Master volume control
- **PipeWire** — Audio server status

#### Keyboard
- **Layout** — Keyboard layouts
- **Shortcuts** — Custom keybindings
- **Input Method** — IME configuration

#### Network
- **WiFi** — Wireless networks
- **Wired** — Ethernet settings
- **VPN** — VPN connections

#### Power
- **Profile** — Performance/Balanced/Power Saver
- **Suspend** — Auto-suspend settings
- **Battery** — Battery status (laptops)

#### Storage
- **Disks** — Mounted drives
- **Usage** — Storage breakdown
- **Cleanup** — Remove temporary files

#### About
- **System Info** — OS version, hardware
- **Updates** — Check for updates

---

## Workflows

### Workflow Profiles
RururuOS optimizes system settings based on your creative work:

#### Video Editor
- **Apps**: DaVinci Resolve, Kdenlive, Handbrake
- **Settings**: Performance governor, GPU acceleration
- **Color**: Rec.709, ACES support

```bash
rururu-workflow activate video
```

#### 3D Artist
- **Apps**: Blender, FreeCAD
- **Settings**: Maximum GPU performance
- **Color**: ACEScg working space

```bash
rururu-workflow activate 3d
```

#### 2D Designer
- **Apps**: Krita, GIMP, Inkscape
- **Settings**: Balanced performance
- **Color**: Adobe RGB, print proofing

```bash
rururu-workflow activate 2d
```

#### Audio Producer
- **Apps**: Ardour, Bitwig, Audacity
- **Settings**: Low-latency audio, realtime priority
- **PipeWire**: 64-sample buffer

```bash
rururu-workflow activate audio
```

#### Photographer
- **Apps**: Darktable, RawTherapee, digiKam
- **Settings**: Color-accurate display
- **Color**: ProPhoto RGB, soft proofing

```bash
rururu-workflow activate photo
```

### Managing Workflows
```bash
# List all workflows
rururu-workflow list

# Show workflow details
rururu-workflow info video

# Check current status
rururu-workflow status

# Install workflow apps
rururu-workflow install audio
```

---

## Color Management

### Color Calibration Tool
Launch with `rururu-colorcal`.

#### Calibration Process
1. **Warm Up** — Let monitor warm up (15-30 min)
2. **Brightness** — Set to target brightness
3. **Contrast** — Adjust contrast level
4. **Gamma** — Set gamma curve (usually 2.2)
5. **White Point** — Set color temperature (D65 = 6500K)
6. **Verification** — Check calibration

#### Test Patterns
- **Color Bars** — Primary and secondary colors
- **Gradient** — Smooth gradient transitions
- **Black Level** — Shadow detail visibility
- **White Level** — Highlight clipping
- **Gamma** — Gamma accuracy check
- **Dead Pixel** — Find dead/stuck pixels

### ICC Profiles
```bash
# List installed profiles
rururu-color profiles list

# Apply profile to monitor
rururu-color profiles apply /path/to/profile.icc --monitor HDMI-1

# Install new profile
rururu-color profiles install ~/Downloads/my-profile.icc
```

### OpenColorIO
For professional color workflows:
```bash
# Set OCIO config
export OCIO=/usr/share/ocio/aces_1.2/config.ocio

# Or use rururu-color
rururu-color ocio set /path/to/config.ocio
```

---

## Audio Production

### PipeWire Configuration

#### Low-Latency Setup
RururuOS uses PipeWire for professional audio:

```bash
# Check audio status
wpctl status

# Set low-latency mode
mkdir -p ~/.config/pipewire/pipewire.conf.d
cat << 'EOF' > ~/.config/pipewire/pipewire.conf.d/low-latency.conf
context.properties = {
    default.clock.rate = 48000
    default.clock.quantum = 64
    default.clock.min-quantum = 32
}
EOF
systemctl --user restart pipewire
```

#### JACK Compatibility
PipeWire provides JACK compatibility:
```bash
# Connect applications
qpwgraph  # GUI patchbay

# Command-line
pw-jack ardour8
```

### Realtime Audio
For the audio workflow:
```bash
# Activate audio workflow (enables realtime)
rururu-workflow activate audio

# Manual setup
sudo usermod -aG audio $USER
# Reboot required
```

---

## Keyboard Shortcuts

### Global Shortcuts
| Action | Shortcut |
|--------|----------|
| Application launcher | `Super` |
| Terminal | `Super + Enter` |
| File manager | `Super + E` |
| Close window | `Super + Shift + Q` |
| Lock screen | `Super + L` |
| Logout | `Super + Shift + E` |

### Window Management
| Action | Shortcut |
|--------|----------|
| Focus left/right/up/down | `Super + ←/→/↑/↓` |
| Move window | `Super + Shift + ←/→/↑/↓` |
| Resize mode | `Super + R` |
| Fullscreen | `Super + F` |
| Floating toggle | `Super + Shift + Space` |
| Split horizontal | `Super + H` |
| Split vertical | `Super + V` |

### Workspaces
| Action | Shortcut |
|--------|----------|
| Switch to workspace 1-9 | `Super + 1-9` |
| Move window to workspace | `Super + Shift + 1-9` |
| Previous workspace | `Super + Tab` |

### Media Keys
| Action | Key |
|--------|-----|
| Volume up | `XF86AudioRaiseVolume` |
| Volume down | `XF86AudioLowerVolume` |
| Mute | `XF86AudioMute` |
| Brightness up | `XF86MonBrightnessUp` |
| Brightness down | `XF86MonBrightnessDown` |

### Screenshots
| Action | Shortcut |
|--------|----------|
| Full screen | `Print` |
| Select area | `Super + Shift + S` |
| Active window | `Super + Print` |

### Creative Shortcuts
| Action | Shortcut |
|--------|----------|
| Launch Blender | `Super + Shift + B` |
| Launch Krita | `Super + Shift + K` |
| Launch Darktable | `Super + Shift + D` |
| Launch Ardour | `Super + Shift + A` |
| Launch Kdenlive | `Super + Shift + V` |

---

## Tips and Tricks

### Performance Optimization
```bash
# Check current governor
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor

# Set performance mode
rururu-workflow activate video  # Includes performance governor
```

### GPU Acceleration
```bash
# NVIDIA: Check if working
nvidia-smi

# AMD: Check Vulkan
vulkaninfo | grep deviceName
```

### Disk Performance
```bash
# Check if TRIM is enabled (SSD)
sudo fstrim -v /
```

### Memory Management
```bash
# Reduce swappiness for creative work
sudo sysctl vm.swappiness=10
```

---

## Getting Help

- Press `Super + /` for in-app help
- `man rururu-<command>` for command documentation
- Visit https://docs.rururu.os for full documentation
- Join Discord: https://discord.gg/rururu
