#!/bin/bash

# Comprehensive fix script for communication module imports

echo "Fixing communication module imports..."

# Fix all crate::service imports
find /Users/zool/RustroverProjects/open-lark/crates/open-lark-communication/src -name "*.rs" -type f | while read file; do
  # Replace crate::service:: with appropriate paths
  sed -i '' 's/crate::service::im::/crate::im::/g' "$file"
  sed -i '' 's/crate::service::contact::/crate::contact::/g' "$file"
  sed -i '' 's/open_lark_core::service::/open_lark_core::/g' "$file"

  # Replace any remaining service:: references
  sed -i '' 's/service::im::models::/im::models::/g' "$file"
  sed -i '' 's/service::contact::models::/contact::models::/g' "$file"
done

# Fix contact module imports specifically
find /Users/zool/RustroverProjects/open-lark/crates/open-lark-communication/src/contact -name "*.rs" -type f | while read file; do
  # Fix open_lark_core::contact:: to contact::
  sed -i '' 's/open_lark_core::contact::/contact::/g' "$file"
done

echo "Import fixes completed"