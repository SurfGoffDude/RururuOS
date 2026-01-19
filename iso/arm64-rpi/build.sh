#!/bin/bash
# RururuOS ARM64 Raspberry Pi Image Build Script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work"
OUT_DIR="${SCRIPT_DIR}/out"

VERSION="1.0.0"
IMAGE_NAME="rururu-${VERSION}-arm64-rpi"
IMAGE_SIZE="8G"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

check_deps() {
    log_info "Checking dependencies..."
    
    local deps=("qemu-user-static" "binfmt-support" "debootstrap" "parted" "dosfstools")
    local missing=()
    
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &>/dev/null && ! dpkg -l "$dep" &>/dev/null; then
            missing+=("$dep")
        fi
    done
    
    if [ ${#missing[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing[*]}"
        exit 1
    fi
}

clean() {
    log_info "Cleaning previous build..."
    sudo umount -R "${WORK_DIR}/mnt" 2>/dev/null || true
    sudo losetup -D 2>/dev/null || true
    sudo rm -rf "${WORK_DIR}"
    mkdir -p "${WORK_DIR}" "${OUT_DIR}"
}

create_image() {
    log_info "Creating disk image (${IMAGE_SIZE})..."
    
    truncate -s "${IMAGE_SIZE}" "${WORK_DIR}/disk.img"
    
    # Partition: 256MB boot (FAT32) + rest root (ext4)
    sudo parted -s "${WORK_DIR}/disk.img" mklabel msdos
    sudo parted -s "${WORK_DIR}/disk.img" mkpart primary fat32 1MiB 257MiB
    sudo parted -s "${WORK_DIR}/disk.img" mkpart primary ext4 257MiB 100%
    sudo parted -s "${WORK_DIR}/disk.img" set 1 boot on
    
    # Setup loop device
    LOOP_DEV=$(sudo losetup -fP --show "${WORK_DIR}/disk.img")
    log_info "Loop device: ${LOOP_DEV}"
    
    # Format partitions
    sudo mkfs.vfat -F 32 -n BOOT "${LOOP_DEV}p1"
    sudo mkfs.ext4 -L rootfs "${LOOP_DEV}p2"
    
    # Mount
    mkdir -p "${WORK_DIR}/mnt"
    sudo mount "${LOOP_DEV}p2" "${WORK_DIR}/mnt"
    sudo mkdir -p "${WORK_DIR}/mnt/boot"
    sudo mount "${LOOP_DEV}p1" "${WORK_DIR}/mnt/boot"
}

bootstrap_system() {
    log_info "Bootstrapping Arch Linux ARM..."
    
    # Download Arch Linux ARM tarball
    local ALARM_URL="http://os.archlinuxarm.org/os/ArchLinuxARM-rpi-aarch64-latest.tar.gz"
    
    if [ ! -f "${WORK_DIR}/alarm.tar.gz" ]; then
        wget -O "${WORK_DIR}/alarm.tar.gz" "${ALARM_URL}"
    fi
    
    # Extract
    sudo bsdtar -xpf "${WORK_DIR}/alarm.tar.gz" -C "${WORK_DIR}/mnt"
    
    log_info "System bootstrapped"
}

configure_system() {
    log_info "Configuring system..."
    
    local ROOT="${WORK_DIR}/mnt"
    
    # Hostname
    echo "rururu-rpi" | sudo tee "${ROOT}/etc/hostname"
    
    # Locale
    echo "en_US.UTF-8 UTF-8" | sudo tee "${ROOT}/etc/locale.gen"
    echo "LANG=en_US.UTF-8" | sudo tee "${ROOT}/etc/locale.conf"
    
    # Timezone
    sudo ln -sf /usr/share/zoneinfo/UTC "${ROOT}/etc/localtime"
    
    # fstab
    cat << 'EOF' | sudo tee "${ROOT}/etc/fstab"
# RururuOS fstab for Raspberry Pi
/dev/mmcblk0p1  /boot   vfat    defaults        0       0
/dev/mmcblk0p2  /       ext4    defaults        0       1
EOF

    # Network
    sudo mkdir -p "${ROOT}/etc/systemd/network"
    cat << 'EOF' | sudo tee "${ROOT}/etc/systemd/network/20-wired.network"
[Match]
Name=eth*

[Network]
DHCP=yes
EOF

    # Enable services
    sudo arch-chroot "${ROOT}" systemctl enable systemd-networkd
    sudo arch-chroot "${ROOT}" systemctl enable systemd-resolved
    sudo arch-chroot "${ROOT}" systemctl enable sshd
    
    # Create rururu user
    sudo arch-chroot "${ROOT}" useradd -m -G wheel -s /bin/bash rururu || true
    echo "rururu:rururu" | sudo arch-chroot "${ROOT}" chpasswd
    echo "root:root" | sudo arch-chroot "${ROOT}" chpasswd
    
    # Sudoers
    echo "%wheel ALL=(ALL) ALL" | sudo tee "${ROOT}/etc/sudoers.d/wheel"
    
    # RururuOS packages list
    cat << 'EOF' | sudo tee "${ROOT}/etc/rururu-packages.txt"
# Desktop
sway
waybar
foot
wofi

# Audio
pipewire
pipewire-pulse
wireplumber

# Creative (lightweight)
gimp
inkscape
audacity

# Development
git
rust
EOF

    log_info "System configured"
}

install_rururu_packages() {
    log_info "Installing RururuOS packages..."
    
    local ROOT="${WORK_DIR}/mnt"
    
    # Copy pre-built ARM64 binaries if available
    local BIN_DIR="${SCRIPT_DIR}/../../target/aarch64-unknown-linux-gnu/release"
    
    if [ -d "${BIN_DIR}" ]; then
        local binaries=(
            "rururu-files"
            "rururu-settings"
            "rururu-monitor"
            "rururu-colorcal"
            "rururu-workflow"
        )
        
        for bin in "${binaries[@]}"; do
            if [ -f "${BIN_DIR}/${bin}" ]; then
                sudo cp "${BIN_DIR}/${bin}" "${ROOT}/usr/local/bin/"
                log_info "Installed ${bin}"
            fi
        done
    else
        log_warn "ARM64 binaries not found. Build with: cargo build --release --target aarch64-unknown-linux-gnu"
    fi
    
    # Copy configs
    sudo mkdir -p "${ROOT}/etc/skel/.config/sway"
    sudo cp -r "${SCRIPT_DIR}/../../config/sway/"* "${ROOT}/etc/skel/.config/sway/" 2>/dev/null || true
}

finalize_image() {
    log_info "Finalizing image..."
    
    # Sync and unmount
    sync
    sudo umount -R "${WORK_DIR}/mnt"
    sudo losetup -d "${LOOP_DEV}"
    
    # Compress
    log_info "Compressing image..."
    xz -T0 -9 -k "${WORK_DIR}/disk.img"
    mv "${WORK_DIR}/disk.img.xz" "${OUT_DIR}/${IMAGE_NAME}.img.xz"
    
    # Generate checksum
    cd "${OUT_DIR}"
    sha256sum "${IMAGE_NAME}.img.xz" > "${IMAGE_NAME}.img.xz.sha256"
    
    log_info "Image created: ${OUT_DIR}/${IMAGE_NAME}.img.xz"
}

main() {
    log_info "RururuOS ARM64 Raspberry Pi Build"
    log_info "Version: ${VERSION}"
    
    case "${1:-full}" in
        clean)
            clean
            ;;
        full)
            check_deps
            clean
            create_image
            bootstrap_system
            configure_system
            install_rururu_packages
            finalize_image
            ;;
        *)
            echo "Usage: $0 {clean|full}"
            ;;
    esac
}

main "$@"
