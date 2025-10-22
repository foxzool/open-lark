#!/bin/bash

# Fix import paths in communication crate
echo "Fixing import paths in open-lark-communication crate..."

find crates/open-lark-communication/src -name "*.rs" -type f | while read file; do
    echo "Processing: $file"
    # Replace crate::core:: with open_lark_core::core::
    sed -i '' 's/use crate::core::/use open_lark_core::core::/g' "$file"
    # Handle specific core trait imports
    sed -i '' 's/use crate::core::{/use open_lark_core::{/g' "$file"
done

# Fix import paths in collaboration crate
echo "Fixing import paths in open-lark-collaboration crate..."

find crates/open-lark-collaboration/src -name "*.rs" -type f | while read file; do
    echo "Processing: $file"
    # Replace crate::core:: with open_lark_core::core::
    sed -i '' 's/use crate::core::/use open_lark_core::core::/g' "$file"
    # Handle specific core trait imports
    sed -i '' 's/use crate::core::{/use open_lark_core::{/g' "$file"
done

echo "Import path fixes completed!"