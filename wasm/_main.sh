#!/bin/sh
set -e
echo "Setup"
CONTAINER_DIR="$( cd "$( dirname "$0" )" && pwd )"
cd $CONTAINER_DIR
pwd
echo "Build"
wasm-pack build --target web
wasm-pack test --firefox --headless --release
