# NUX Launcher Linux — Personal Edition

> **High-Performance Native Minecraft Launcher for Debian Linux**  
> Engineered specifically for Intel Core i3-7100U / Intel HD Graphics 620 / 8 GB RAM targets.

![NUX Launcher Linux](assets/icons/nux-launcher.svg)

---

## Overview

**NUX Launcher Linux** is an ultra-lightweight, 100% native Linux Minecraft launcher written in **Rust**, **GTK4**, and **Relm4**. It eliminates the massive RAM, CPU, and process overhead of Electron/Chromium-based launchers, providing an instantaneous startup, minimal background resource usage, and a workflow tailored for Minecraft 1.20.1 performance.

---

## Key Hardware & Performance Philosophy

Built and benchmarked for Debian Linux on modern budget/laptop hardware:

- **Target CPU**: Intel Core i3-7100U (2 cores, 4 threads @ 2.40GHz)
- **Target GPU**: Intel HD Graphics 620 (Mesa / Vulkan / OpenGL 4.6)
- **System Memory**: 8 GB RAM (leaves >= 5 GB free for desktop & OS)
- **Display Target**: 1366x768 and 1080p responsive scaling
- **Zero Electron Overhead**: Under 25 MB RAM idle; zero background Chromium helper processes
- **RAM Guard**: Automatically detects system RAM and prevents JVM over-allocation

### Tuned Hardware Presets:

| Preset | Max RAM | Render Distance | Sim Distance | Max FPS | Target Setup |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **BALANCED** *(Default)* | **3072 MB** | 6 Chunks | 4 Chunks | 120 FPS | Sodium + Iris + Lithium + FerriteCore |
| **LOW RESOURCE** | **2048 MB** | 4 Chunks | 3 Chunks | 90 FPS | Pure performance, shaders off, low heat |
| **CUSTOM** | User | Configurable | Configurable | Uncapped | Custom flags and memory allocations |

---

## Feature Matrix

### 1. Official Minecraft Engine
- Direct communication with official Mojang endpoints (`piston-meta.mojang.com`)
- Version metadata parsing with full Linux x86_64 library rule evaluation
- Chunked parallel downloader with SHA-1 checksum verification
- Native library extraction (`natives/` directory)
- Full JVM and Game argument generator with low-pause G1GC flags

### 2. Mod Loader & Performance Integration
- **Fabric**: First-class 1.20.1 integration via Fabric Meta API (`meta.fabricmc.net`)
- **Sodium**: Next-generation OpenGL replacement renderer
- **Iris**: Shader loader compatible with modern Sodium pipeline
- **Lithium**: General-purpose physics, AI, and chunk ticking optimization
- **FerriteCore**: Heavy memory reduction mod reducing Java heap pressure
- **Modrinth API v2**: Integrated mod search, version resolution, and 1-click download directly to `mods/`
- **Quilt & Forge/NeoForge**: Extensible loader profiles

### 3. User Interface (GTK4 + Relm4)
- **Minimalist Dark Theme**: Black, charcoal (`#121418`), slate (`#1a1d24`), with emerald action accents
- **No RGB, no neon, no bloat**
- **7 Main Views**:
  1. **Home**: Active instance, Java status, RAM allocation, Big Play button, real-time log viewer.
  2. **Instances**: Create, switch, open game folder, and manage isolated Minecraft profiles.
  3. **Mods**: List mods, toggle enabled/disabled via `.jar.disabled`, install performance mods from Modrinth.
  4. **Downloads**: Version manifest browser and complete 1.20.1 asset/library downloader.
  5. **Settings**: Memory slider, presets, custom Java paths, JVM arguments, and resolution controls.
  6. **Account**: Secure Microsoft Device Code Flow authentication and offline/local player profiles.
  7. **About**: Real-time system hardware diagnostics (CPU, GPU, RAM, OS).

### 4. Account Management
- **Microsoft OAuth 2.0**: RFC 8628 Device Code Flow (`microsoft.com/devicelogin`)
  - No passwords ever stored or requested
  - Xbox Live (XBL) & XSTS token exchange
  - Minecraft license & ownership validation
- **Local Personal Profile**: Fast offline name selection for LAN or personal offline sessions

---

## Quick Start

### Running from source:
```bash
cargo run --release
```

### Direct command-line launch:
```bash
nux-launcher --launch
```

---

## License

NUX Launcher Linux is licensed under the **GNU General Public License v3.0** (`GPL-3.0-or-later`).
See [LICENSE](LICENSE) for details.
