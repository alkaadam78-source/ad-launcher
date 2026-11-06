# Known Limitations & Platform Notes

This document describes technical boundaries, known limitations, and operational characteristics of NUX Launcher Linux v1.0.0 on Debian.

---

## 1. Graphics Pipeline & Wayland Integration
- **Wayland vs X11**: NUX Launcher runs natively under Wayland (`GDK_BACKEND=wayland`) and X11 (`GDK_BACKEND=x11`).
- **Intel HD Graphics 620 OpenGL Limits**: The integrated Intel HD 620 supports OpenGL 4.6 via the Mesa `iris` driver.
  - Recommended shader packs: **Complementary Reimagined (Low Profile)**, **Sildur's Enhanced Default**, or **MakeUp Ultra Fast**.
  - Heavy path-traced shaders (such as SEUS PTGI or Kappa) will run below 20 FPS due to the hardware limitations of the 24 EU Intel GPU.
- **Sodium Version Compatibility**: For Minecraft 1.20.1, Sodium versions 0.5.x (`sodium-fabric-0.5.13+mc1.20.1`) and Iris 1.7.x (`iris-1.7.6+mc1.20.1`) are required. Installing Sodium 0.6.x (which is for 1.21+) will cause Fabric to reject startup due to Minecraft version mismatches.

---

## 2. Java Runtime Constraints
- **Java 17 Requirement**: Minecraft 1.20.1 requires Java 17 (Java 16 and below are unsupported by modern LWJGL 3.3.1).
- **64-bit Architecture**: Only 64-bit Java runtimes are supported. 32-bit runtimes are automatically rejected by the Java probe engine because Minecraft 1.20.1 requires native 64-bit memory addressing for chunk caching.
- **Wayland Window Decoration**: OpenJDK 17 under pure Wayland may sometimes use XWayland fallback depending on whether the `-Dawt.useSystemAAFontSettings=on` or LWJGL Wayland glfw backend is active.

---

## 3. Account & Authentication Notes
- **Microsoft Device Flow Polling**: Microsoft OAuth 2.0 Device Code authorization expires after 15 minutes. If authorization is not completed in the browser within this window, the login request must be re-initiated.
- **Game Ownership**: Microsoft login requires an active Minecraft Java Edition license attached to the Microsoft account. Accounts that only own Bedrock on mobile/console will be reported by the entitlements endpoint as unowned.
- **Offline / Local Nicknames**: Offline accounts generate deterministic UUIDs according to standard Mojang specification (`Uuid::new_v5(&Uuid::NAMESPACE_DNS, "OfflinePlayer:<name>")`). These allow joining offline-mode / LAN servers but cannot join online-mode authenticated servers.

---

## 4. Mod Loader Ecosystem
- **Primary Loader**: Fabric 1.20.1 is prioritized, fully tested, and automatically configured with Sodium and Iris.
- **Forge / NeoForge**: Requires manual installer execution or pre-installed profile in `.minecraft/versions` because Forge's 1.20.1 installer uses complex bytecode remapping processors that are best executed via official Forge installer jars.
- **CurseForge API**: Requires a developer API key (`x-api-key`) from Eternal API due to CurseForge's API paywall/restrictions. Modrinth is used as the primary un-gated open-source mod source.
