#!/bin/bash

# 修复Transport::new调用
files=$(find crates/openlark-docs/src/bitable -name "*.rs" -type f | xargs grep -l "Transport::new(&self.config)")

for file in $files; do
    echo "Fixing Transport in $file"
    sed -i '' 's/Transport::new(&self.config)/Transport::new()/g' "$file"
done

# 修复lib.rs中的调用
sed -i '' 's/transport: Transport::new(&config)/transport: Transport::new()/g' crates/openlark-docs/src/bitable/lib.rs

echo "Transport constructors fixed"