#!/usr/bin/env python3
import os
import re
import subprocess

def find_files_needing_endpoints_import():
    """Find v3 files that use Endpoints but don't have the import"""
    cmd = '''find /Users/zool/RustroverProjects/open-lark/src/service/cloud_docs/sheets/v3 -name "*.rs" -exec grep -l "Endpoints::" {} \\; | xargs grep -L "use.*endpoints::Endpoints"'''
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return result.stdout.strip().split('\n') if result.stdout.strip() else []

def add_endpoints_import(file_path):
    """Add endpoints::Endpoints import to a file's core imports"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Pattern to find core imports block
        core_import_pattern = r'(use crate::\{\s*core::\{[^}]*\})'
        
        match = re.search(core_import_pattern, content, re.DOTALL)
        if match:
            core_block = match.group(1)
            
            # Check if endpoints::Endpoints is already there
            if 'endpoints::Endpoints' in core_block:
                print(f"  Already has endpoints import: {file_path}")
                return False
            
            # Find where to insert endpoints::Endpoints
            # Look for constants::AccessTokenType and add endpoints before it
            if 'constants::AccessTokenType' in core_block:
                new_core_block = core_block.replace(
                    'constants::AccessTokenType',
                    'endpoints::Endpoints,\n        constants::AccessTokenType'
                )
            else:
                # Try to add after api_resp
                if 'api_resp::{' in core_block:
                    new_core_block = re.sub(
                        r'(api_resp::\{[^}]*\}),',
                        r'\1,\n        endpoints::Endpoints,',
                        core_block
                    )
                else:
                    print(f"  Could not find insertion point in: {file_path}")
                    return False
            
            content = content.replace(core_block, new_core_block)
            
            with open(file_path, 'w') as f:
                f.write(content)
            
            print(f"  Added endpoints import: {file_path}")
            return True
        else:
            print(f"  Could not find core imports block: {file_path}")
            return False
    
    except Exception as e:
        print(f"  Error processing {file_path}: {e}")
        return False

def main():
    files = find_files_needing_endpoints_import()
    
    print(f"Found {len(files)} files needing Endpoints import")
    
    success_count = 0
    for file_path in files:
        if file_path:  # Skip empty strings
            if add_endpoints_import(file_path):
                success_count += 1
    
    print(f"\nSuccessfully added imports to {success_count} files")

if __name__ == '__main__':
    main()