# Contributing to AD Launcher

Thank you for your interest in contributing to AD Launcher (The Dark Baroque Edition).

## Development Setup

### System Prerequisites (Debian / Ubuntu)
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libgtk-4-dev libgl1-mesa-dev libsqlite3-dev
```

### Building & Testing
```bash
# Verify formatting
cargo fmt --all -- --check

# Compile and type check
cargo check

# Run automated tests
cargo test

# Build release binary
cargo build --release
```

## Architectural Guidelines
1. **Dark Baroque Visual Identity**: All new views must use `create_page_header` and `create_section_divider` from `src/ui/components.rs`.
2. **Minimalist Buttons**: Buttons must remain clean, modern, and uncluttered.
3. **Privacy First**: Never commit personal accounts, Microsoft OAuth secrets, or Minecraft world data.
4. **Local-First Architecture**: All diagnostics, analytics, and mod caches remain strictly on the local machine.
