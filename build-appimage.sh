#!/bin/bash
set -ue

# AppDir Struktur anlegen
APPDIR="AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/looksyk/static"
mkdir -p "$APPDIR/usr/share/icons/hicolor/256x256/apps"
mkdir -p "$APPDIR/usr/share/licenses/looksyk"

cp target/looksyk "$APPDIR/usr/bin/looksyk-backend"
cp application-wrapper/Looksyk-AppRun "$APPDIR/AppRun"

if [ -f "$APPDIR/AppRun" ]; then
  chmod +x "$APPDIR/AppRun"
fi

cp -r target/static/* "$APPDIR/usr/share/looksyk/static/"

cp -r target/application-wrapper/* "$APPDIR/usr/share/looksyk/"

cp icon/Looksyk_256.png "$APPDIR/usr/share/icons/hicolor/256x256/apps/de.sebastianruziczka.looksyk.png"

cp application-wrapper/Looksyk.desktop "$APPDIR/looksyk.desktop"

cp LICENSE "$APPDIR/usr/share/licenses/looksyk/LICENSE"

if [ ! -f git-static ]; then
  wget -O git-static https://www.kernel.org/pub/software/scm/git/git-2.9.5.tar.gz
  tar -xzf git-static -C .
  cp git-2.45.1-linux-x86_64-static/bin/git "$APPDIR/usr/bin/git"
  rm -rf git-2.45.1-linux-x86_64-static git-static
fi

# AppImage bauen
if [ ! -f linuxdeploy ]; then
  wget -O linuxdeploy https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
  chmod +x linuxdeploy
fi

./linuxdeploy --appdir "$APPDIR" -d "$APPDIR/looksyk.desktop" -i "$APPDIR/usr/share/icons/hicolor/256x256/apps/de.sebastianruziczka.looksyk.png" --output appimage

echo "AppImage gebaut: $(ls *.AppImage)"
