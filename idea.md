# RururuOS — Creative Linux Distribution

## Концепция

ОС на базе **Arch Linux** для дизайнеров, 3D-моделеров и творческих профессионалов.  
Ключевая задача: **универсальная поддержка всех форматов файлов** — медиа, документы, 3D, аудио.

---

## Почему Arch Linux

- **Rolling release** — всегда актуальные версии Blender, Krita, DaVinci Resolve
- **Полный FFmpeg** — все кодеки без ограничений
- **AUR** — любой редкий формат уже упакован
- **PipeWire** — лучшая интеграция для pro audio
- **Свежие драйверы GPU** — критично для CUDA/OptiX рендеринга
- **Arch Wiki** — лучшая документация в Linux-мире

---

## Поддерживаемые архитектуры

### x86_64 (AMD64)

**Основная платформа.**

- Полная поддержка всех пакетов Arch Linux
- NVIDIA CUDA / OptiX для GPU-рендеринга
- Максимальная совместимость с профессиональным ПО (DaVinci Resolve, Maya, Houdini)
- Поддержка legacy hardware

**Целевое железо:**

- Рабочие станции (AMD Ryzen / Intel Core)
- Серверы рендер-фермы
- Ноутбуки с дискретной GPU

### ARM64 (AArch64)

**Растущая платформа.**

- Arch Linux ARM (alarm) как база
- Apple Silicon (M1/M2/M3) через Asahi Linux
- Raspberry Pi 4/5 для легких задач
- Qualcomm Snapdragon X Elite

**Особенности ARM-сборки:**

- Часть проприетарного ПО недоступна (DaVinci Resolve, некоторые плагины)
- Blender, Krita, GIMP — полная поддержка
- FFmpeg — полная поддержка всех кодеков
- Энергоэффективность для мобильных рабочих станций

### Матрица совместимости

| Компонент | x86_64 | ARM64 |
|-----------|--------|-------|
| Ядро Linux | ✅ | ✅ |
| Rust toolchain | ✅ | ✅ |
| FFmpeg (все кодеки) | ✅ | ✅ |
| Blender | ✅ | ✅ |
| DaVinci Resolve | ✅ | ❌ |
| NVIDIA CUDA | ✅ | ❌ |
| AMD ROCm | ✅ | ⚠️ (ограничено) |
| PipeWire | ✅ | ✅ |
| Wayland | ✅ | ✅ |

### Сборка для разных архитектур

```bash
# x86_64 ISO
mkarchiso -v -A x86_64 ...

# ARM64 ISO (требует Arch Linux ARM)
# Используется отдельный процесс сборки через alarm
```

---

## Поддерживаемые кодеки и форматы

### 1. Видео кодеки

#### Международные и массовые (MPEG/ITU-T)

- MPEG-1 Video
- MPEG-2 Video (H.262)
- MPEG-4 Part 2 (ASP, SP)
- MPEG-4 Part 10 (H.264 / AVC)
- MPEG-H Part 2 (H.265 / HEVC)
- MPEG-I Part 3 (H.266 / VVC)
- AV1
- AV2 (в разработке)
- H.120, H.261, H.262, H.263, H.264, H.265, H.266

#### Google / Open-source

- VP3, VP4, VP5, VP6, VP7, VP8, VP9

#### Apple

- ProRes 422
- ProRes 4444
- ProRes RAW

#### Avid

- DNxHD
- DNxHR

#### Microsoft

- VC-1
- WMV7, WMV8, WMV9

#### RealNetworks

- RealVideo RV10, RV20, RV30, RV40

#### CineForm

- CFHD

#### JPEG-семейство (видео)

- Motion JPEG (MJPEG)
- JPEG 2000 (MJ2)
- JPEG XS
- JPEG XT

#### Экспериментальные / нишевые

- Theora
- Dirac
- Schroedinger
- Snow
- Smacker Video
- Bink Video
- Indeo Video (2–5)
- Cinepak
- TechSmith Screen Capture

#### RAW видео

- BRAW, R3D, ARRIRAW, CinemaDNG

---

### 2. Аудио кодеки

#### Без сжатия

- PCM (signed/unsigned)
- Linear PCM
- Floating-point PCM
- DSD
- DST

#### С потерями (lossy)

- MP3, MP2
- AAC (LC, HE-AAC, HE-AAC v2, xHE-AAC)
- Vorbis
- Opus
- WMA
- AC-3 (Dolby Digital)
- E-AC-3
- DTS, DTS-HD
- ATRAC
- AMR-NB, AMR-WB
- Speex
- Musepack
- SBC
- LC3
- G.711, G.722, G.723, G.726, G.729
- QCELP
- EVRC

#### Без потерь (lossless)

- FLAC
- ALAC
- WavPack
- Monkey's Audio (APE)
- TAK
- TTA
- Shorten
- OptimFROG

#### Экспериментальные / устаревшие

- TwinVQ
- RealAudio
- Voxware
- GSM 06.10
- iLBC

#### Spatial Audio

- Dolby Atmos
- Ambisonics
- Binaural

#### MIDI/Tracker

- MIDI, MOD, XM, IT, S3M

---

### 3. Кодеки изображений

#### JPEG-семейство

- JPEG (Baseline, Progressive)
- JPEG-LS
- JPEG 2000
- JPEG XR
- JPEG XT
- JPEG XS
- JPEG XL

#### Web и современные

- WebP (lossy / lossless)
- AVIF (AV1 Image File Format)
- HEIF / HEIC (на базе HEVC)

#### Без сжатия или с минимальным сжатием

- RAW (обобщённо)
- BMP (uncompressed)
- TIFF (uncompressed)
- PAM
- PPM / PGM / PBM
- OpenEXR (uncompressed modes)

#### Без потерь (lossless)

- PNG
- APNG
- TIFF (LZW, Deflate, PackBits)
- WebP lossless
- JPEG-LS (lossless mode)
- JPEG 2000 (lossless)
- FLIF
- QOI
- OpenEXR (ZIP, PIZ, DWAA lossless)

#### HDR и high-precision

- OpenEXR
- Radiance HDR (RGBE)
- JPEG XT
- JPEG XL
- HEIF HDR
- AVIF HDR
- TIFF HDR
- LogLuv TIFF

#### RAW-кодеки (сенсорные)

- DNG
- CR2 / CR3 (Canon)
- NEF (Nikon)
- ARW (Sony)
- ORF (Olympus)
- RW2 (Panasonic)
- RAF (Fujifilm)
- SRW (Samsung)
- PEF (Pentax)
- X3F (Sigma)

#### Индексированные и палитровые

- GIF
- PNG-8
- BMP indexed
- TIFF palette

#### Устаревшие и нишевые

- PCX
- TGA (RLE)
- IFF ILBM
- Sun Raster
- SGI RGB
- Kodak Photo CD (PCD)
- MrSID (геоданные)
- ECW
- JBIG, JBIG2

#### Алгоритмы внутри форматов

- Huffman coding
- Arithmetic coding
- Run-Length Encoding (RLE)
- LZW
- Deflate
- LZ77
- Delta encoding
- Wavelet compression
- Predictive coding
- YCbCr subsampling
- ANS / rANS (AVIF, JXL)

---

### 4. 3D кодеки и форматы

#### Форматы сцен и моделей

- glTF / glTF 2.0
- FBX
- OBJ
- COLLADA (.dae)
- USD / USDA / USDC / USDZ
- Blender (.blend)
- Maya (.mb / .ma)
- 3ds Max (.max)
- Cinema 4D (.c4d)

#### CAD форматы

- STEP
- IGES
- STL
- 3MF

#### Кодеки геометрии

- Google Draco
- OpenCTM
- MPEG-4 3DMC
- MPEG-I G-PCC (Geometry-based Point Cloud Compression)
- MPEG-I V-PCC
- Edgebreaker
- Topological Surgery
- Predictive Geometry Compression

#### Кодеки анимации

- MPEG-4 BIFS
- MPEG-4 AFX
- FBX internal animation compression
- COLLADA animation compression
- USD internal compression

#### Воксели и Point Cloud

- Draco Point Cloud
- LASzip
- MPEG PCC

---

### 5. Документы (кодирование/сжатие)

#### Кодировки текста

- ASCII
- UTF-7, UTF-8, UTF-16, UTF-32
- ISO-8859 (все части)
- Windows-125x
- KOI8-R / KOI8-U
- Shift-JIS
- EUC-JP
- GB2312, GBK
- Big5

#### Алгоритмы сжатия

- ZIP (Deflate)
- Deflate64
- LZ77, LZ78
- LZW
- Brotli
- Zstandard
- LZMA, LZMA2
- PPMd
- Bzip2
- XZ

#### PDF-специфические

- FlateDecode
- ASCII85Decode
- ASCIIHexDecode
- RunLengthDecode
- CCITT Fax
- JBIG2
- JPEG
- JPEG2000

#### Форматы документов

- DOCX, XLSX, PPTX
- ODT, ODS, ODP
- PDF
- EPUB
- INDD (частично)
- TXT, MD, RTF, LaTeX

#### Код

- Все языки через tree-sitter

---

## Архитектура системы

```
┌─────────────────────────────────────────────────────────────┐
│                    RururuOS Desktop                          │
│            (Rust GUI: iced/slint или кастомный DE)           │
├─────────────────────────────────────────────────────────────┤
│                 Universal File Handler                       │
│                    (Rust daemon + API)                       │
├─────────────────────────────────────────────────────────────┤
│                    Codec Registry                            │
│                  (plugin-based система)                      │
├────────────┬────────────┬────────────┬──────────────────────┤
│   FFmpeg   │   Assimp   │  OpenEXR   │   Custom Rust libs   │
│  (media)   │    (3D)    │   (HDR)    │     (extensions)     │
├────────────┴────────────┴────────────┴──────────────────────┤
│                      PipeWire                                │
│           (audio: consumer + pro JACK-compatible)            │
├─────────────────────────────────────────────────────────────┤
│                    Wayland + wlroots                         │
├─────────────────────────────────────────────────────────────┤
│                   Linux Kernel (PREEMPT_RT)                  │
│              + NVIDIA/AMD/Intel GPU drivers                  │
└─────────────────────────────────────────────────────────────┘
```

---

## Rust-компоненты для разработки

| Компонент | Технология |
|-----------|------------|
| GUI Framework | iced, slint, egui |
| Wayland Compositor | smithay |
| Системные утилиты | uutils (замена coreutils) |
| Media handling | ffmpeg-next crate |
| 3D viewer | wgpu + gltf crate |
| Color management | OpenColorIO bindings |
| File manager | Кастомный с превью всех форматов |

---

## Ключевые системные пакеты

```bash
# Мультимедиа
ffmpeg gstreamer pipewire pipewire-jack wireplumber

# Кодеки
libavcodec libvorbis libopus flac lame x264 x265 aom

# 3D/Graphics
blender krita gimp inkscape darktable
openexr opencolorio assimp

# Документы
libreoffice pandoc poppler

# Разработка
rust cargo gcc clang cmake meson

# GPU
nvidia-dkms cuda (или mesa для AMD/Intel)
```

---

## План разработки

1. **Фаза 1**: Базовая система
   - [ ] Arch Linux minimal install
   - [ ] Скрипты автоустановки кодеков
   - [ ] PipeWire + realtime kernel config

2. **Фаза 2**: Rust инфраструктура
   - [ ] Universal File Handler daemon
   - [ ] Plugin API для кодеков
   - [ ] Rust-обёртки над C-библиотеками

3. **Фаза 3**: Desktop Environment
   - [ ] Wayland compositor (smithay) или настройка существующего
   - [ ] File manager с превью
   - [ ] Системные утилиты на Rust

4. **Фаза 4**: Интеграция
   - [ ] Color management на уровне системы
   - [ ] Профили для разных workflow (Video/3D/2D/Audio)
   - [ ] ISO-образ для установки

---

## Целевая аудитория

- 3D-моделеры и аниматоры
- Видеографы и colorists
- Графические дизайнеры
- Музыканты и sound designers
- Фотографы (RAW workflow)

---

## Ссылки и ресурсы

- [Arch Wiki](https://wiki.archlinux.org/)
- [FFmpeg Documentation](https://ffmpeg.org/documentation.html)
- [PipeWire](https://pipewire.org/)
- [Smithay (Rust Wayland)](https://github.com/Smithay/smithay)
- [iced GUI](https://github.com/iced-rs/iced)
- [uutils coreutils](https://github.com/uutils/coreutils)
