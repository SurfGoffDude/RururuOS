# Разработка RururuOS на macOS

Инструкция по настройке среды разработки и тестирования Linux-дистрибутива на Mac.

---

## Требования

- **macOS**: 12.0+ (Monterey или новее)
- **RAM**: минимум 16 GB (рекомендуется 32 GB)
- **Диск**: 100+ GB свободного места
- **CPU**: Apple Silicon (M1/M2/M3) или Intel

---

## 1. Установка базовых инструментов

### Homebrew

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### Основные пакеты

```bash
brew install \
    rust \
    qemu \
    lima \
    docker \
    podman \
    wget \
    curl \
    git \
    coreutils \
    gnu-sed \
    gnu-tar \
    xz \
    zstd
```

---

## 2. Виртуализация

### Вариант A: UTM (рекомендуется для Apple Silicon)

UTM — нативный гипервизор для macOS с поддержкой Apple Virtualization Framework.

```bash
brew install --cask utm
```

**Настройка VM для Arch Linux:**

1. Скачать ISO: <https://archlinux.org/download/>
2. Создать новую VM в UTM:
   - **Type**: Virtualize (для ARM) или Emulate (для x86)
   - **RAM**: 8+ GB
   - **CPU**: 4+ cores
   - **Disk**: 64+ GB
3. Установить Arch Linux по [официальному гайду](https://wiki.archlinux.org/title/Installation_guide)

### Вариант B: Lima (CLI-ориентированный)

Lima запускает Linux VM с автоматическим монтированием директорий.

```bash
brew install lima

# Создать Arch Linux VM
limactl create --name=rururu --vm-type=vz --mount-type=virtiofs template://archlinux

# Запустить
limactl start rururu

# Войти в VM
limactl shell rururu
```

**Конфигурация Lima** (`~/.lima/rururu/lima.yaml`):

```yaml
cpus: 4
memory: 8GiB
disk: 64GiB

mounts:
  - location: "~/Projects/Rust/RururuOS"
    writable: true

provision:
  - mode: system
    script: |
      #!/bin/bash
      pacman -Syu --noconfirm
      pacman -S --noconfirm base-devel rust git
```

### Вариант C: Docker/Podman (для сборки пакетов)

```bash
# Запуск Arch Linux контейнера
docker run -it --rm \
    -v ~/Projects/Rust/RururuOS:/workspace \
    archlinux:latest \
    /bin/bash
```

---

## 3. Кросс-компиляция Rust для Linux

### Установка target

```bash
# Для x86_64 Linux
rustup target add x86_64-unknown-linux-gnu

# Для ARM64 Linux (если целевая платформа ARM)
rustup target add aarch64-unknown-linux-gnu
```

### Cross-компилятор

```bash
# Установить cross (использует Docker)
cargo install cross

# Собрать проект для Linux
cross build --target x86_64-unknown-linux-gnu --release
```

### Альтернатива: zig как линкер

```bash
brew install zig

# В .cargo/config.toml
```

**`.cargo/config.toml`:**

```toml
[target.x86_64-unknown-linux-gnu]
linker = "zig"
rustflags = ["-C", "link-arg=-target", "-C", "link-arg=x86_64-linux-gnu"]
```

---

## 4. Структура проекта

```
RururuOS/
├── idea.md                 # Концепция и спецификация
├── mac-develop.md          # Эта инструкция
├── packages/               # Собственные пакеты
│   ├── rururu-codecs/      # Meta-пакет с кодеками
│   ├── rururu-file-handler/# Rust daemon
│   └── rururu-desktop/     # DE компоненты
├── scripts/
│   ├── build-iso.sh        # Сборка ISO
│   ├── install-deps.sh     # Установка зависимостей
│   └── test-vm.sh          # Запуск тестовой VM
├── config/
│   ├── pacman.conf         # Конфигурация pacman
│   ├── mkinitcpio.conf     # Initramfs
│   └── sysctl.conf         # Kernel параметры
└── iso/
    └── airootfs/           # Содержимое live-образа
```

---

## 5. Сборка ISO-образа

### Archiso (внутри VM или контейнера)

```bash
# В Arch Linux VM
sudo pacman -S archiso

# Создать рабочую директорию
cp -r /usr/share/archiso/configs/releng ~/rururu-iso
cd ~/rururu-iso

# Добавить пакеты в packages.x86_64
cat >> packages.x86_64 << EOF
# Codecs
ffmpeg
gstreamer
pipewire
pipewire-jack

# 3D
blender
openexr

# Development
rust
base-devel
EOF

# Собрать ISO
sudo mkarchiso -v -w /tmp/archiso-tmp -o ~/iso-output .
```

### Автоматизация на Mac

**`scripts/build-iso.sh`:**

```bash
#!/bin/bash
set -e

VM_NAME="rururu"
PROJECT_DIR="$HOME/Projects/Rust/RururuOS"

# Синхронизировать конфиги в VM
limactl shell $VM_NAME -- rsync -av /mnt/workspace/config/ ~/rururu-iso/

# Собрать ISO
limactl shell $VM_NAME -- sudo mkarchiso -v -w /tmp/archiso -o ~/iso-output ~/rururu-iso/

# Скопировать ISO на хост
limactl copy $VM_NAME:~/iso-output/*.iso $PROJECT_DIR/iso/
```

---

## 6. Тестирование

### Запуск ISO в QEMU

```bash
# x86_64
qemu-system-x86_64 \
    -m 4G \
    -smp 4 \
    -enable-kvm \
    -cdrom iso/rururu-latest.iso \
    -boot d

# На Apple Silicon (эмуляция x86)
qemu-system-x86_64 \
    -m 4G \
    -smp 4 \
    -cdrom iso/rururu-latest.iso \
    -boot d \
    -accel tcg
```

### Тестирование в UTM

1. Создать новую VM
2. Подключить ISO как CD/DVD
3. Boot from CD

### Автоматизированные тесты

**`scripts/test-vm.sh`:**

```bash
#!/bin/bash
# Автоматический тест загрузки

qemu-system-x86_64 \
    -m 4G \
    -smp 4 \
    -cdrom iso/rururu-latest.iso \
    -boot d \
    -nographic \
    -serial mon:stdio \
    -append "console=ttyS0" \
    | tee boot.log

# Проверить успешность загрузки
grep -q "RururuOS" boot.log && echo "Boot OK" || echo "Boot FAILED"
```

---

## 7. Разработка Rust-компонентов

### Локальная разработка (на Mac)

```bash
cd packages/rururu-file-handler

# Разработка и тесты (macOS)
cargo build
cargo test

# Кросс-компиляция для Linux
cross build --target x86_64-unknown-linux-gnu --release
```

### Тестирование в VM

```bash
# Скопировать бинарник в VM
limactl copy target/x86_64-unknown-linux-gnu/release/rururu-file-handler rururu:/tmp/

# Запустить в VM
limactl shell rururu -- /tmp/rururu-file-handler --test
```

---

## 8. Отладка

### GDB через QEMU

```bash
# Запустить QEMU с GDB сервером
qemu-system-x86_64 \
    -m 4G \
    -cdrom iso/rururu-latest.iso \
    -s -S  # -s = gdbserver на :1234, -S = пауза на старте

# В другом терминале
gdb -ex "target remote :1234"
```

### Serial console

```bash
# В QEMU добавить
-serial stdio -append "console=ttyS0"

# Логи будут выводиться в терминал
```

---

## 9. CI/CD (опционально)

### GitHub Actions

**`.github/workflows/build.yml`:**

```yaml
name: Build ISO

on:
  push:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    container: archlinux:latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install dependencies
        run: |
          pacman -Syu --noconfirm
          pacman -S --noconfirm archiso base-devel rust
      
      - name: Build ISO
        run: |
          mkarchiso -v -w /tmp/archiso -o ./output .
      
      - uses: actions/upload-artifact@v4
        with:
          name: rururu-iso
          path: output/*.iso
```

---

## 10. Полезные команды

| Задача | Команда |
|--------|---------|
| Войти в Lima VM | `limactl shell rururu` |
| Остановить VM | `limactl stop rururu` |
| Удалить VM | `limactl delete rururu` |
| Обновить Arch в VM | `sudo pacman -Syu` |
| Проверить ISO размер | `ls -lh iso/*.iso` |
| Мониторинг сборки | `tail -f /tmp/archiso/build.log` |

---

## Troubleshooting

### Lima не стартует

```bash
limactl stop rururu --force
limactl delete rururu
limactl create --name=rururu template://archlinux
```

### Ошибки кросс-компиляции

```bash
# Убедиться что Docker/Podman запущен
docker ps

# Переустановить cross
cargo install cross --force
```

### QEMU медленный на Apple Silicon

- Использовать UTM с Apple Virtualization Framework
- Или Lima с `--vm-type=vz`
- x86 эмуляция будет медленной — для разработки лучше ARM64

---

## Ссылки

- [Arch Wiki: Archiso](https://wiki.archlinux.org/title/Archiso)
- [Lima](https://github.com/lima-vm/lima)
- [UTM](https://mac.getutm.app/)
- [Cross](https://github.com/cross-rs/cross)
- [Rust cross-compilation](https://rust-lang.github.io/rustup/cross-compilation.html)
