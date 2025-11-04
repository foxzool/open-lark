#!/usr/bin/env python3
"""
Script to fix missing 'mut' keywords in ApiRequest declarations.
"""

import os
import re

def fix_mut_issues():
    """Fix all missing 'mut' keywords in ApiRequest declarations"""
    hire_dir = "/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire"

    fixed_files = 0

    for root, dirs, files in os.walk(hire_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)

                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        content = f.read()

                    original_content = content

                    # Fix: let api_req = ApiRequest::default(); -> let mut api_req = ApiRequest::default();
                    content = re.sub(
                        r'let  api_req = ApiRequest::default\(\);',
                        r'let mut api_req = ApiRequest::default();',
                        content
                    )

                    # Fix: let api_req = ApiRequest::default(); -> let mut api_req = ApiRequest::default();
                    content = re.sub(
                        r'let api_req = ApiRequest::default\(\);',
                        r'let mut api_req = ApiRequest::default();',
                        content
                    )

                    # Fix api_req.body assignments that should use set_body method instead
                    # But since there's no set_body method, we need to keep it as is but ensure mut

                    if content != original_content:
                        with open(file_path, 'w', encoding='utf-8') as f:
                            f.write(content)
                        fixed_files += 1
                        print(f"Fixed mut issues in {os.path.relpath(file_path, hire_dir)}")

                except Exception as e:
                    print(f"Error processing {file_path}: {e}")

    return fixed_files

def main():
    """Main function"""
    print("Fixing missing 'mut' keywords...")
    fixed_files = fix_mut_issues()
    print(f"Fixed mut issues in {fixed_files} files")

if __name__ == "__main__":
    main()