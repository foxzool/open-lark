#!/usr/bin/env python3
import os
import re
import subprocess

def find_files_with_broken_imports():
    """Find files with broken import structure"""
    cmd = '''find /Users/zool/RustroverProjects/open-lark/src/service/cloud_docs/sheets -name "*.rs" -exec grep -l "ResponseFormat," {} \\;'''
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return result.stdout.strip().split('\n') if result.stdout.strip() else []

def fix_import_structure(file_path):
    """Fix broken import structures in files"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Fix broken api_resp import line
        broken_pattern = r'api_resp::\{ApiResponseTrait, BaseResponse, ResponseFormat,\s*\n\s*\},\s*constants::AccessTokenType,'
        if re.search(broken_pattern, content, re.MULTILINE):
            content = re.sub(
                broken_pattern,
                'api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},\n        constants::AccessTokenType,\n        endpoints::Endpoints,',
                content,
                flags=re.MULTILINE
            )
            
            with open(file_path, 'w') as f:
                f.write(content)
            
            print(f"  Fixed broken import structure: {file_path}")
            return True
        
        # Alternative pattern
        broken_pattern2 = r'api_resp::\{ApiResponseTrait, BaseResponse, ResponseFormat,\s*\n\s*\n\s*\},'
        if re.search(broken_pattern2, content, re.MULTILINE):
            content = re.sub(
                broken_pattern2,
                'api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},',
                content,
                flags=re.MULTILINE
            )
            
            # Check if endpoints::Endpoints is missing
            if 'endpoints::Endpoints' not in content:
                content = re.sub(
                    r'(constants::AccessTokenType,)',
                    r'endpoints::Endpoints,\n        \1',
                    content
                )
            
            with open(file_path, 'w') as f:
                f.write(content)
            
            print(f"  Fixed broken import structure (v2): {file_path}")
            return True
        
        print(f"  No broken import pattern found: {file_path}")
        return False
    
    except Exception as e:
        print(f"  Error processing {file_path}: {e}")
        return False

def main():
    files = find_files_with_broken_imports()
    
    print(f"Found {len(files)} files with potential import issues")
    
    success_count = 0
    for file_path in files:
        if file_path:  # Skip empty strings
            if fix_import_structure(file_path):
                success_count += 1
    
    print(f"\nSuccessfully fixed import structures in {success_count} files")

if __name__ == '__main__':
    main()