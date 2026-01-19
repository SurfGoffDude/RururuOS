#!/bin/bash
# RururuOS ARM64 Apple Silicon (Asahi Linux) Build Script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work"
OUT_DIR="${SCRIPT_DIR}/out"

VERSION="1.0.0"
IMAGE_NAME="rururu-${VERSION}-arm64-asahi"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

check_deps() {
    log_info "Checking dependencies..."
    
    # Asahi builds typically use the Asahi installer
    # This script prepares a customization overlay
    
    if ! command -v arch-chroot &>/dev/null; then
        log_error "arch-install-scripts required"
        exit 1
    fi
}

clean() {
    log_info "Cleaning..."
    rm -rf "${WORK_DIR}"
    mkdir -p "${WORK_DIR}" "${OUT_DIR}"
}

create_overlay() {
    log_info "Creating RururuOS overlay for Asahi..."
    
    local OVERLAY="${WORK_DIR}/overlay"
    mkdir -p "${OVERLAY}"
    
    # Directory structure
    mkdir -p "${OVERLAY}/etc/skel/.config/sway"
    mkdir -p "${OVERLAY}/etc/rururu"
    mkdir -p "${OVERLAY}/usr/local/bin"
    mkdir -p "${OVERLAY}/usr/share/rururu"
    mkdir -p "${OVERLAY}/etc/systemd/system"
    
    # Sway config for Apple Silicon
    cat << 'EOF' > "${OVERLAY}/etc/skel/.config/sway/config.d/asahi.conf"
# Apple Silicon specific configuration

# Trackpad settings
input type:touchpad {
    tap enabled
    natural_scroll enabled
    dwt enabled
    accel_profile adaptive
    pointer_accel 0.3
}

# HiDPI for Retina displays
output * scale 2

# Function keys
bindsym XF86MonBrightnessUp exec brightnessctl set +5%
bindsym XF86MonBrightnessDown exec brightnessctl set 5%-
bindsym XF86AudioRaiseVolume exec wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+
bindsym XF86AudioLowerVolume exec wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-
bindsym XF86AudioMute exec wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle
EOF

    # Asahi-specific packages
    cat << 'EOF' > "${OVERLAY}/etc/rururu/asahi-packages.txt"
# Asahi Linux specific
asahi-meta
asahi-scripts
mesa-asahi-edge
linux-asahi

# Desktop
sway
waybar
foot
wofi
mako

# Audio (with Asahi speakers support)
pipewire
pipewire-pulse
wireplumber
bankstown-lv2

# Creative
krita
gimp
inkscape
darktable
audacity
kdenlive

# Development
git
rustup
base-devel
EOF

    # Post-install script
    cat << 'EOF' > "${OVERLAY}/usr/local/bin/rururu-asahi-setup"
#!/bin/bash
# RururuOS Asahi post-install setup

set -e

echo "Setting up RururuOS on Apple Silicon..."

# Enable speaker support
systemctl --user enable --now pipewire pipewire-pulse wireplumber

# Setup rust
if command -v rustup &>/dev/null; then
    rustup default stable
fi

# Enable Sway on login
if [ ! -f ~/.bash_profile ]; then
    cat << 'PROFILE' > ~/.bash_profile
if [ -z "$DISPLAY" ] && [ "$XDG_VTNR" = 1 ]; then
    exec sway
fi
PROFILE
fi

echo "Setup complete! Please reboot."
EOF
    chmod +x "${OVERLAY}/usr/local/bin/rururu-asahi-setup"

    # Systemd service for first boot
    cat << 'EOF' > "${OVERLAY}/etc/systemd/system/rururu-firstboot.service"
[Unit]
Description=RururuOS First Boot Setup
After=network-online.target
Wants=network-online.target
ConditionPathExists=!/var/lib/rururu/firstboot-done

[Service]
Type=oneshot
ExecStart=/usr/local/bin/rururu-firstboot
ExecStartPost=/usr/bin/touch /var/lib/rururu/firstboot-done
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
EOF

    cat << 'EOF' > "${OVERLAY}/usr/local/bin/rururu-firstboot"
#!/bin/bash
# First boot initialization

mkdir -p /var/lib/rururu

# Generate locales
locale-gen

# Update package database
pacman -Sy --noconfirm

# Install recommended packages
pacman -S --needed --noconfirm $(cat /etc/rururu/asahi-packages.txt | grep -v '^#' | tr '\n' ' ')

echo "First boot setup complete"
EOF
    chmod +x "${OVERLAY}/usr/local/bin/rururu-firstboot"
    
    # Copy RururuOS configs
    cp -r "${SCRIPT_DIR}/../../config/sway/"* "${OVERLAY}/etc/skel/.config/sway/" 2>/dev/null || true
}

build_packages() {
    log_info "Cross-compiling RururuOS packages for ARM64..."
    
    cd "${SCRIPT_DIR}/../.."
    
    # Requires cross-compilation toolchain
    if command -v cross &>/dev/null; then
        cross build --release --target aarch64-unknown-linux-gnu --workspace
        
        local BIN_DIR="target/aarch64-unknown-linux-gnu/release"
        local OVERLAY="${WORK_DIR}/overlay/usr/local/bin"
        
        for bin in rururu-files rururu-settings rururu-monitor rururu-colorcal rururu-workflow; do
            if [ -f "${BIN_DIR}/${bin}" ]; then
                cp "${BIN_DIR}/${bin}" "${OVERLAY}/"
                log_info "Copied ${bin}"
            fi
        done
    else
        log_warn "cross not installed. Install with: cargo install cross"
        log_warn "Skipping package cross-compilation"
    fi
}

package_overlay() {
    log_info "Packaging overlay..."
    
    cd "${WORK_DIR}"
    tar -czvf "${OUT_DIR}/${IMAGE_NAME}-overlay.tar.gz" -C overlay .
    
    # Create installation instructions
    cat << 'EOF' > "${OUT_DIR}/${IMAGE_NAME}-README.md"
# RururuOS for Apple Silicon (Asahi Linux)

## Installation

1. First, install Asahi Linux following the official instructions:
   ```
   curl https://alx.sh | sh
   ```

2. Boot into Asahi Linux and extract the RururuOS overlay:
   ```
   sudo tar -xzf rururu-*-asahi-overlay.tar.gz -C /
   ```

3. Run the setup script:
   ```
   sudo systemctl enable rururu-firstboot
   sudo reboot
   ```

4. After reboot, run the user setup:
   ```
   rururu-asahi-setup
   ```

## Notes

- This overlay is designed for Asahi Linux (Arch-based)
- Tested on MacBook Air M1/M2 and MacBook Pro M1/M2/M3
- Speaker support requires asahi-audio package
- GPU acceleration uses Mesa ASAHI drivers

## Troubleshooting

If you encounter issues:
- Check Asahi Linux wiki: https://asahilinux.org/
- Join our community: https://rururu.os/community
EOF

    log_info "Overlay created: ${OUT_DIR}/${IMAGE_NAME}-overlay.tar.gz"
}

main() {
    log_info "RururuOS ARM64 Asahi Build"
    log_info "Version: ${VERSION}"
    
    case "${1:-full}" in
        clean)
            clean
            ;;
        overlay)
            clean
            create_overlay
            package_overlay
            ;;
        full)
            check_deps
            clean
            create_overlay
            build_packages
            package_overlay
            ;;
        *)
            echo "Usage: $0 {clean|overlay|full}"
            ;;
    esac
}

main "$@"
