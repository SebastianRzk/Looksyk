#!/bin/bash
set -ue

pushd frontend/looksyk
npm install
npm run build --configuration=production
popd

pushd backend
cargo build --release
popd

pushd application-wrapper/Looksyk/
npm install
npm run package
popd



mkdir -p target
cp backend/target/release/looksyk target/
mkdir -p target/static
cp -rf frontend/looksyk/dist/looksyk/browser/* target/static
mkdir -p target/application-wrapper
cp -r application-wrapper/Looksyk/out/looksyk-linux-x64/* target/application-wrapper

echo "done";
