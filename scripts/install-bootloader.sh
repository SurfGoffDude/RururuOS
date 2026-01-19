#!/bin/bash
# RururuOS Bootloader Installation Script
# Supports systemd-boot (UEFI) and GRUB (BIOS/UEFI)

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

log() { echo -e "${GREEN}[INFO]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

[[ $EUID -ne 0 ]] && error "This script must be run as root"

# Detect boot mode
if [[ -d /sys/firmware/efi ]]; then
    BOOT_MODE="uefi"
else
    BOOT_MODE="bios"
fi

log "Boot mode: $BOOT_MODE"

# Get root partition UUID
ROOT_UUID=$(findmnt -no UUID /)
[[ -z "$ROOT_UUID" ]] && error "Cannot determine root partition UUID"

log "Root UUID: $ROOT_UUID"

install_systemd_boot() {
    log "Installing systemd-boot..."
    
    bootctl install
    
    # Create loader.conf
    cat > /boot/loader/loader.conf << EOF
default  rururu.conf
timeout  3
console-mode max
editor   no
EOF

    # Create RururuOS entry
    cat > /boot/loader/entries/rururu.conf << EOF
title   RururuOS
linux   /vmlinuz-linux
initrd  /initramfs-linux.img
options root=UUID=$ROOT_UUID rw quiet splash
EOF

    # Create fallback entry
    cat > /boot/loader/entries/rururu-fallback.conf << EOF
title   RururuOS (Fallback)
linux   /vmlinuz-linux
initrd  /initramfs-linux-fallback.img
options root=UUID=$ROOT_UUID rw
EOF

    # RT kernel entry (if installed)
    if [[ -f /boot/vmlinuz-linux-rt ]]; then
        cat > /boot/loader/entries/rururu-rt.conf << EOF
title   RururuOS (RT Kernel)
linux   /vmlinuz-linux-rt
initrd  /initramfs-linux-rt.img
options root=UUID=$ROOT_UUID rw quiet splash threadirqs
EOF
    fi
    
    log "systemd-boot installed successfully"
}

install_grub_uefi() {
    log "Installing GRUB for UEFI..."
    
    pacman -S --noconfirm --needed grub efibootmgr
    
    grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id=RururuOS
    
    # Configure GRUB
    sed -i 's/GRUB_TIMEOUT=5/GRUB_TIMEOUT=3/' /etc/default/grub
    sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT="loglevel=3 quiet"/GRUB_CMDLINE_LINUX_DEFAULT="quiet splash"/' /etc/default/grub
    
    # Generate config
    grub-mkconfig -o /boot/grub/grub.cfg
    
    log "GRUB (UEFI) installed successfully"
}

install_grub_bios() {
    log "Installing GRUB for BIOS..."
    
    # Find boot disk
    BOOT_DISK=$(lsblk -no PKNAME $(findmnt -no SOURCE /boot) | head -1)
    [[ -z "$BOOT_DISK" ]] && BOOT_DISK=$(lsblk -no PKNAME $(findmnt -no SOURCE /) | head -1)
    
    log "Boot disk: /dev/$BOOT_DISK"
    
    pacman -S --noconfirm --needed grub
    
    grub-install --target=i386-pc /dev/$BOOT_DISK
    
    # Configure GRUB
    sed -i 's/GRUB_TIMEOUT=5/GRUB_TIMEOUT=3/' /etc/default/grub
    sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT="loglevel=3 quiet"/GRUB_CMDLINE_LINUX_DEFAULT="quiet splash"/' /etc/default/grub
    
    # Generate config
    grub-mkconfig -o /boot/grub/grub.cfg
    
    log "GRUB (BIOS) installed successfully"
}

# Main
echo ""
echo "RururuOS Bootloader Installer"
echo "=============================="
echo ""
echo "Select bootloader:"
echo "  1) systemd-boot (UEFI only, recommended)"
echo "  2) GRUB"
echo ""

read -p "Choice [1]: " CHOICE
CHOICE=${CHOICE:-1}

case $CHOICE in
    1)
        if [[ "$BOOT_MODE" != "uefi" ]]; then
            error "systemd-boot requires UEFI. Use GRUB instead."
        fi
        install_systemd_boot
        ;;
    2)
        if [[ "$BOOT_MODE" == "uefi" ]]; then
            install_grub_uefi
        else
            install_grub_bios
        fi
        ;;
    *)
        error "Invalid choice"
        ;;
esac

log "Bootloader installation complete!"
