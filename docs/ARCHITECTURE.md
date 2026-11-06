# NUX Launcher Linux — Technical Architecture

```
nux-launcher/
├── Cargo.toml
├── assets/
│   ├── icons/
│   │   └── nux-launcher.svg
│   └── styles/
│       └── style.css
├── src/
│   ├── main.rs                   # Entry point, CLI args, GTK Application lifecycle
│   ├── app.rs                    # Main window, navigation, event orchestrator
│   ├── config/
│   │   ├── mod.rs
│   │   ├── presets.rs            # BALANCED, LOW RESOURCE, CUSTOM presets
│   │   └── settings.rs           # JSON persistence in ~/.config/nux-launcher/settings.json
│   ├── core/
│   │   ├── mod.rs
│   │   ├── authentication.rs     # Microsoft OAuth Device Code Flow & Offline Profile
│   │   ├── downloader.rs         # Async chunked downloader with SHA-1 validation
│   │   ├── instances.rs          # SQLite database instance manager
│   │   ├── java.rs               # Detection & compatibility testing for Java 17+ runtimes
│   │   ├── launcher.rs           # Core launcher coordinator
│   │   ├── manifest.rs           # Mojang Manifest v2 parser & Linux library rule evaluation
│   │   ├── minecraft.rs          # JVM & Game arguments builder, Fabric classpath injection
│   │   ├── mods.rs               # Mod inspector, jar toggler (.jar.disabled), performance scanner
│   │   └── process.rs            # Child process execution, stdout/stderr streaming, crash detection
│   ├── integrations/
│   │   ├── mod.rs
│   │   ├── fabric.rs             # Fabric Meta API client
│   │   ├── modrinth.rs           # Modrinth v2 API client (mod search & 1-click download)
│   │   ├── curseforge.rs         # CurseForge API client
│   │   ├── forge.rs              # Forge profile structures
│   │   ├── neoforge.rs           # NeoForge status
│   │   └── quilt.rs              # Quilt Meta client
│   └── utils/
│       ├── mod.rs
│       ├── crypto.rs             # SHA-1 calculation & verification
│       ├── fs.rs                 # Safe file operations & zip extraction
│       └── system.rs             # sysinfo specs (CPU, GPU, RAM limits)
└── tests/                        # Integration and unit tests
```

---

## 1. Process Lifecycle & Execution Flow

1. **Launcher Initialization**:
   - Reads `~/.config/nux-launcher/settings.json` (or creates defaults).
   - Initializes `launcher.db` SQLite connection.
   - Probes host system for Java 17+ 64-bit binaries.
   - Loads active account (`auth.json` or offline name).

2. **Launch Sequence**:
   - Validates existence of `versions/{version}/{version}.jar` and `{version}.json`.
   - Resolves all Linux x86_64 library dependencies from `libraries/`.
   - Unpacks any native `.so` files into `natives/{version}/`.
   - If **Fabric**: merges Fabric loader libraries from `.minecraft/libraries/net/fabricmc/` and prepends them to the classpath, setting Knot client as main class (`net.fabricmc.loader.impl.launch.knot.KnotClient`).
   - Generates G1GC low-pause garbage collection arguments.
   - Generates Game arguments (`--username`, `--version`, `--gameDir`, `--assetsDir`, `--assetIndex`, `--uuid`, `--accessToken`, etc.).
   - Spawns child process via `std::process::Command` with redirected stdout/stderr pipes.
   - Pushes real-time log events to the UI console viewer using MPSC channels.

---

## 2. Security & Account Privacy

- **Zero Password Storage**: The launcher never asks for or stores user Microsoft passwords.
- **Device Code Flow (RFC 8628)**: The user authorizes directly on Microsoft's official login portal (`microsoft.com/devicelogin`).
- **File Permissions**: Saved auth tokens in `~/.config/nux-launcher/auth.json` are restricted to `0600` (read/write only by the owner).
- **No Token Printing**: Access tokens and bearer headers are strictly masked and never printed to console or logs.
