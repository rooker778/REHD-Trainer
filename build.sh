#!/bin/sh
set -e
if [ ! -f assets/trainer.ico ]; then
    python tools/make_assets.py
fi
rustc -O src/main.rs -o REHDTrainer.exe -C strip=symbols
echo "Built REHDTrainer.exe"
