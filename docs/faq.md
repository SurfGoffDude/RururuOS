# RururuOS FAQ

## General Questions

### What is RururuOS?

RururuOS is a Linux distribution designed specifically for creative professionals. It provides:
- Pre-configured creative workflows
- Optimized audio/video production settings
- Professional color management
- Streamlined application installation

### Who is RururuOS for?

- Video editors
- 3D artists
- 2D designers and digital painters
- Audio producers and musicians
- Photographers
- Anyone doing creative work on Linux

### Is RururuOS free?

Yes, RururuOS is completely free and open source under the MIT license.

### What is RururuOS based on?

RururuOS is based on Arch Linux, providing access to the latest software through the Arch repositories and AUR.

---

## Installation

### What are the minimum system requirements?

- 64-bit CPU (x86_64 or ARM64)
- 4 GB RAM (8+ GB recommended)
- 20 GB storage (SSD recommended)
- OpenGL 3.3 compatible GPU

### Can I dual-boot with Windows/macOS?

Yes! The installer can detect existing operating systems and set up dual-boot automatically. See the [Installation Guide](installation-guide.md) for details.

### Does RururuOS support UEFI and Secure Boot?

RururuOS supports UEFI. Secure Boot must be disabled during installation.

### Can I install on a Raspberry Pi?

Yes! We provide ARM64 images for Raspberry Pi 4 and 5. A model with 4+ GB RAM is recommended.

### What about Apple Silicon Macs?

RururuOS runs on Apple Silicon via Asahi Linux. We provide an overlay package that adds RururuOS customizations on top of Asahi.

---

## Desktop Environment

### What desktop environment does RururuOS use?

RururuOS uses **Sway**, a tiling Wayland compositor. It's lightweight, fast, and keyboard-driven — perfect for productive creative work.

### Can I use GNOME or KDE instead?

Yes! RururuOS includes configuration files for GNOME and KDE. Install your preferred DE:
```bash
sudo pacman -S gnome
# or
sudo pacman -S plasma
```

### How do I customize the desktop?

Edit Sway config at `~/.config/sway/config` or use `rururu-settings` for common options.

### Where are the window controls?

Sway is a tiling window manager — windows are managed with keyboard shortcuts. Press `Super + ?` for a shortcut reference.

---

## Software

### What creative apps are included?

RururuOS doesn't bundle apps by default but makes them easy to install via workflow profiles:
- **Video**: DaVinci Resolve, Kdenlive, Handbrake
- **3D**: Blender, FreeCAD
- **2D**: Krita, GIMP, Inkscape
- **Audio**: Ardour, Bitwig, Audacity
- **Photo**: Darktable, RawTherapee, digiKam

### How do I install software?

```bash
# Using pacman
sudo pacman -S blender

# Using workflow installer
rururu-workflow install video

# Using Flatpak
flatpak install flathub org.blender.Blender
```

### Does RururuOS support Flatpak and Snap?

Flatpak is fully supported and recommended for sandboxed applications. Snap is not included by default but can be installed.

### Can I use proprietary software?

Yes, proprietary software like DaVinci Resolve works on RururuOS. Enable the appropriate repositories or download directly from vendors.

---

## Audio

### What audio system does RururuOS use?

RururuOS uses **PipeWire**, which provides:
- Low-latency audio
- JACK compatibility
- PulseAudio compatibility
- Bluetooth audio support

### Is RururuOS suitable for professional audio production?

Yes! The audio workflow profile configures:
- 64-sample buffer (1.3ms latency at 48kHz)
- Realtime scheduling
- Optimized CPU governor

### How do I connect audio applications?

Use `qpwgraph` for a graphical patchbay, or configure routing in your DAW.

---

## Graphics & Display

### Does RururuOS support NVIDIA GPUs?

Yes, both open-source (Nouveau) and proprietary NVIDIA drivers are supported. The proprietary driver is recommended for creative work.

### How do I enable GPU acceleration?

For NVIDIA:
```bash
sudo pacman -S nvidia nvidia-utils
```

For AMD, GPU acceleration works out of the box with Mesa.

### Does RururuOS support HDR?

Yes! HDR is supported on compatible displays. Enable via:
```bash
rururu-color hdr enable HDMI-1
```

### How do I calibrate my monitor?

Use `rururu-colorcal` for guided calibration, or load an ICC profile:
```bash
rururu-color profiles apply my-profile.icc --monitor HDMI-1
```

---

## Color Management

### Does RururuOS support color management?

Yes! RururuOS includes:
- ICC profile management
- OpenColorIO integration
- Display calibration tools
- Soft proofing support

### How do I use ACES/OCIO?

```bash
# Set OCIO config
export OCIO=/usr/share/ocio/aces_1.2/config.ocio

# Or use rururu-color
rururu-color ocio set /path/to/config.ocio
```

### Where can I get ICC profiles?

- Generate with `rururu-colorcal`
- Download from monitor manufacturer
- Use a hardware calibrator (ColorChecker, Spyder)

---

## Performance

### How do I optimize for creative work?

Use workflow profiles:
```bash
rururu-workflow activate video  # For video editing
rururu-workflow activate audio  # For audio production
```

### Why is my system slow?

Check resource usage:
```bash
rururu-monitor
```

Common solutions:
- Close unnecessary applications
- Check for runaway processes
- Ensure you're using GPU acceleration

### How do I reduce audio latency?

Activate the audio workflow:
```bash
rururu-workflow activate audio
```

This configures PipeWire for low-latency and enables realtime scheduling.

---

## Updates

### How do I update RururuOS?

```bash
sudo pacman -Syu
```

### Will updates break my system?

RururuOS follows Arch Linux's rolling release model. We recommend:
- Regular updates (weekly)
- Reading Arch news before major updates
- Keeping backups

### How do I rollback an update?

RururuOS uses BTRFS by default with snapshots. Restore from a previous snapshot:
```bash
sudo btrfs subvolume list /
sudo btrfs subvolume set-default <snapshot-id> /
reboot
```

---

## Community

### Where can I get help?

- **Documentation**: https://docs.rururu.os
- **Discord**: https://discord.gg/rururu
- **Forum**: https://forum.rururu.os
- **IRC**: #rururu on Libera.Chat
- **GitHub**: https://github.com/rururu/RururuOS

### How can I contribute?

- Report bugs on GitHub
- Submit pull requests
- Help with documentation
- Translate to your language
- Help other users in the community

### Where do I report bugs?

https://github.com/rururu/RururuOS/issues

Please include:
- RururuOS version
- Hardware info (`rururu-hwdetect`)
- Steps to reproduce
- Relevant logs

---

## Comparison

### How is RururuOS different from Ubuntu Studio?

| Feature | RururuOS | Ubuntu Studio |
|---------|----------|---------------|
| Base | Arch Linux | Ubuntu |
| Audio | PipeWire | PipeWire/JACK |
| Desktop | Sway (Wayland) | KDE Plasma |
| Updates | Rolling | LTS |
| Focus | All creative workflows | Audio production |

### How is RururuOS different from Fedora Design Suite?

| Feature | RururuOS | Fedora Design Suite |
|---------|----------|---------------------|
| Base | Arch Linux | Fedora |
| Desktop | Sway (Wayland) | GNOME |
| Updates | Rolling | Semi-annual |
| Apps | On-demand | Pre-installed |
| Workflows | Multiple profiles | Design-focused |
