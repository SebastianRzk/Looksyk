#! /bin/sh

read -p "enter graph name: " graphname
read -p "enter graph path (absolute path preferred): " graphpath
read -p "enter unused port for looksyk (or just use 8989): " graphport


echo "creating [Desktop Entry] for $graphname"

echo "[Desktop Entry]
Type=Application
Name=Looksyk - $graphname
GenericName=Markdown knowledge platform
Icon=$HOME/.local/share/icons/looksyk-logo.png
Path=$PWD/target/
Terminal=true
Exec=bash -c 'cd \"$PWD/target/\" && $PWD/target/looksyk --graph-location $graphpath --port $graphport'" >   ~/.local/share/applications/Looksyk-"$graphname".desktop

cp ./icon/Looksyk-scaled.png ~/.local/share/icons/looksyk-logo.png

echo "Done";
