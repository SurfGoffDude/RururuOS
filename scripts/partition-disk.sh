#!/bin/bash
# RururuOS Automatic Disk Partitioning Script
# Supports UEFI and BIOS systems

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# Check root
[[ $EUID -ne 0 ]] && error "This script must be run as root"

# Detect UEFI or BIOS
if [[ -d /sys/firmware/efi ]]; then
    BOOT_MODE="uefi"
    log "Detected UEFI boot mode"
else
    BOOT_MODE="bios"
    log "Detected BIOS/Legacy boot mode"
fi

# List available disks
echo ""
log "Available disks:"
lsblk -d -o NAME,SIZE,MODEL | grep -v "loop\|sr"
echo ""

# Select disk
read -p "Enter disk to partition (e.g., sda, nvme0n1): " DISK

# Validate disk
[[ ! -b "/dev/$DISK" ]] && error "Disk /dev/$DISK not found"

# Confirm
warn "WARNING: All data on /dev/$DISK will be DESTROYED!"
read -p "Type 'yes' to continue: " CONFIRM
[[ "$CONFIRM" != "yes" ]] && error "Aborted by user"

# Get disk size in GB
DISK_SIZE_GB=$(lsblk -b -d -o SIZE /dev/$DISK | tail -1 | awk '{print int($1/1024/1024/1024)}')
log "Disk size: ${DISK_SIZE_GB}GB"

# Calculate partition sizes
# EFI: 512MB (UEFI) or none (BIOS)
# Boot: 1GB (for BIOS GRUB)
# Swap: min(RAM, 32GB)
# Root: remaining

RAM_GB=$(free -g | awk '/^Mem:/{print $2}')
SWAP_GB=$((RAM_GB > 32 ? 32 : RAM_GB))

log "RAM detected: ${RAM_GB}GB, Swap will be: ${SWAP_GB}GB"

# Wipe disk
log "Wiping disk..."
wipefs -af /dev/$DISK
sgdisk -Z /dev/$DISK

if [[ "$BOOT_MODE" == "uefi" ]]; then
    log "Creating GPT partition table for UEFI..."
    
    # Create partitions
    sgdisk -n 1:0:+512M -t 1:ef00 -c 1:"EFI" /dev/$DISK      # EFI
    sgdisk -n 2:0:+${SWAP_GB}G -t 2:8200 -c 2:"Swap" /dev/$DISK  # Swap
    sgdisk -n 3:0:0 -t 3:8300 -c 3:"Root" /dev/$DISK          # Root
    
    # Determine partition naming
    if [[ "$DISK" == nvme* ]]; then
        PART_PREFIX="${DISK}p"
    else
        PART_PREFIX="$DISK"
    fi
    
    EFI_PART="/dev/${PART_PREFIX}1"
    SWAP_PART="/dev/${PART_PREFIX}2"
    ROOT_PART="/dev/${PART_PREFIX}3"
    
    # Format partitions
    log "Formatting EFI partition..."
    mkfs.fat -F32 "$EFI_PART"
    
    log "Formatting swap..."
    mkswap "$SWAP_PART"
    
    log "Formatting root partition (ext4)..."
    mkfs.ext4 -F "$ROOT_PART"
    
    # Mount
    log "Mounting partitions..."
    mount "$ROOT_PART" /mnt
    mkdir -p /mnt/boot/efi
    mount "$EFI_PART" /mnt/boot/efi
    swapon "$SWAP_PART"
    
else
    log "Creating MBR partition table for BIOS..."
    
    # Create MBR partitions
    parted -s /dev/$DISK mklabel msdos
    parted -s /dev/$DISK mkpart primary ext4 1MiB 1GiB        # Boot
    parted -s /dev/$DISK set 1 boot on
    parted -s /dev/$DISK mkpart primary linux-swap 1GiB $((1 + SWAP_GB))GiB  # Swap
    parted -s /dev/$DISK mkpart primary ext4 $((1 + SWAP_GB))GiB 100%        # Root
    
    if [[ "$DISK" == nvme* ]]; then
        PART_PREFIX="${DISK}p"
    else
        PART_PREFIX="$DISK"
    fi
    
    BOOT_PART="/dev/${PART_PREFIX}1"
    SWAP_PART="/dev/${PART_PREFIX}2"
    ROOT_PART="/dev/${PART_PREFIX}3"
    
    # Format
    log "Formatting boot partition..."
    mkfs.ext4 -F "$BOOT_PART"
    
    log "Formatting swap..."
    mkswap "$SWAP_PART"
    
    log "Formatting root partition..."
    mkfs.ext4 -F "$ROOT_PART"
    
    # Mount
    log "Mounting partitions..."
    mount "$ROOT_PART" /mnt
    mkdir -p /mnt/boot
    mount "$BOOT_PART" /mnt/boot
    swapon "$SWAP_PART"
fi

# Generate fstab
log "Generating fstab..."
mkdir -p /mnt/etc
genfstab -U /mnt >> /mnt/etc/fstab

log "Partitioning complete!"
echo ""
log "Partition layout:"
lsblk /dev/$DISK
echo ""
log "Mount points:"
findmnt -R /mnt

echo ""
log "Next steps:"
echo "  1. pacstrap /mnt base linux linux-firmware"
echo "  2. arch-chroot /mnt"
echo "  3. Configure bootloader"
