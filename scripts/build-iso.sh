#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
WORK_DIR="/tmp/rururu-archiso"
OUTPUT_DIR="$PROJECT_DIR/output"

echo "=== RururuOS ISO Builder ==="
echo "Project: $PROJECT_DIR"
echo "Work: $WORK_DIR"
echo "Output: $OUTPUT_DIR"

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root (for archiso)"
   exit 1
fi

# Check for archiso
if ! command -v mkarchiso &> /dev/null; then
    echo "archiso not found. Install with: pacman -S archiso"
    exit 1
fi

# Clean previous build
rm -rf "$WORK_DIR"
mkdir -p "$OUTPUT_DIR"

# Copy base profile
echo ">>> Preparing ISO profile..."
cp -r /usr/share/archiso/configs/releng "$WORK_DIR/profile"

# Add our packages
if [[ -f "$PROJECT_DIR/config/packages.x86_64" ]]; then
    cat "$PROJECT_DIR/config/packages.x86_64" >> "$WORK_DIR/profile/packages.x86_64"
fi

# Copy custom airootfs
if [[ -d "$PROJECT_DIR/iso/airootfs" ]]; then
    cp -r "$PROJECT_DIR/iso/airootfs"/* "$WORK_DIR/profile/airootfs/" 2>/dev/null || true
fi

# Build Rust components
echo ">>> Building Rust components..."
cd "$PROJECT_DIR"
cargo build --release

# Copy binaries to airootfs
mkdir -p "$WORK_DIR/profile/airootfs/usr/local/bin"
cp target/release/rururu-file-handler "$WORK_DIR/profile/airootfs/usr/local/bin/" 2>/dev/null || true

# Build ISO
echo ">>> Building ISO..."
mkarchiso -v -w "$WORK_DIR/work" -o "$OUTPUT_DIR" "$WORK_DIR/profile"

echo "=== Build complete ==="
ls -lh "$OUTPUT_DIR"/*.iso
