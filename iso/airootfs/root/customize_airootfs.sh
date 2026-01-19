#!/usr/bin/env bash
# RururuOS airootfs customization script

set -e -u

# Set locale
sed -i 's/#en_US.UTF-8/en_US.UTF-8/' /etc/locale.gen
sed -i 's/#ru_RU.UTF-8/ru_RU.UTF-8/' /etc/locale.gen
locale-gen

# Set timezone
ln -sf /usr/share/zoneinfo/UTC /etc/localtime

# Enable services
systemctl enable NetworkManager.service
systemctl enable bluetooth.service
systemctl enable cups.service
systemctl enable pipewire.service
systemctl enable pipewire-pulse.service
systemctl enable wireplumber.service
systemctl enable rururu-file-handler.service

# Create audio group if not exists
getent group audio || groupadd audio
getent group video || groupadd video
getent group realtime || groupadd realtime

# Create cache directory for file handler
mkdir -p /var/cache/rururu
chmod 755 /var/cache/rururu

# Set permissions
chmod 700 /root
chmod 755 /usr/local/bin/*

echo "RururuOS customization complete"
