#!/bin/bash

set -ue

pushd application-wrapper/Looksyk
npm run package
npm run lint
popd


pushd frontend/looksyk
npm run build --configuration=production
npm run lint
popd


pushd backend
cargo build
cargo test
cargo fmt
cargo clippy
popd

echo "all done"
