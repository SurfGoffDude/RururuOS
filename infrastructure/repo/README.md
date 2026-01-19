# RururuOS Package Repository

## Repository Structure

```
repo/
├── x86_64/
│   ├── rururu.db
│   ├── rururu.db.tar.gz
│   └── packages/
├── aarch64/
│   ├── rururu.db
│   ├── rururu.db.tar.gz
│   └── packages/
└── keys/
    └── rururu.gpg
```

## Setting Up the Repository

### Server Requirements

- Web server (nginx/Apache)
- HTTPS certificate
- Sufficient storage (50GB+)

### Nginx Configuration

```nginx
server {
    listen 443 ssl http2;
    server_name repo.rururu.os;
    
    root /var/www/repo;
    
    location / {
        autoindex on;
        autoindex_exact_size off;
        autoindex_localtime on;
    }
    
    location ~ \.db$ {
        add_header Cache-Control "no-cache";
    }
}
```

## Building Packages

### PKGBUILD Structure

```bash
# packages/rururu-files/PKGBUILD
pkgname=rururu-files
pkgver=0.1.0
pkgrel=1
pkgdesc="RururuOS File Manager"
arch=('x86_64' 'aarch64')
url="https://rururu.os"
license=('MIT')
depends=('gtk4' 'libadwaita')
makedepends=('rust' 'cargo')

build() {
    cd "$srcdir"
    cargo build --release -p rururu-files
}

package() {
    install -Dm755 target/release/rururu-files "$pkgdir/usr/bin/rururu-files"
}
```

### Building

```bash
# Build package
makepkg -s

# Sign package
gpg --detach-sign --use-agent rururu-files-0.1.0-1-x86_64.pkg.tar.zst

# Add to repository
repo-add rururu.db.tar.gz rururu-files-0.1.0-1-x86_64.pkg.tar.zst
```

## Client Configuration

### Add Repository

Add to `/etc/pacman.conf`:

```ini
[rururu]
Server = https://repo.rururu.os/$arch
SigLevel = Required DatabaseRequired
```

### Import Key

```bash
sudo pacman-key --recv-keys RURURU_KEY_ID
sudo pacman-key --lsign-key RURURU_KEY_ID
```

### Update and Install

```bash
sudo pacman -Sy rururu-files rururu-settings
```

## Automation

### GitHub Actions Upload

```yaml
- name: Upload to repository
  env:
    REPO_SSH_KEY: ${{ secrets.REPO_SSH_KEY }}
  run: |
    rsync -avz --delete \
      packages/*.pkg.tar.zst \
      repo@repo.rururu.os:/var/www/repo/x86_64/packages/
    
    ssh repo@repo.rururu.os "cd /var/www/repo/x86_64 && repo-add rururu.db.tar.gz packages/*.pkg.tar.zst"
```

## Mirror Setup

To set up a mirror:

1. Sync from main repository:
   ```bash
   rsync -avz repo.rururu.os::rururu /var/www/repo/
   ```

2. Configure cron for regular sync

3. Register mirror at https://rururu.os/mirrors
