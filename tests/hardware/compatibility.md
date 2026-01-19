# Hardware Compatibility Matrix

## CPUs

| Vendor | Architecture | Status | Notes |
|--------|--------------|--------|-------|
| Intel | x86_64 (Core 2+) | ✅ Tested | Full support |
| AMD | x86_64 (Zen+) | ✅ Tested | Full support |
| Apple | ARM64 (M1/M2/M3) | ✅ Tested | Via Asahi Linux |
| Raspberry Pi | ARM64 (BCM2711+) | ✅ Tested | Pi 4/5 |
| Qualcomm | ARM64 | ⚠️ Untested | Should work |

## GPUs

### NVIDIA

| Series | Driver | Status | Notes |
|--------|--------|--------|-------|
| RTX 40xx | nvidia 545+ | ✅ Tested | Full support |
| RTX 30xx | nvidia 525+ | ✅ Tested | Full support |
| RTX 20xx | nvidia 470+ | ✅ Tested | Full support |
| GTX 16xx | nvidia 470+ | ✅ Tested | Full support |
| GTX 10xx | nvidia 470+ | ✅ Tested | Full support |
| GTX 9xx | nvidia 470+ | ⚠️ Partial | Legacy driver |
| Older | nouveau | ⚠️ Partial | Basic support |

### AMD

| Series | Driver | Status | Notes |
|--------|--------|--------|-------|
| RX 7000 | amdgpu | ✅ Tested | Full support |
| RX 6000 | amdgpu | ✅ Tested | Full support |
| RX 5000 | amdgpu | ✅ Tested | Full support |
| Vega | amdgpu | ✅ Tested | Full support |
| Polaris | amdgpu | ✅ Tested | Full support |
| GCN 1-3 | amdgpu/radeon | ⚠️ Partial | Use amdgpu |

### Intel

| Generation | Driver | Status | Notes |
|------------|--------|--------|-------|
| Arc | i915/xe | ✅ Tested | Full support |
| Gen 12 (Xe) | i915 | ✅ Tested | Full support |
| Gen 9-11 | i915 | ✅ Tested | Full support |
| Gen 7-8 | i915 | ⚠️ Partial | Older hardware |

### Apple Silicon

| Chip | Driver | Status | Notes |
|------|--------|--------|-------|
| M3 | asahi | ✅ Tested | Via Asahi |
| M2 | asahi | ✅ Tested | Via Asahi |
| M1 | asahi | ✅ Tested | Via Asahi |

## Audio

| Interface | Status | Notes |
|-----------|--------|-------|
| Intel HDA | ✅ Tested | PipeWire |
| USB Audio | ✅ Tested | Class-compliant |
| Focusrite Scarlett | ✅ Tested | Full support |
| Behringer UMC | ✅ Tested | Full support |
| MOTU | ⚠️ Partial | Some models |
| RME | ✅ Tested | Full support |
| Bluetooth | ✅ Tested | A2DP/AAC/LDAC |

## Displays

| Connection | Status | Notes |
|------------|--------|-------|
| HDMI 2.1 | ✅ Tested | 4K120, VRR |
| DisplayPort 1.4 | ✅ Tested | 4K120, VRR |
| USB-C/DP | ✅ Tested | Alt mode |
| Thunderbolt | ✅ Tested | Daisy chain |
| VGA | ⚠️ Partial | Via adapter |

### HDR Support

| Monitor Type | Status | Notes |
|--------------|--------|-------|
| HDR10 | ✅ Tested | Requires compatible GPU |
| HDR400 | ✅ Tested | Limited HDR |
| HDR600+ | ✅ Tested | Full HDR |
| Dolby Vision | ❌ Not supported | No Linux support |

## Storage

| Type | Status | Notes |
|------|--------|-------|
| NVMe | ✅ Tested | Full support |
| SATA SSD | ✅ Tested | Full support |
| SATA HDD | ✅ Tested | Full support |
| USB 3.x | ✅ Tested | Full support |
| SD Card | ✅ Tested | Full support |
| Thunderbolt | ✅ Tested | External drives |

## Network

| Type | Status | Notes |
|------|--------|-------|
| Intel WiFi 6/6E | ✅ Tested | Full support |
| Realtek WiFi | ⚠️ Partial | Driver varies |
| Broadcom WiFi | ⚠️ Partial | May need firmware |
| Intel Ethernet | ✅ Tested | Full support |
| Realtek Ethernet | ✅ Tested | Full support |

## Input Devices

| Type | Status | Notes |
|------|--------|-------|
| Wacom tablets | ✅ Tested | Full support |
| Huion tablets | ✅ Tested | Full support |
| XP-Pen tablets | ✅ Tested | Full support |
| Gaming mice | ✅ Tested | libratbag |
| Mechanical keyboards | ✅ Tested | Full support |

## Tested Configurations

### Desktop

| Config | Components | Status |
|--------|------------|--------|
| AMD Workstation | Ryzen 9, RX 6800, 64GB | ✅ Pass |
| Intel Gaming | i9-12900K, RTX 4080, 32GB | ✅ Pass |
| Budget Build | Ryzen 5, GTX 1660, 16GB | ✅ Pass |

### Laptop

| Model | Status | Notes |
|-------|--------|-------|
| ThinkPad X1 Carbon | ✅ Pass | Full support |
| Dell XPS 15 | ✅ Pass | Full support |
| Framework 16 | ✅ Pass | Full support |
| MacBook Pro M2 | ✅ Pass | Via Asahi |
| ASUS ROG | ✅ Pass | NVIDIA hybrid |

### ARM

| Device | Status | Notes |
|--------|--------|-------|
| Raspberry Pi 5 | ✅ Pass | 8GB recommended |
| Raspberry Pi 4 | ✅ Pass | 4GB minimum |
| MacBook Air M1 | ✅ Pass | Via Asahi |
| Mac Mini M2 | ✅ Pass | Via Asahi |
