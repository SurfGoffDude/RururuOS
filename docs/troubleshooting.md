# RururuOS Troubleshooting Guide

## Boot Issues

### System Won't Boot

**Symptom**: Black screen or boot loop after installation.

**Solutions**:

1. **Try recovery mode**
   - At GRUB menu, select "Advanced options" → "Recovery mode"

2. **Add nomodeset**
   ```
   At GRUB, press 'e' to edit
   Add 'nomodeset' to the linux line
   Press Ctrl+X to boot
   ```

3. **Check boot order**
   - Enter BIOS/UEFI
   - Ensure correct drive is first boot device

### UEFI Secure Boot Error

**Symptom**: "Security violation" or unsigned kernel error.

**Solution**:
```
1. Enter BIOS/UEFI settings
2. Find "Secure Boot" option
3. Disable Secure Boot
4. Save and exit
```

### GRUB Not Showing

**Symptom**: Boots directly to Windows or other OS.

**Solution**:
```bash
# Boot from Live USB
sudo mount /dev/sdX2 /mnt
sudo mount /dev/sdX1 /mnt/boot/efi
sudo arch-chroot /mnt
grub-install --target=x86_64-efi --efi-directory=/boot/efi
grub-mkconfig -o /boot/grub/grub.cfg
exit
reboot
```

---

## Graphics Issues

### NVIDIA Driver Problems

**Symptom**: Low resolution, no GPU acceleration, or black screen.

**Solution**:
```bash
# Install NVIDIA driver
sudo pacman -S nvidia nvidia-utils nvidia-settings

# Rebuild initramfs
sudo mkinitcpio -P

# Reboot
sudo reboot
```

**For laptop with hybrid graphics:**
```bash
sudo pacman -S nvidia-prime

# Run app on NVIDIA
prime-run blender
```

### AMD GPU Issues

**Symptom**: Screen artifacts, crashes, or poor performance.

**Solution**:
```bash
# Update Mesa
sudo pacman -Syu mesa vulkan-radeon

# Check Vulkan
vulkaninfo | grep deviceName
```

**For older GPUs:**
```bash
# Use AMDGPU driver instead of radeon
# Add to kernel parameters: amdgpu.si_support=1 amdgpu.cik_support=1
```

### Screen Tearing

**Symptom**: Horizontal lines during video or scrolling.

**Solution for NVIDIA:**
```bash
# Edit /etc/environment
__GL_SYNC_TO_VBLANK=1
```

**Solution for AMD/Intel:**
```bash
# In ~/.config/sway/config
output * adaptive_sync on
```

### HiDPI/Scaling Issues

**Symptom**: UI too small or blurry on high-resolution displays.

**Solution:**
```bash
# In ~/.config/sway/config
output * scale 2

# Or fractional:
output * scale 1.5
```

**For XWayland apps:**
```bash
# In ~/.config/environment.d/hidpi.conf
GDK_SCALE=2
QT_SCALE_FACTOR=2
```

---

## Audio Issues

### No Sound

**Symptom**: No audio output.

**Step 1: Check PipeWire**
```bash
systemctl --user status pipewire pipewire-pulse wireplumber
```

**Step 2: Restart audio services**
```bash
systemctl --user restart pipewire wireplumber
```

**Step 3: Check output device**
```bash
wpctl status
wpctl set-default <sink-id>
```

**Step 4: Unmute**
```bash
wpctl set-mute @DEFAULT_AUDIO_SINK@ 0
wpctl set-volume @DEFAULT_AUDIO_SINK@ 100%
```

### Audio Crackling/Latency

**Symptom**: Audio pops, crackles, or high latency.

**Solution:**
```bash
# Increase buffer size
mkdir -p ~/.config/pipewire/pipewire.conf.d
cat << 'EOF' > ~/.config/pipewire/pipewire.conf.d/latency.conf
context.properties = {
    default.clock.quantum = 1024
    default.clock.min-quantum = 512
}
EOF
systemctl --user restart pipewire
```

### Bluetooth Audio

**Symptom**: Bluetooth headphones not working.

**Solution:**
```bash
# Install Bluetooth support
sudo pacman -S bluez bluez-utils

# Enable service
sudo systemctl enable --now bluetooth

# Pair device
bluetoothctl
> power on
> scan on
> pair XX:XX:XX:XX:XX:XX
> connect XX:XX:XX:XX:XX:XX
```

---

## Network Issues

### WiFi Not Working

**Symptom**: No wireless networks visible.

**Step 1: Check interface**
```bash
ip link show
# Look for wlan0 or wlp*
```

**Step 2: Enable interface**
```bash
sudo ip link set wlan0 up
```

**Step 3: Check NetworkManager**
```bash
sudo systemctl enable --now NetworkManager
nmcli device wifi list
nmcli device wifi connect "SSID" password "password"
```

### Slow Network

**Symptom**: Poor network performance.

**Solution:**
```bash
# Disable power saving
sudo iw dev wlan0 set power_save off

# Make permanent
cat << 'EOF' | sudo tee /etc/NetworkManager/conf.d/wifi-powersave.conf
[connection]
wifi.powersave = 2
EOF
sudo systemctl restart NetworkManager
```

---

## Application Issues

### Application Won't Start

**Step 1: Run from terminal**
```bash
blender  # See error messages
```

**Step 2: Check dependencies**
```bash
ldd /usr/bin/blender | grep "not found"
```

**Step 3: Reinstall**
```bash
sudo pacman -S blender
```

### Flatpak Apps Issues

**Symptom**: Flatpak apps crash or have permission issues.

**Solution:**
```bash
# Grant permissions
flatpak override --user --filesystem=home org.example.App

# Reset permissions
flatpak override --user --reset org.example.App
```

### XWayland Apps Blurry

**Symptom**: Some apps look blurry on HiDPI.

**Solution:**
```bash
# For GTK apps
GDK_SCALE=2 gimp

# For Qt apps
QT_SCALE_FACTOR=2 some-qt-app

# For Electron apps
--force-device-scale-factor=2
```

---

## Performance Issues

### System Slow

**Step 1: Check resource usage**
```bash
rururu-monitor
# or
htop
```

**Step 2: Check disk I/O**
```bash
iotop
```

**Step 3: Check for high CPU process**
```bash
ps aux --sort=-%cpu | head
```

### High Memory Usage

**Solution:**
```bash
# Clear caches
sync; echo 3 | sudo tee /proc/sys/vm/drop_caches

# Reduce swappiness
sudo sysctl vm.swappiness=10
```

### SSD Performance

**Enable TRIM:**
```bash
sudo systemctl enable --now fstrim.timer
sudo fstrim -v /
```

---

## Color Management Issues

### ICC Profile Not Applying

**Solution:**
```bash
# Check service
systemctl --user status rururu-color-daemon

# Apply manually
rururu-color profiles apply /path/to/profile.icc --monitor HDMI-1
```

### Colors Look Wrong

**Step 1: Calibrate display**
```bash
rururu-colorcal
```

**Step 2: Check color profile**
```bash
rururu-color profiles list
```

---

## Workflow Issues

### Workflow Not Activating

**Solution:**
```bash
# Check current workflow
rururu-workflow status

# Activate manually
rururu-workflow activate video

# Check logs
journalctl --user -u rururu-workflow
```

### Apps Not Installing

**Solution:**
```bash
# Check package manager
sudo pacman -Syu

# Install manually
sudo pacman -S blender

# Or via Flatpak
flatpak install flathub org.blender.Blender
```

---

## Recovery

### Chroot from Live USB

```bash
# Boot from Live USB
# Mount partitions
sudo mount /dev/sdX2 /mnt
sudo mount /dev/sdX1 /mnt/boot/efi  # If UEFI

# Chroot
sudo arch-chroot /mnt

# Fix issues...
# Exit and reboot
exit
sudo reboot
```

### Reinstall Packages

```bash
# Reinstall all packages
sudo pacman -Qqn | sudo pacman -S -

# Reinstall specific package
sudo pacman -S --overwrite '*' package-name
```

### Reset Configuration

```bash
# Backup and reset Sway config
mv ~/.config/sway ~/.config/sway.bak
cp -r /etc/skel/.config/sway ~/.config/

# Reset all configs
rm -rf ~/.config/*
cp -r /etc/skel/. ~/
```

---

## Getting Help

If none of the above solutions work:

1. **Collect system info:**
   ```bash
   rururu-hwdetect > ~/hw-info.txt
   journalctl -b > ~/journal.txt
   ```

2. **Search existing issues:**
   https://github.com/rururu/RururuOS/issues

3. **Ask on Discord:**
   https://discord.gg/rururu

4. **Create bug report:**
   Include hardware info and logs
