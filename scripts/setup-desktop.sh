#!/bin/bash
# RururuOS Desktop Environment Setup Script
# Configures Sway, GNOME, or KDE for creative workflow

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_DIR="$(dirname "$SCRIPT_DIR")/config"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

print_usage() {
    cat << EOF
RururuOS Desktop Setup

Usage: $0 <desktop-environment>

Desktop Environments:
    sway    - Sway (Wayland tiling compositor)
    gnome   - GNOME with extensions
    kde     - KDE Plasma

Options:
    -h, --help    Show this help message
    -b, --backup  Backup existing configs before replacing

Examples:
    $0 sway
    $0 gnome --backup
    $0 kde
EOF
}

backup_config() {
    local path="$1"
    if [[ -e "$path" ]]; then
        local backup="${path}.bak.$(date +%Y%m%d_%H%M%S)"
        log_info "Backing up $path to $backup"
        cp -r "$path" "$backup"
    fi
}

install_packages_arch() {
    local packages=("$@")
    log_info "Installing packages: ${packages[*]}"
    sudo pacman -S --needed --noconfirm "${packages[@]}"
}

install_aur_packages() {
    local packages=("$@")
    if command -v yay &> /dev/null; then
        yay -S --needed --noconfirm "${packages[@]}"
    elif command -v paru &> /dev/null; then
        paru -S --needed --noconfirm "${packages[@]}"
    else
        log_warn "No AUR helper found. Please install: ${packages[*]}"
    fi
}

setup_sway() {
    log_info "Setting up Sway..."

    # Install packages
    local packages=(
        sway swaylock swayidle swaybg
        waybar wofi mako
        foot
        grim slurp wl-clipboard
        wf-recorder
        brightnessctl playerctl
        pipewire pipewire-pulse wireplumber
        xdg-desktop-portal-wlr
        polkit-gnome
        udiskie
        cliphist
        imagemagick  # for color picker
    )
    install_packages_arch "${packages[@]}"

    # AUR packages
    local aur_packages=(
        wdisplays  # GUI for display configuration
    )
    install_aur_packages "${aur_packages[@]}"

    # Create config directories
    mkdir -p ~/.config/sway
    mkdir -p ~/.config/waybar
    mkdir -p ~/.config/mako
    mkdir -p ~/.config/foot

    # Backup and copy configs
    if [[ "$BACKUP" == "true" ]]; then
        backup_config ~/.config/sway/config
        backup_config ~/.config/waybar/config
        backup_config ~/.config/waybar/style.css
    fi

    cp "$CONFIG_DIR/sway/config" ~/.config/sway/config
    cp "$CONFIG_DIR/sway/waybar/config.jsonc" ~/.config/waybar/config
    cp "$CONFIG_DIR/sway/waybar/style.css" ~/.config/waybar/style.css

    # Create mako config
    cat > ~/.config/mako/config << 'EOF'
font=Inter 11
background-color=#1a1b26
text-color=#c0caf5
border-color=#7aa2f7
border-radius=4
border-size=2
padding=10
default-timeout=5000
max-visible=5
layer=overlay

[urgency=high]
background-color=#f7768e
text-color=#1a1b26
border-color=#f7768e
default-timeout=0
EOF

    # Create foot config
    cat > ~/.config/foot/foot.ini << 'EOF'
[main]
font=JetBrains Mono:size=11
dpi-aware=yes

[colors]
background=1a1b26
foreground=c0caf5

regular0=15161e
regular1=f7768e
regular2=9ece6a
regular3=e0af68
regular4=7aa2f7
regular5=bb9af7
regular6=7dcfff
regular7=a9b1d6

bright0=414868
bright1=f7768e
bright2=9ece6a
bright3=e0af68
bright4=7aa2f7
bright5=bb9af7
bright6=7dcfff
bright7=c0caf5
EOF

    # Create screenshots directory
    mkdir -p ~/Pictures/Screenshots
    mkdir -p ~/Videos

    log_info "Sway setup complete!"
    log_info "Start Sway with: sway"
}

setup_gnome() {
    log_info "Setting up GNOME..."

    # Install packages
    local packages=(
        gnome gnome-tweaks gnome-shell-extensions
        gnome-terminal nautilus
        pipewire pipewire-pulse wireplumber
        xdg-desktop-portal-gnome
    )
    install_packages_arch "${packages[@]}"

    # AUR packages for extensions
    local aur_packages=(
        gnome-shell-extension-appindicator
        gnome-shell-extension-dash-to-dock
        gnome-shell-extension-pop-shell
        adw-gtk3
    )
    install_aur_packages "${aur_packages[@]}"

    # Load dconf settings
    if [[ -f "$CONFIG_DIR/gnome/dconf-settings.ini" ]]; then
        log_info "Loading GNOME dconf settings..."
        dconf load / < "$CONFIG_DIR/gnome/dconf-settings.ini"
    fi

    # Enable GDM
    sudo systemctl enable gdm

    log_info "GNOME setup complete!"
    log_info "Reboot and select GNOME from the login screen"
}

setup_kde() {
    log_info "Setting up KDE Plasma..."

    # Install packages
    local packages=(
        plasma-meta kde-applications-meta
        sddm sddm-kcm
        konsole dolphin
        pipewire pipewire-pulse wireplumber
        xdg-desktop-portal-kde
        papirus-icon-theme
    )
    install_packages_arch "${packages[@]}"

    # Copy KDE configs
    mkdir -p ~/.config

    if [[ "$BACKUP" == "true" ]]; then
        backup_config ~/.config/kwinrc
        backup_config ~/.config/kglobalshortcutsrc
        backup_config ~/.config/kdeglobals
    fi

    cp "$CONFIG_DIR/kde/kwinrc" ~/.config/kwinrc
    cp "$CONFIG_DIR/kde/kglobalshortcutsrc" ~/.config/kglobalshortcutsrc
    cp "$CONFIG_DIR/kde/kdeglobals" ~/.config/kdeglobals

    # Enable SDDM
    sudo systemctl enable sddm

    log_info "KDE Plasma setup complete!"
    log_info "Reboot and select Plasma from the login screen"
}

install_creative_apps() {
    log_info "Installing creative applications..."

    local packages=(
        # Image editing
        gimp inkscape krita darktable rawtherapee
        # Video editing
        kdenlive shotcut
        # Audio
        ardour audacity
        # 3D
        blender freecad
        # Utilities
        imagemagick ffmpeg
    )

    install_packages_arch "${packages[@]}"

    # AUR creative apps
    local aur_packages=(
        davinci-resolve  # if needed
    )
    # install_aur_packages "${aur_packages[@]}"

    log_info "Creative applications installed!"
}

setup_fonts() {
    log_info "Installing fonts..."

    local packages=(
        ttf-jetbrains-mono
        inter-font
        ttf-liberation
        noto-fonts noto-fonts-cjk noto-fonts-emoji
        ttf-font-awesome
    )

    install_packages_arch "${packages[@]}"

    # AUR fonts
    local aur_packages=(
        ttf-symbols-nerd-font
    )
    install_aur_packages "${aur_packages[@]}"

    # Refresh font cache
    fc-cache -fv

    log_info "Fonts installed!"
}

# Parse arguments
BACKUP=false
DE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            print_usage
            exit 0
            ;;
        -b|--backup)
            BACKUP=true
            shift
            ;;
        sway|gnome|kde)
            DE="$1"
            shift
            ;;
        *)
            log_error "Unknown option: $1"
            print_usage
            exit 1
            ;;
    esac
done

if [[ -z "$DE" ]]; then
    log_error "Please specify a desktop environment"
    print_usage
    exit 1
fi

# Check if running on Arch
if ! command -v pacman &> /dev/null; then
    log_error "This script is designed for Arch Linux"
    exit 1
fi

# Setup fonts first
setup_fonts

# Setup selected DE
case "$DE" in
    sway)
        setup_sway
        ;;
    gnome)
        setup_gnome
        ;;
    kde)
        setup_kde
        ;;
esac

# Ask about creative apps
read -p "Install creative applications? [y/N] " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    install_creative_apps
fi

log_info "Desktop setup complete!"
log_info "Please reboot your system to apply all changes."
