#!/bin/bash
set -ue

# AppDir Struktur anlegen
APPDIR="AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/looksyk/static"
mkdir -p "$APPDIR/usr/share/icons/hicolor/256x256/apps"
mkdir -p "$APPDIR/usr/share/licenses/looksyk"

# Backend-Binary kopieren
cp target/looksyk "$APPDIR/usr/bin/looksyk-backend"
# Electron-Client kopieren
cp application-wrapper/looksyk "$APPDIR/usr/bin/looksyk"
chmod +x "$APPDIR/usr/bin/looksyk"

# AppRun ausführbar machen, falls vorhanden
if [ -f "$APPDIR/AppRun" ]; then
  chmod +x "$APPDIR/AppRun"
fi

# Statische Dateien kopieren
cp -r target/static/* "$APPDIR/usr/share/looksyk/static/"

# app.asar kopieren
cp target/application-wrapper/resources/app.asar "$APPDIR/usr/share/looksyk/app.asar"

# Icon kopieren
cp icon/Looksyk_256.png "$APPDIR/usr/share/icons/hicolor/256x256/apps/de.sebastianruziczka.looksyk.png"

# Desktop Datei kopieren
cp application-wrapper/Looksyk.desktop "$APPDIR/looksyk.desktop"

# Lizenz kopieren
cp LICENSE "$APPDIR/usr/share/licenses/looksyk/LICENSE"

# AppImage bauen
if [ ! -f linuxdeploy ]; then
  wget -O linuxdeploy https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
  chmod +x linuxdeploy
fi

./linuxdeploy --appdir "$APPDIR" -d "$APPDIR/looksyk.desktop" -i "$APPDIR/usr/share/icons/hicolor/256x256/apps/de.sebastianruziczka.looksyk.png" --output appimage

echo "AppImage gebaut: $(ls *.AppImage)"
