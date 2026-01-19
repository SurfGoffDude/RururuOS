# RururuOS Installation Guide

## System Requirements

### Minimum Requirements
- **CPU**: 64-bit processor (x86_64 or ARM64)
- **RAM**: 4 GB
- **Storage**: 20 GB free space
- **Graphics**: OpenGL 3.3 / Vulkan 1.0 compatible GPU

### Recommended Requirements
- **CPU**: 8+ core processor
- **RAM**: 16 GB or more
- **Storage**: 100 GB SSD (NVMe preferred)
- **Graphics**: Dedicated GPU with 4+ GB VRAM

### Supported Hardware

| Platform | Status | Notes |
|----------|--------|-------|
| x86_64 PC | ✅ Full support | Intel/AMD processors |
| Raspberry Pi 4/5 | ✅ Full support | 4GB+ RAM recommended |
| Apple Silicon | ✅ Via Asahi | M1/M2/M3 Macs |
| NVIDIA GPU | ✅ Full support | Proprietary driver recommended |
| AMD GPU | ✅ Full support | Mesa RADV driver |
| Intel GPU | ✅ Full support | Integrated graphics |

---

## Installation Methods

### Method 1: Live USB (Recommended)

#### Step 1: Download ISO
Download the latest RururuOS ISO from the official website:
- **x86_64**: `rururu-1.0.0-x86_64.iso`
- **Checksum**: `rururu-1.0.0-x86_64.iso.sha256`

#### Step 2: Verify Download
```bash
sha256sum -c rururu-1.0.0-x86_64.iso.sha256
```

#### Step 3: Create Bootable USB
**On Linux:**
```bash
sudo dd if=rururu-1.0.0-x86_64.iso of=/dev/sdX bs=4M status=progress
sync
```

**On macOS:**
```bash
sudo dd if=rururu-1.0.0-x86_64.iso of=/dev/rdiskX bs=4m
```

**On Windows:**
Use [Rufus](https://rufus.ie/) or [balenaEtcher](https://etcher.io/)

#### Step 4: Boot from USB
1. Insert USB drive
2. Restart computer
3. Enter BIOS/UEFI (usually F2, F12, Del, or Esc)
4. Select USB drive as boot device
5. Save and exit

#### Step 5: Run Installer
1. Select "Install RururuOS" from boot menu
2. Follow the Calamares installer wizard
3. Select your preferred workflow during installation
4. Wait for installation to complete
5. Reboot and remove USB

---

### Method 2: Raspberry Pi

#### Step 1: Download Image
```bash
wget https://releases.rururu.os/arm64/rururu-1.0.0-arm64-rpi.img.xz
```

#### Step 2: Flash to SD Card
```bash
xz -d rururu-1.0.0-arm64-rpi.img.xz
sudo dd if=rururu-1.0.0-arm64-rpi.img of=/dev/sdX bs=4M status=progress
sync
```

#### Step 3: First Boot
1. Insert SD card into Raspberry Pi
2. Connect display, keyboard, and power
3. Wait for first-boot setup to complete
4. Login: `rururu` / `rururu`
5. Run `rururu-asahi-setup` to complete configuration

---

### Method 3: Apple Silicon (Asahi Linux)

#### Step 1: Install Asahi Linux
```bash
curl https://alx.sh | sh
```
Follow the Asahi installer prompts.

#### Step 2: Download RururuOS Overlay
```bash
wget https://releases.rururu.os/arm64/rururu-1.0.0-arm64-asahi-overlay.tar.gz
```

#### Step 3: Apply Overlay
```bash
sudo tar -xzf rururu-1.0.0-arm64-asahi-overlay.tar.gz -C /
sudo systemctl enable rururu-firstboot
sudo reboot
```

#### Step 4: User Setup
After reboot:
```bash
rururu-asahi-setup
```

---

## Post-Installation

### First Boot Wizard
On first login, the RururuOS Setup Wizard will guide you through:
1. **Language selection**
2. **Hardware detection** — automatic driver recommendations
3. **Workflow selection** — choose your creative focus
4. **Application installation** — one-click install
5. **System settings** — appearance, updates, privacy

### Manual Configuration
If you skipped the wizard, you can run it manually:
```bash
rururu-setup
```

### Essential Commands
```bash
# System settings
rururu-settings

# File manager
rururu-files

# System monitor
rururu-monitor

# Color calibration
rururu-colorcal

# Workflow management
rururu-workflow list
rururu-workflow activate video

# Hardware detection
rururu-hwdetect
```

---

## Dual Boot Setup

### With Windows

1. **Shrink Windows partition** using Disk Management
2. **Disable Fast Startup** in Power Options
3. **Boot from USB** and install to free space
4. **GRUB** will auto-detect Windows

### With macOS (Intel)
1. Use Disk Utility to create a new partition
2. Boot from USB installer
3. Install to the new partition
4. Hold Option key at boot to select OS

---

## Troubleshooting Installation

### Boot Issues

**UEFI Secure Boot:**
```
Disable Secure Boot in BIOS settings
Or enroll RururuOS signing key
```

**Black screen after boot:**
```
Add 'nomodeset' to kernel parameters
Press 'e' in GRUB, add to linux line
```

### Graphics Issues

**NVIDIA driver not loading:**
```bash
sudo pacman -S nvidia nvidia-utils
sudo mkinitcpio -P
reboot
```

**AMD GPU artifacts:**
```bash
# Use latest Mesa
sudo pacman -Syu mesa
```

### Audio Issues

**No sound:**
```bash
# Restart PipeWire
systemctl --user restart pipewire wireplumber
```

### Network Issues

**WiFi not detected:**
```bash
# Check interface
ip link show

# Enable NetworkManager
sudo systemctl enable --now NetworkManager
```

---

## Getting Help

- **Documentation**: https://docs.rururu.os
- **Community Forum**: https://forum.rururu.os
- **Discord**: https://discord.gg/rururu
- **Bug Reports**: https://github.com/rururu/RururuOS/issues
- **IRC**: #rururu on Libera.Chat

---

## Next Steps

After installation, see:
- [User Manual](user-manual.md) — complete usage guide
- [Workflow Guide](workflows.md) — optimize for your creative work
- [Color Management](color-management.md) — calibration and profiles
