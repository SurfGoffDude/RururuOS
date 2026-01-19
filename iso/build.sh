#!/bin/bash
# RururuOS ISO Build Script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work"
OUT_DIR="${SCRIPT_DIR}/out"
PROFILE_DIR="${SCRIPT_DIR}/profile"

VERSION="1.0.0"
ISO_NAME="rururu-${VERSION}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_deps() {
    log_info "Checking dependencies..."
    
    local deps=("archiso" "squashfs-tools" "dosfstools" "mtools")
    local missing=()
    
    for dep in "${deps[@]}"; do
        if ! pacman -Qi "$dep" &>/dev/null; then
            missing+=("$dep")
        fi
    done
    
    if [ ${#missing[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing[*]}"
        log_info "Install with: sudo pacman -S ${missing[*]}"
        exit 1
    fi
    
    log_info "All dependencies satisfied"
}

clean() {
    log_info "Cleaning previous build..."
    sudo rm -rf "${WORK_DIR}"
    mkdir -p "${WORK_DIR}"
    mkdir -p "${OUT_DIR}"
}

prepare_profile() {
    log_info "Preparing ISO profile..."
    
    # Copy base archiso profile
    cp -r /usr/share/archiso/configs/releng/* "${PROFILE_DIR}/"
    
    # Apply RururuOS customizations
    
    # Packages
    cat >> "${PROFILE_DIR}/packages.x86_64" << 'EOF'
# RururuOS Base
base
base-devel
linux
linux-firmware
linux-headers

# Boot
grub
efibootmgr
os-prober

# Network
networkmanager
iwd

# Audio (PipeWire)
pipewire
pipewire-alsa
pipewire-pulse
pipewire-jack
wireplumber

# Graphics
mesa
vulkan-icd-loader
xf86-video-amdgpu
xf86-video-intel

# Wayland
wayland
xorg-xwayland

# Desktop Environment (Sway default)
sway
swaylock
swayidle
waybar
wofi
mako
foot

# File Manager
nautilus
tumbler
gvfs

# Essential Creative Apps
ffmpeg
gstreamer
gst-plugins-base
gst-plugins-good
gst-plugins-bad
gst-plugins-ugly
imagemagick
raw-thumbnailer

# Fonts
ttf-dejavu
ttf-liberation
noto-fonts
noto-fonts-emoji
noto-fonts-cjk

# Development
git
rustup
gcc
make
cmake
pkg-config

# Installer
calamares

# RururuOS packages (built locally)
# rururu-file-handler
# rururu-files
# rururu-settings
# rururu-monitor
# rururu-colorcal
# rururu-color
# rururu-workflows
EOF
}

build_packages() {
    log_info "Building RururuOS packages..."
    
    cd "${SCRIPT_DIR}/.."
    
    # Build all packages
    cargo build --release --workspace
    
    # Create package directory
    mkdir -p "${PROFILE_DIR}/airootfs/usr/bin"
    mkdir -p "${PROFILE_DIR}/airootfs/usr/lib"
    
    # Copy binaries
    local binaries=(
        "rururu-files"
        "rururu-settings"
        "rururu-monitor"
        "rururu-colorcal"
        "rururu-color-daemon"
        "rururu-workflow"
    )
    
    for bin in "${binaries[@]}"; do
        if [ -f "target/release/${bin}" ]; then
            cp "target/release/${bin}" "${PROFILE_DIR}/airootfs/usr/bin/"
            log_info "Copied ${bin}"
        fi
    done
}

configure_system() {
    log_info "Configuring system..."
    
    local airootfs="${PROFILE_DIR}/airootfs"
    
    # Create directories
    mkdir -p "${airootfs}/etc/skel/.config"
    mkdir -p "${airootfs}/etc/rururu"
    mkdir -p "${airootfs}/usr/share/rururu"
    
    # Copy Sway config
    mkdir -p "${airootfs}/etc/skel/.config/sway"
    cp -r "${SCRIPT_DIR}/../config/sway/"* "${airootfs}/etc/skel/.config/sway/"
    
    # Copy Waybar config
    mkdir -p "${airootfs}/etc/skel/.config/waybar"
    cp -r "${SCRIPT_DIR}/../config/sway/waybar/"* "${airootfs}/etc/skel/.config/waybar/" 2>/dev/null || true
    
    # Enable services
    mkdir -p "${airootfs}/etc/systemd/system/multi-user.target.wants"
    
    # NetworkManager
    ln -sf /usr/lib/systemd/system/NetworkManager.service \
        "${airootfs}/etc/systemd/system/multi-user.target.wants/"
    
    # PipeWire user service will be enabled by user
    
    # Set default shell to zsh
    echo "SHELL=/bin/zsh" >> "${airootfs}/etc/default/useradd"
    
    # Configure pacman
    cat >> "${airootfs}/etc/pacman.conf" << 'EOF'

[rururu]
SigLevel = Optional TrustAll
Server = https://repo.rururu.os/$arch
EOF
    
    # Set hostname
    echo "rururu-live" > "${airootfs}/etc/hostname"
    
    # Locale
    echo "en_US.UTF-8 UTF-8" > "${airootfs}/etc/locale.gen"
    echo "LANG=en_US.UTF-8" > "${airootfs}/etc/locale.conf"
}

build_iso() {
    log_info "Building ISO..."
    
    cd "${PROFILE_DIR}"
    
    sudo mkarchiso -v -w "${WORK_DIR}" -o "${OUT_DIR}" "${PROFILE_DIR}"
    
    local iso_file=$(ls -t "${OUT_DIR}"/*.iso 2>/dev/null | head -1)
    
    if [ -n "$iso_file" ]; then
        # Rename to our naming convention
        mv "$iso_file" "${OUT_DIR}/${ISO_NAME}-x86_64.iso"
        
        # Generate checksums
        cd "${OUT_DIR}"
        sha256sum "${ISO_NAME}-x86_64.iso" > "${ISO_NAME}-x86_64.iso.sha256"
        
        log_info "ISO built successfully: ${OUT_DIR}/${ISO_NAME}-x86_64.iso"
    else
        log_error "ISO build failed"
        exit 1
    fi
}

main() {
    log_info "RururuOS ISO Build Script"
    log_info "Version: ${VERSION}"
    echo
    
    case "${1:-}" in
        clean)
            clean
            ;;
        packages)
            build_packages
            ;;
        full)
            check_deps
            clean
            prepare_profile
            build_packages
            configure_system
            build_iso
            ;;
        *)
            echo "Usage: $0 {clean|packages|full}"
            echo
            echo "Commands:"
            echo "  clean     - Clean previous build"
            echo "  packages  - Build RururuOS packages only"
            echo "  full      - Full ISO build"
            exit 1
            ;;
    esac
}

main "$@"
