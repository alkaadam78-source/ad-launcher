#!/bin/bash
set -e
HERE="$(dirname "$(readlink -f "$0")")"
PROJECT_ROOT="$(dirname "$(dirname "$HERE")")"
APPDIR="$HERE/AppDir"
OUTPUT="$PROJECT_ROOT/nux-launcher-1.0.0.AppImage"

# Create self-extracting script header
cat << 'HEADER' > "$OUTPUT"
#!/bin/bash
set -e
TMPDIR=$(mktemp -d /tmp/nux-launcher-XXXXXX)
trap 'rm -rf "$TMPDIR"' EXIT
sed -e '1,/^__SQUASHFS_CONTENT__/d' "$0" > "$TMPDIR/app.sqfs"
if command -v unsquashfs >/dev/null 2>&1; then
    unsquashfs -q -f -d "$TMPDIR/app" "$TMPDIR/app.sqfs"
    exec "$TMPDIR/app/AppRun" "$@"
else
    echo "unsquashfs required to run this portable bundle."
    exit 1
fi
exit 0
__SQUASHFS_CONTENT__
HEADER

cat "$PROJECT_ROOT/nux-launcher-1.0.0.squashfs" >> "$OUTPUT"
chmod +x "$OUTPUT"
ls -lh "$OUTPUT"
