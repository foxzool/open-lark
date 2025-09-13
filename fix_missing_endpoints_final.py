#!/usr/bin/env python3
import os
import subprocess

def find_files_missing_endpoints():
    """Find files that use Endpoints but don't have import"""
    cmd = '''find /Users/zool/RustroverProjects/open-lark/src/service/cloud_docs/sheets -name "*.rs" -exec grep -l "Endpoints::" {} \\; | xargs grep -L "endpoints::Endpoints"'''
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return [f for f in result.stdout.strip().split('\n') if f]

def add_endpoints_import_simple(file_path):
    """Add endpoints::Endpoints to core imports"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Simple replacement - add after constants::AccessTokenType
        if 'constants::AccessTokenType,' in content:
            content = content.replace(
                'constants::AccessTokenType,',
                'constants::AccessTokenType,\n        endpoints::Endpoints,'
            )
        elif 'constants::AccessTokenType' in content:
            content = content.replace(
                'constants::AccessTokenType',
                'constants::AccessTokenType,\n        endpoints::Endpoints'
            )
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        print(f"  Added endpoints import: {file_path}")
        return True
        
    except Exception as e:
        print(f"  Error: {e}")
        return False

def main():
    files = find_files_missing_endpoints()
    print(f"Found {len(files)} files missing Endpoints imports")
    
    for file_path in files:
        add_endpoints_import_simple(file_path)

if __name__ == '__main__':
    main()