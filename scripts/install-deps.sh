#!/bin/bash
set -euo pipefail

echo "=== RururuOS Dependencies Installer ==="

# Detect package manager
if command -v pacman &> /dev/null; then
    PM="pacman"
elif command -v apt &> /dev/null; then
    PM="apt"
elif command -v dnf &> /dev/null; then
    PM="dnf"
else
    echo "Unsupported package manager"
    exit 1
fi

echo "Detected package manager: $PM"

case $PM in
    pacman)
        sudo pacman -Syu --noconfirm
        
        # Base development
        sudo pacman -S --noconfirm --needed \
            base-devel \
            rust \
            git \
            cmake \
            meson \
            ninja
        
        # Multimedia codecs
        sudo pacman -S --noconfirm --needed \
            ffmpeg \
            gstreamer \
            gst-plugins-base \
            gst-plugins-good \
            gst-plugins-bad \
            gst-plugins-ugly \
            gst-libav
        
        # Audio
        sudo pacman -S --noconfirm --needed \
            pipewire \
            pipewire-pulse \
            pipewire-jack \
            wireplumber \
            flac \
            opus \
            libvorbis \
            lame
        
        # Image
        sudo pacman -S --noconfirm --needed \
            libjpeg-turbo \
            libpng \
            libwebp \
            libavif \
            libheif \
            libtiff \
            openexr \
            libraw
        
        # 3D
        sudo pacman -S --noconfirm --needed \
            assimp \
            opencolorio
        
        # Documents
        sudo pacman -S --noconfirm --needed \
            poppler \
            pandoc
        
        # Archiso (for ISO building)
        sudo pacman -S --noconfirm --needed archiso
        ;;
        
    apt)
        sudo apt update
        sudo apt install -y \
            build-essential \
            rustc \
            cargo \
            git \
            cmake \
            meson \
            ninja-build \
            ffmpeg \
            libavcodec-dev \
            libavformat-dev \
            pipewire \
            libflac-dev \
            libopus-dev \
            libvorbis-dev \
            libjpeg-dev \
            libpng-dev \
            libwebp-dev \
            libtiff-dev \
            libopenexr-dev \
            libraw-dev \
            libassimp-dev
        ;;
        
    dnf)
        sudo dnf install -y \
            @development-tools \
            rust \
            cargo \
            git \
            cmake \
            meson \
            ninja-build \
            ffmpeg-free \
            ffmpeg-free-devel \
            pipewire \
            flac-devel \
            opus-devel \
            libvorbis-devel \
            libjpeg-turbo-devel \
            libpng-devel \
            libwebp-devel \
            libtiff-devel \
            openexr-devel \
            LibRaw-devel \
            assimp-devel
        ;;
esac

echo "=== Dependencies installed ==="
