# Installing NUX Launcher Linux

NUX Launcher Linux can be installed as a standard native Debian package (`.deb`), installed to user-local space (`~/.local/bin`), or run directly from the portable binary.

---

## Method 1: Install via Debian Package (.deb)

If you have the `.deb` package built:

```bash
sudo dpkg -i nux-launcher_1.0.0_amd64.deb
sudo apt-get install -f # If any runtime dependencies need resolving
```

This installs:
- Binary: `/usr/bin/nux-launcher`
- Desktop shortcut: `/usr/share/applications/nux-launcher.desktop`
- Application icon: `/usr/share/icons/hicolor/scalable/apps/nux-launcher.svg`

### Uninstalling:
```bash
sudo dpkg -r nux-launcher
```

*Your personal user data in `~/.minecraft` and `~/.config/nux-launcher` is preserved during upgrades and uninstallation.*

---

## Method 2: User-Local Installation (No Sudo Required)

To install NUX Launcher for the current user without needing root privileges:

```bash
mkdir -p ~/.local/bin ~/.local/share/applications ~/.local/share/icons/hicolor/scalable/apps

# Copy binary
cp target/release/nux-launcher ~/.local/bin/
chmod +x ~/.local/bin/nux-launcher

# Copy icon and desktop entry
cp assets/icons/nux-launcher.svg ~/.local/share/icons/hicolor/scalable/apps/
cp packaging/debian/nux-launcher.desktop ~/.local/share/applications/

# Update desktop database
update-desktop-database ~/.local/share/applications/ 2>/dev/null || true
```

Ensure `~/.local/bin` is in your `$PATH`.

---

## Method 3: Direct Portable Run

You can run the binary directly from any location:

```bash
./target/release/nux-launcher
```

Or run directly in headless launch mode:
```bash
./target/release/nux-launcher --launch
```
