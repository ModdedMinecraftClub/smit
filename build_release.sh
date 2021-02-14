#!/bin/bash
set -e

rm -rf frontend/dist frontend/node_modules /frontend/.routify
cargo clean
cd frontend
npm install
npm run buildRelease
cd ..
cargo build --release
