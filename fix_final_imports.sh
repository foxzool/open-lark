#!/bin/bash

echo "Fixing remaining import path issues..."

# Fix crate::im:: and crate::contact:: imports in communication crate
find crates/open-lark-communication/src -name "*.rs" -type f | while read file; do
    # Fix crate::im:: to crate::im::
    sed -i '' 's/crate::crate::im::/crate::im::/g' "$file"
    # Fix crate::contact:: to crate::contact::
    sed -i '' 's/crate::crate::contact::/crate::contact::/g' "$file"
    # Fix any remaining crate::crate:: patterns
    sed -i '' 's/crate::crate::/crate::/g' "$file"
done

# Fix crate::cloud_docs:: imports in collaboration crate
find crates/open-lark-collaboration/src -name "*.rs" -type f | while read file; do
    sed -i '' 's/crate::crate::cloud_docs::/crate::cloud_docs::/g' "$file"
    sed -i '' 's/crate::crate::/crate::/g' "$file"
done

echo "Final import fixes completed!"