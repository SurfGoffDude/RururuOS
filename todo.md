# RururuOS — План разработки

## Статус проекта

**Текущая фаза**: 3 — Desktop Environment  
**Прогресс**: 70%

---

## Фаза 0: Подготовка и планирование

### Инфраструктура разработки

- [!] Настроить dev-окружение на Mac (Lima/UTM) — требует ручной настройки
- [x] Создать репозиторий с CI/CD (GitHub Actions)
- [x] Настроить кросс-компиляцию Rust для Linux
- [x] Подготовить базовую структуру проекта

### Исследование

- [x] Изучить archiso для сборки ISO
- [!] Протестировать Arch Linux ARM для Apple Silicon — требует VM
- [!] Оценить smithay vs готовые Wayland композиторы — требует исследования
- [x] Составить список всех необходимых пакетов

---

## Фаза 1: Базовая система

### 1.1 Минимальная установка

- [x] Создать базовый профиль archiso
- [x] Настроить автоматическую разметку дисков
- [x] Конфигурация systemd-boot / GRUB
- [x] Базовая локализация (en_US, ru_RU)

### 1.2 Ядро и драйверы

- [x] Конфигурация ядра с PREEMPT_RT патчами
- [x] Настройка NVIDIA драйверов (nvidia-dkms) — в packages.x86_64
- [x] Настройка AMD драйверов (mesa, amdgpu) — в packages.x86_64
- [x] Intel GPU поддержка — в packages.x86_64
- [x] Конфигурация sysctl для low-latency

### 1.3 Аудио подсистема

- [x] Установка и настройка PipeWire
- [x] PipeWire-JACK для pro audio совместимости
- [x] WirePlumber конфигурация
- [!] Тестирование latency (< 10ms target) — требует VM
- [x] Realtime scheduling для аудио процессов

### 1.4 Мета-пакет кодеков

- [x] Создать PKGBUILD для `rururu-codecs`
- [x] FFmpeg с полной поддержкой кодеков
- [x] GStreamer plugins (good, bad, ugly)
- [x] Аудио кодеки (flac, opus, vorbis, lame, fdkaac)
- [x] Видео кодеки (x264, x265, aom, svt-av1)
- [!] Тестирование всех кодеков — требует Linux VM

---

## Фаза 2: Rust инфраструктура

### 2.1 Universal File Handler

- [x] Архитектура daemon (D-Bus API)
- [x] Определение формата через magic bytes
- [x] Интеграция с FFmpeg (ffmpeg-next crate)
- [x] Интеграция с Assimp для 3D (russimp crate)
- [x] Интеграция с libraw для RAW фото (rawloader crate)
- [x] Plugin API (динамическая загрузка .so)
- [x] Кэширование метаданных (sled DB)
- [x] Генерация thumbnails

### 2.2 Codec Registry

- [x] Реестр доступных кодеков
- [x] Автоопределение capabilities
- [x] Fallback механизм
- [x] Логирование и диагностика (tracing)

### 2.3 Rust-обёртки

- [x] ffmpeg-next — интегрирован в file-handler
- [x] openexr-rs — HDR изображения (rururu-wrappers/exr)
- [x] assimp-rs/russimp — 3D форматы (rururu-wrappers/model3d)
- [!] Draco bindings — нет готового crate, требует FFI
- [x] OpenColorIO bindings — software fallback (rururu-wrappers/color)

### 2.4 Системные утилиты

- [!] uutils — оценено, можно интегрировать позже
- [x] Кастомные утилиты (rururu-utils/process, system)
- [x] Интеграция с systemd (rururu-utils/systemd)

---

## Фаза 3: Desktop Environment

### 3.1 Wayland Compositor

**Вариант A: Smithay (кастомный)**

- [ ] Базовый compositor на smithay
- [ ] Поддержка XWayland
- [ ] Декорации окон
- [ ] Мультимонитор
- [ ] HiDPI поддержка

**Вариант B: Существующий DE** ✓

- [x] Настройка под creative workflow
- [x] Кастомные расширения (GNOME Shell, KDE shortcuts)
- [x] Оптимизация производительности (VRR, tiling, shortcuts)

### 3.2 File Manager

- [x] Архитектура (iced)
- [x] Интеграция с Universal File Handler
- [x] Превью всех форматов:
  - [x] Изображения (включая RAW, HDR)
  - [!] Видео (thumbnails + hover preview) — требует доработки
  - [!] Аудио (waveform) — требует доработки
  - [!] 3D модели (рендер превью) — требует доработки
  - [!] Документы (первая страница) — требует доработки
- [x] Быстрый поиск
- [x] Теги и метаданные
- [x] Batch операции

### 3.3 Системные приложения

- [x] Settings app (Rust + iced) — rururu-settings
- [x] System monitor — rururu-monitor
- [!] Audio mixer (PipeWire control) — базовый контроль в settings
- [x] Color calibration tool — rururu-colorcal

### 3.4 Кастомизация DE

**Sway:**

- [x] Темы (Tokyo Night, Catppuccin)
- [x] Scripts (screenshot, creative-workspace)
- [x] Waybar конфигурация

**GNOME:**

- [x] dconf настройки
- [x] GSchema для RururuOS
- [x] Список расширений

**KDE:**

- [x] KWin конфигурация
- [x] Shortcuts для creative apps
- [x] Creative Launcher plasmoid

---

## Фаза 4: Интеграция и polish

### 4.1 Color Management

- [x] OpenColorIO интеграция — rururu-color (ocio.rs)
- [x] ICC профили для мониторов — rururu-color (icc.rs)
- [x] HDR display поддержка — rururu-color (hdr.rs)
- [x] D-Bus сервис для color management
- [x] Калибровка workflow — интеграция с rururu-colorcal

### 4.2 Профили workflow

- [x] **Video Editor** — DaVinci Resolve, Kdenlive, настройки
- [x] **3D Artist** — Blender, оптимизация GPU
- [x] **2D Designer** — Krita, GIMP, Inkscape
- [x] **Audio Producer** — Ardour, Bitwig, realtime audio
- [x] **Photographer** — Darktable, RawTherapee
- [x] CLI утилита rururu-workflow
- [x] Автопереключение профилей

### 4.3 Installer

- [x] Calamares конфигурация
- [x] Branding RururuOS
- [x] Модуль выбора workflow
- [x] Автоопределение hardware — rururu-hardware-detect
- [x] Post-install setup wizard — rururu-setup-wizard (iced GUI)

### 4.4 ISO образы

- [x] Build script для x86_64
- [x] Profile definition
- [x] Makefile для сборки
- [x] ARM64 образ (Raspberry Pi) — iso/arm64-rpi/build.sh
- [x] ARM64 образ (Apple Silicon / Asahi) — iso/arm64-asahi/build.sh
- [x] CI/CD pipeline — GitHub Actions (ci.yml, release.yml, iso-nightly.yml)

---

## Фаза 5: Релиз и поддержка

### 5.1 Документация

- [x] Installation guide — docs/installation-guide.md
- [x] User manual — docs/user-manual.md
- [x] Developer documentation — docs/developer-guide.md
- [x] Troubleshooting guide — docs/troubleshooting.md
- [x] FAQ — docs/faq.md

### 5.2 Тестирование

- [x] Hardware compatibility testing — tests/hardware/compatibility.md
- [ ] Performance benchmarks
- [x] Codec compatibility matrix — tests/codecs/compatibility.md
- [ ] User acceptance testing

### 5.3 Релиз

- [x] Release process — RELEASE.md
- [x] Changelog — CHANGELOG.md
- [ ] Alpha release (internal testing)
- [ ] Beta release (community testing)
- [ ] RC release
- [ ] Stable 1.0 release

### 5.4 Инфраструктура поддержки

- [x] Package repository — infrastructure/repo/README.md
- [x] Update mechanism — infrastructure/updates/update-checker.sh
- [x] Bug tracker — GitHub Issues (см. infrastructure/community/README.md)
- [x] Community forum / Discord — infrastructure/community/README.md

---

## Приоритеты

### P0 — Критично (MVP)

1. Базовая система с кодеками
2. PipeWire аудио
3. Работающий ISO

### P1 — Важно

1. Universal File Handler
2. File manager с превью
3. Профили workflow

### P2 — Желательно

1. Кастомный Wayland compositor
2. Color management
3. ARM64 поддержка

### P3 — Nice to have

1. Собственный installer
2. Auto-update система
3. Cloud sync настроек

---

## Временные оценки

| Фаза | Оценка | Зависимости |
|------|--------|-------------|
| Фаза 0 | 1-2 недели | — |
| Фаза 1 | 2-4 недели | Фаза 0 |
| Фаза 2 | 4-8 недель | Фаза 1 |
| Фаза 3 | 6-12 недель | Фаза 2 |
| Фаза 4 | 4-6 недель | Фаза 3 |
| Фаза 5 | 2-4 недели | Фаза 4 |

**Общая оценка до MVP**: 3-4 месяца  
**Общая оценка до 1.0**: 6-9 месяцев

---

## Заметки

*Место для заметок по ходу разработки*
