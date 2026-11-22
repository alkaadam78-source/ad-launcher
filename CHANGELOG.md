# Changelog

All notable changes to AD Launcher will be documented in this file.

## [0.1.0-beta.1] — 2026-11-22

### Added
- **Dark Baroque Complete Redesign**: Full application-wide European Baroque aesthetic featuring Chiaroscuro lighting, antique gold accents, and deep walnut surfaces.
- **Baroque Decorative Placement System**: Symmetrical fleur-de-lis section dividers, refined page headers, and minimalist modern buttons adhering to the 10 placement rules.
- **Universal Instance Importer**: Non-destructive discovery and migration engine for Vanilla `.minecraft`, Prism Launcher, and MultiMC profiles.
- **Instance Portability & Snapshot System**: Creation of portable `.zip` archives and reversible instance snapshots with metadata preservation.
- **Advanced Multi-Java Manager**: Detection and telemetry for installed OpenJDK, Eclipse Temurin, and GraalVM runtimes on Linux.
- **Sanitized Diagnostics Assistant**: Redacted troubleshooting report generator for GitHub issue reporting without credential exposure.
- **Demo Mode**: Safe explorer simulation profile utilizing sample data without launching external JVM processes.
- **Multilingual Support**: Centralized i18n infrastructure supporting English (US) and Bahasa Indonesia.
- **Court Multiplayer & Discovery**: Curated server showcase with live TCP latency pinging and compatibility telemetry.
- **Local-First Analytics**: Private session playtime tracker stored locally in SQLite with zero telemetry.

### Changed
- Migrated crate naming from `nux-launcher` to `ad-launcher` with standardized MIT licensing.
- Optimized release compilation profile with thin LTO and binary stripping (~6.9 MB standalone executable).
- Updated Debian packaging and AppImage configurations to AD Launcher standards.
