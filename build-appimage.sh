#!/bin/bash
set -ue

# Arch erkennen und passenden linuxdeploy wählen
ARCH=$(uname -m)
case "$ARCH" in
  x86_64)
    LINUXDEPLOY_URL="https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage"
    LINUXDEPLOY_BIN="linuxdeploy-x86_64.AppImage"
    OUT_ARCH="x86_64"
    ;;
  aarch64|arm64)
    LINUXDEPLOY_URL="https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-aarch64.AppImage"
    LINUXDEPLOY_BIN="linuxdeploy-aarch64.AppImage"
    OUT_ARCH="aarch64"
    ;;
  *)
    echo "Unsupported architecture: $ARCH" >&2
    exit 1
    ;;
 esac

# AppDir Struktur anlegen
APPDIR="AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/looksyk/static"
mkdir -p "$APPDIR/usr/share/icons/hicolor/256x256/apps"
mkdir -p "$APPDIR/usr/share/licenses/looksyk"
mkdir -p "$APPDIR/usr/share/metainfo"

cp target/looksyk "$APPDIR/usr/bin/looksyk-backend"
cp application-wrapper/Looksyk-AppRun "$APPDIR/AppRun"

if [ -f "$APPDIR/AppRun" ]; then
  chmod +x "$APPDIR/AppRun"
fi

cp -r target/static/* "$APPDIR/usr/share/looksyk/static/"

cp -r target/application-wrapper/* "$APPDIR/usr/share/looksyk/"

cp de.sebastianruziczka.looksyk.metainfo.xml "$APPDIR/usr/share/metainfo/de.sebastianruziczka.looksyk.appdata.xml"

cp icon/Looksyk_256.png "$APPDIR/usr/share/icons/hicolor/256x256/apps/de.sebastianruziczka.looksyk.png"

cp application-wrapper/Looksyk.desktop "$APPDIR/de.sebastianruziczka.looksyk.desktop"

cp LICENSE "$APPDIR/usr/share/licenses/looksyk/LICENSE"

if [ ! -f git/git ]; then
  git clone --depth 1 https://github.com/git/git
  cd git
  make
  cd ..
fi
cp git/git "$APPDIR/usr/bin/git"

# AppImage bauen
if [ ! -f "$LINUXDEPLOY_BIN" ]; then
  wget -O "$LINUXDEPLOY_BIN" "$LINUXDEPLOY_URL"
  chmod +x "$LINUXDEPLOY_BIN"
fi

./"$LINUXDEPLOY_BIN" --appdir "$APPDIR" -d "$APPDIR/de.sebastianruziczka.looksyk.desktop" -i "$APPDIR/usr/share/icons/hicolor/256x256/apps/de.sebastianruziczka.looksyk.png" --output appimage

# Ergebnisdatei ermitteln und mit Architektur suffixen
APPIMG=$(ls -1 *.AppImage | head -n1)
if [ -n "$APPIMG" ] && [ ! -f "Looksyk-$OUT_ARCH.AppImage" ]; then
  mv "$APPIMG" "Looksyk-$OUT_ARCH.AppImage"
fi

echo "AppImage gebaut: $(ls Looksyk-*.AppImage 2>/dev/null || true)"
