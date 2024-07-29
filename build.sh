#!/bin/bash


pushd frontend/looksyk
npm install
npm run build --configuration=production
popd

pushd backend
cargo build --release
popd


mkdir target
cp backend/target/release/looksyk target/
mkdir -p target/static
cp -rf frontend/looksyk/dist/looksyk/browser/* target/static
