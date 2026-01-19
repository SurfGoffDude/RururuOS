# Codec Compatibility Matrix

## Video Codecs

| Codec | Decode | Encode | HW Accel | Notes |
|-------|--------|--------|----------|-------|
| H.264/AVC | ✅ | ✅ | ✅ NVENC/VAAPI/QSV | Universal support |
| H.265/HEVC | ✅ | ✅ | ✅ NVENC/VAAPI/QSV | 8/10-bit |
| AV1 | ✅ | ✅ | ✅ NVENC/VAAPI | RTX 40xx, Intel Arc |
| VP9 | ✅ | ✅ | ✅ VAAPI | Google/YouTube |
| VP8 | ✅ | ✅ | ⚠️ Limited | WebM legacy |
| ProRes | ✅ | ✅ | ❌ | FFmpeg software |
| DNxHD/HR | ✅ | ✅ | ❌ | Avid format |
| MPEG-2 | ✅ | ✅ | ✅ VAAPI | DVD/Broadcast |
| MPEG-4 | ✅ | ✅ | ⚠️ Limited | Legacy |
| Theora | ✅ | ✅ | ❌ | Open format |

## Audio Codecs

| Codec | Decode | Encode | Notes |
|-------|--------|--------|-------|
| AAC | ✅ | ✅ | libfdk_aac recommended |
| MP3 | ✅ | ✅ | LAME encoder |
| FLAC | ✅ | ✅ | Lossless |
| ALAC | ✅ | ✅ | Apple lossless |
| Opus | ✅ | ✅ | Best for voice/music |
| Vorbis | ✅ | ✅ | OGG container |
| WAV/PCM | ✅ | ✅ | Uncompressed |
| AC3/E-AC3 | ✅ | ✅ | Dolby Digital |
| DTS | ✅ | ⚠️ | Decode only free |
| TrueHD | ✅ | ❌ | Decode only |

## Image Formats

| Format | Read | Write | Notes |
|--------|------|-------|-------|
| JPEG | ✅ | ✅ | libjpeg-turbo |
| PNG | ✅ | ✅ | libpng |
| WebP | ✅ | ✅ | Google format |
| AVIF | ✅ | ✅ | AV1-based |
| HEIC/HEIF | ✅ | ✅ | libheif |
| TIFF | ✅ | ✅ | libtiff |
| OpenEXR | ✅ | ✅ | HDR/VFX |
| RAW (CR2/NEF/ARW) | ✅ | ❌ | libraw |
| PSD | ✅ | ⚠️ | Limited write |
| SVG | ✅ | ✅ | librsvg |
| GIF | ✅ | ✅ | Animated support |
| BMP | ✅ | ✅ | Legacy |
| ICO | ✅ | ✅ | Icons |

## Container Formats

| Format | Read | Write | Notes |
|--------|------|-------|-------|
| MP4/M4V | ✅ | ✅ | Most common |
| MKV | ✅ | ✅ | Matroska |
| WebM | ✅ | ✅ | Web video |
| MOV | ✅ | ✅ | QuickTime |
| AVI | ✅ | ✅ | Legacy |
| FLV | ✅ | ✅ | Flash video |
| OGG/OGV | ✅ | ✅ | Open container |
| MXF | ✅ | ✅ | Broadcast |
| TS/M2TS | ✅ | ✅ | MPEG transport |

## RAW Camera Support

| Brand | Formats | Status |
|-------|---------|--------|
| Canon | CR2, CR3, CRW | ✅ Full |
| Nikon | NEF, NRW | ✅ Full |
| Sony | ARW, SR2 | ✅ Full |
| Fujifilm | RAF | ✅ Full |
| Panasonic | RW2 | ✅ Full |
| Olympus | ORF | ✅ Full |
| Pentax | PEF, DNG | ✅ Full |
| Leica | DNG, RWL | ✅ Full |
| Hasselblad | 3FR, FFF | ✅ Full |
| Phase One | IIQ | ✅ Full |
| Adobe | DNG | ✅ Full |

## 3D Model Formats

| Format | Import | Export | Notes |
|--------|--------|--------|-------|
| glTF/GLB | ✅ | ✅ | Modern standard |
| FBX | ✅ | ⚠️ | Autodesk |
| OBJ | ✅ | ✅ | Simple geometry |
| STL | ✅ | ✅ | 3D printing |
| BLEND | ✅ | ✅ | Blender native |
| USD | ✅ | ✅ | Pixar format |
| 3DS | ✅ | ⚠️ | Legacy |
| DAE (Collada) | ✅ | ✅ | XML-based |
| PLY | ✅ | ✅ | Point clouds |

## Document Formats

| Format | Read | Write | Notes |
|--------|------|-------|-------|
| PDF | ✅ | ✅ | poppler/mupdf |
| EPUB | ✅ | ⚠️ | E-books |
| Markdown | ✅ | ✅ | Native |
| LaTeX | ✅ | ✅ | TeX Live |
| ODT | ✅ | ✅ | LibreOffice |
| DOCX | ✅ | ✅ | LibreOffice |

## Hardware Acceleration

### NVIDIA (NVENC/NVDEC)

| Feature | GTX 10xx | RTX 20xx | RTX 30xx | RTX 40xx |
|---------|----------|----------|----------|----------|
| H.264 Dec | ✅ | ✅ | ✅ | ✅ |
| H.264 Enc | ✅ | ✅ | ✅ | ✅ |
| HEVC Dec | ✅ | ✅ | ✅ | ✅ |
| HEVC Enc | ⚠️ | ✅ | ✅ | ✅ |
| AV1 Dec | ❌ | ❌ | ✅ | ✅ |
| AV1 Enc | ❌ | ❌ | ❌ | ✅ |

### AMD (VCN)

| Feature | Polaris | Vega | RDNA 1 | RDNA 2+ |
|---------|---------|------|--------|---------|
| H.264 Dec | ✅ | ✅ | ✅ | ✅ |
| H.264 Enc | ✅ | ✅ | ✅ | ✅ |
| HEVC Dec | ✅ | ✅ | ✅ | ✅ |
| HEVC Enc | ⚠️ | ✅ | ✅ | ✅ |
| AV1 Dec | ❌ | ❌ | ❌ | ✅ |
| AV1 Enc | ❌ | ❌ | ❌ | ⚠️ |

### Intel (QSV)

| Feature | Gen 9 | Gen 11 | Gen 12 | Arc |
|---------|-------|--------|--------|-----|
| H.264 Dec | ✅ | ✅ | ✅ | ✅ |
| H.264 Enc | ✅ | ✅ | ✅ | ✅ |
| HEVC Dec | ✅ | ✅ | ✅ | ✅ |
| HEVC Enc | ⚠️ | ✅ | ✅ | ✅ |
| AV1 Dec | ❌ | ❌ | ✅ | ✅ |
| AV1 Enc | ❌ | ❌ | ⚠️ | ✅ |
