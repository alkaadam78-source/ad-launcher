#!/bin/bash
set -e

MAX_SIZE=50000000 # 50 MB in bytes
FAILED=0

echo "=========================================================="
echo "AD LAUNCHER — PACKAGE SIZE BUDGET VALIDATOR (< 50 MB)"
echo "=========================================================="

ARTIFACTS=(
    "ad-launcher_0.1.0-beta.1_amd64.deb"
    "ad-launcher-0.1.0-beta.1.AppImage"
    "ad-launcher-0.1.0-beta.1-linux-x86_64.tar.gz"
    "target/release/ad-launcher"
)

for file in "${ARTIFACTS[@]}"; do
    if [ -f "$file" ]; then
        SIZE=$(stat -c%s "$file")
        HUMAN=$(ls -lh "$file" | awk '{print $5}')
        if [ "$SIZE" -le "$MAX_SIZE" ]; then
            PERCENT=$(( SIZE * 100 / MAX_SIZE ))
            echo " [PASS] $file ($HUMAN, $SIZE bytes) — ${PERCENT}% of 50MB budget"
        else
            OVER=$(( SIZE - MAX_SIZE ))
            echo " [FAIL] $file ($HUMAN, $SIZE bytes) — EXCEEDS 50MB budget by $OVER bytes!"
            FAILED=1
        fi
    else
        echo " [WARN] $file not found, skipping."
    fi
done

echo "=========================================================="
if [ "$FAILED" -eq 0 ]; then
    echo "STATUS: ALL CHECKED ARTIFACTS SATISFY < 50 MB BUDGET!"
    exit 0
else
    echo "STATUS: ONE OR MORE ARTIFACTS EXCEEDED 50 MB BUDGET!"
    exit 1
fi
