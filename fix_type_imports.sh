#!/bin/bash

echo "Fixing type imports in communication crate..."

# Fix imports in contact v3 modules
find crates/open-lark-communication/src/contact/v3 -name "*.rs" -type f | while read file; do
    echo "Processing: $file"
    # Add models import at the top if not already present
    if ! grep -q "use crate::contact::models::" "$file"; then
        sed -i '' '1i\
use crate::contact::models::*;
' "$file"
    fi

    # Fix any remaining crate::contact::models:: to just use the imported types
    sed -i '' 's/crate::contact::models:://g' "$file"
done

echo "Type imports fixed!"