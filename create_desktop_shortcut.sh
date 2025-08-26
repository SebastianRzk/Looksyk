#! /bin/sh

read -p "enter graph name: " graphname
read -p "enter graph path (absolute path preferred): " graphpath
read -p "enter unused port for looksyk (or just use 8989): " graphport


echo "creating [Desktop Entry] for $graphname"

echo "[Desktop Entry]
Type=Application
StartupWMClass=looksyk
Name=Looksyk - $graphname
GenericName=A markdown centric, fast and local personal knowledge platform
Icon=de.sebastianruziczka.looksyk
Terminal=false
Categories=Office;Viewer;TextTools;TextEditor;Documentation;
Exec=looksyk --graph-location $graphpath --port $graphport" >   ~/.local/share/applications/Looksyk-"$graphname".desktop

echo "Done";
