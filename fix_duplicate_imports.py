#!/usr/bin/env python3
"""
修复重复导入的问题
"""

import os
import re
from pathlib import Path

def fix_duplicate_response_imports():
    """修复重复的 Response 导入"""
    rs_files = []
    for root, dirs, filenames in os.walk("crates/openlark-docs/src"):
        for filename in filenames:
            if filename.endswith(".rs"):
                rs_files.append(os.path.join(root, filename))

    fixed_files = []

    for file_path in rs_files:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # 修复重复的 Response 导入
        # 查找模式: api::{..., Response, Response}
        pattern = r'use openlark_core::\s*\n\s*api::\{([^}]*)\}'

        def fix_response_duplicate(match):
            imports = match.group(1)
            # 移除重复的 Response
            if ', Response, Response' in imports:
                imports = imports.replace(', Response, Response', '')
            elif 'Response, Response,' in imports:
                imports = imports.replace('Response, Response,', 'Response,')
            elif imports.strip() == 'Response, Response':
                imports = imports.replace('Response, Response', 'Response')

            # 清理多余的逗号
            imports = imports.replace(', ,', ',')
            imports = re.sub(r',+$', '', imports)
            imports = re.sub(r'^,\s*', '', imports)

            return f'use openlark_core::\n    api::{{{imports}}}'

        # 应用修复
        new_content = re.sub(pattern, fix_response_duplicate, content, flags=re.MULTILINE)

        # 另一种模式: api::{..., Response, Response}
        new_content = re.sub(
            r'api::\{([^}]*)Response,\s*Response,?([^}]*)\}',
            lambda m: f'api::{{{m.group(1)}Response{m.group(2)}}}',
            new_content
        )

        # 修复错误的 Response 导入位置
        new_content = new_content.replace(
            'use openlark_core::{api::ApiRequest, api::Response, error::LarkAPIError, http::Transport, Response}',
            'use openlark_core::{api::{ApiRequest, Response}, error::LarkAPIError, http::Transport}'
        )

        if content != new_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            fixed_files.append(file_path)

    return fixed_files

if __name__ == "__main__":
    os.chdir("/Users/zool/RustroverProjects/open-lark")
    fixed = fix_duplicate_response_imports()
    print(f"修复了 {len(fixed)} 个文件的重复导入问题")
    for file in fixed[:5]:  # 只显示前5个
        print(f"  - {file}")
    if len(fixed) > 5:
        print(f"  ... 还有 {len(fixed) - 5} 个文件")