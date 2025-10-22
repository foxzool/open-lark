#!/bin/bash

echo "Fixing remaining import issues with correct paths..."

# Fix incorrect use crate::crate:: patterns
find crates/open-lark-communication/src -name "*.rs" -type f | while read file; do
    echo "Fixing crate::crate:: in: $file"
    sed -i '' 's/use crate::crate::/use crate::/g' "$file"
    sed -i '' 's/crate::crate::/crate::/g' "$file"
done

find crates/open-lark-collaboration/src -name "*.rs" -type f | while read file; do
    echo "Fixing crate::crate:: in: $file"
    sed -i '' 's/use crate::crate::/use crate::/g' "$file"
    sed -i '' 's/crate::crate::/crate::/g' "$file"
done

# Fix remaining standalone core imports (not use crate::core::)
find crates/open-lark-communication/src -name "*.rs" -type f | while read file; do
    echo "Fixing standalone core imports in: $file"
    sed -i '' 's/^    core::{/    open_lark_core::core::{/g' "$file"
    sed -i '' 's/^use core::{/use open_lark_core::core::{/g' "$file"
done

find crates/open-lark-collaboration/src -name "*.rs" -type f | while read file; do
    echo "Fixing standalone core imports in: $file"
    sed -i '' 's/^    core::{/    open_lark_core::core::{/g' "$file"
    sed -i '' 's/^use core::{/use open_lark_core::core::{/g' "$file"
done

echo "Final import fixes completed!"