#!/bin/sh
set -ue

cd frontend/looksyk &&  npm ci --cache ./npm-cache && cd ../../

cd application-wrapper/Looksyk
npm install
npm run package
cd ../../


cd backend && cargo fetch && cd ..

mkdir -p backend/cargo-cache
cp -r ~/.cargo/registry backend/cargo-cache/

tar --exclude-vcs \
   --exclude='./**/dist' \
   --exclude='./**/node_modules' \
   --exclude="./target" \
   --exclude="./.flatpak-builder" \
   --exclude="./build-dir" \
   --exclude="./builddir" \
   --exclude="./repo" \
   --exclude="./.idea" \
   --exclude="*.flatpak" \
   --exclude="./looksyk-source.tar.gz" \
   --dereference -czf looksyk-source.tar.gz .
#   --exclude='./**/out' \
