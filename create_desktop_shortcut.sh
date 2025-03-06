#! /bin/sh

read -p "enter graph name: " graphname
read -p "enter graph path (absolute path preferred): " graphpath
read -p "enter unused port for looksyk (or just use 8989): " graphport


echo "creating [Desktop Entry] for $graphname"

echo "[Desktop Entry]
Type=Application
Name=Looksyk - $graphname
GenericName=Markdown knowledge platform
Icon=/usr/share/looksyk/icon.png
Terminal=false
Exec=looksyk --graph-location $graphpath --port $graphport" >   ~/.local/share/applications/Looksyk-"$graphname".desktop

echo "Done";
