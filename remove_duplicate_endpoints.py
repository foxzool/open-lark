#!/usr/bin/env python3
import os
import re
import subprocess

def find_files_with_duplicate_endpoints():
    """Find files with duplicate Endpoints imports"""
    files = []
    cmd = '''find /Users/zool/RustroverProjects/open-lark/src/service/cloud_docs/sheets -name "*.rs" -exec grep -l "endpoints::Endpoints" {} \\;'''
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    
    for file_path in result.stdout.strip().split('\n'):
        if file_path:
            # Count occurrences
            count_cmd = f'grep -c "endpoints::Endpoints" "{file_path}"'
            count_result = subprocess.run(count_cmd, shell=True, capture_output=True, text=True)
            if count_result.stdout.strip() != "1":
                files.append(file_path)
    
    return files

def remove_duplicate_endpoints(file_path):
    """Remove duplicate Endpoints imports"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Find the first occurrence of endpoints::Endpoints in core imports
        in_core_imports = False
        first_endpoints_found = False
        lines = content.split('\n')
        new_lines = []
        
        for line in lines:
            # Check if we're in the core imports section
            if 'core::{' in line:
                in_core_imports = True
            elif in_core_imports and '},' in line and 'core' not in line:
                in_core_imports = False
            
            # If this line contains endpoints::Endpoints
            if 'endpoints::Endpoints' in line:
                if in_core_imports and not first_endpoints_found:
                    # Keep the first one in core imports
                    first_endpoints_found = True
                    new_lines.append(line)
                elif not in_core_imports:
                    # Skip standalone imports
                    print(f"  Removing duplicate import: {line.strip()}")
                    continue
                else:
                    # Skip additional ones in core imports
                    print(f"  Removing duplicate core import: {line.strip()}")
                    continue
            else:
                new_lines.append(line)
        
        new_content = '\n'.join(new_lines)
        
        if new_content != content:
            with open(file_path, 'w') as f:
                f.write(new_content)
            
            print(f"  Fixed duplicate imports: {file_path}")
            return True
        else:
            print(f"  No changes needed: {file_path}")
            return False
    
    except Exception as e:
        print(f"  Error processing {file_path}: {e}")
        return False

def main():
    files = find_files_with_duplicate_endpoints()
    
    print(f"Found {len(files)} files with duplicate Endpoints imports")
    
    success_count = 0
    for file_path in files:
        if remove_duplicate_endpoints(file_path):
            success_count += 1
    
    print(f"\nSuccessfully fixed {success_count} files")

if __name__ == '__main__':
    main()