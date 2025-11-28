#!/usr/bin/env python3
"""
批量修复security模块中的APIError引用的脚本
"""

import os
import re
from pathlib import Path

def fix_file(file_path):
    """修复单个文件中的APIError引用"""
    print(f"正在修复: {file_path}")

    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # 检查是否需要修复
    if 'SecurityError::APIError' not in content:
        print(f"  跳过: 不包含APIError引用")
        return True

    # 添加导入（如果还没有）
    if 'use openlark_core::error::{api_error};' not in content:
        # 找到第一个use语句后添加
        use_pattern = r'(use [^;]+;\n)'
        match = re.search(use_pattern, content)
        if match:
            content = content.replace(match.group(0), match.group(0) + 'use openlark_core::error::{api_error};\n')
        else:
            # 如果没有use语句，在第一个注释后添加
            comment_pattern = r'(^.*!\n\n)'
            match = re.search(comment_pattern, content, re.MULTILINE)
            if match:
                content = content.replace(match.group(0), match.group(0) + 'use openlark_core::error::{api_error};\n\n')
            else:
                # 在文件开头添加
                content = 'use openlark_core::error::{api_error};\n\n' + content

    # 替换API响应错误
    api_error_pattern = r'crate::SecurityError::APIError\s*\{\s*code:\s*api_response\.code,\s*message:\s*api_response\.msg,\s*\}'
    content = re.sub(
        api_error_pattern,
        'api_error(\n                        api_response.code as u16,\n                        "<ENDPOINT>",\n                        &api_response.msg,\n                        None,\n                    )',
        content
    )

    # 替换HTTP错误
    http_error_pattern = r'crate::SecurityError::APIError\s*\{\s*code:\s*response\.status\(\)\.as_u16\(\)\s+as i32,\s*message:\s*format!\(\s*"HTTP {}: \{\}",\s*response\.status\(\),\s*response\.text\(\)\.await\.unwrap_or_default\(\)\s*\),\s*\}'
    content = re.sub(
        http_error_pattern,
        'api_error(\n                    response.status().as_u16(),\n                    "<ENDPOINT>",\n                    &format!(\n                        "HTTP: {}",\n                        response.status()\n                    ),\n                    None,\n                )',
        content
    )

    # 手动处理特殊case（如果有的话）
    # 这里可以添加更多的模式匹配

    # 根据文件路径设置正确的endpoint
    endpoint_map = {
        'access_records': '/acs/v1/access_records',
        'devices': '/acs/v1/devices',
        'users': '/acs/v1/users',
        'user_faces': '/acs/v1/user_faces',
        'rule_external': '/acs/v1/rule_external',
        'visitors': '/acs/v1/visitors',
        'openapi_logs': '/security_and_compliance/v1/openapi_logs',
        'device_apply_records': '/security_and_compliance/v2/device_apply_records',
        'device_records': '/security_and_compliance/v2/device_records',
    }

    for path_key, endpoint in endpoint_map.items():
        if path_key in file_path:
            content = content.replace('<ENDPOINT>', endpoint)
            break

    # 写回文件
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)

    print(f"  完成: {file_path}")
    return True

def main():
    """主函数"""
    security_dir = Path('/Users/zool/RustroverProjects/open-lark/crates/openlark-security/src')

    # 需要修复的文件列表
    files_to_fix = [
        'acs/v1/access_records/mod.rs',
        'acs/v1/devices/mod.rs',
        'acs/v1/users/mod.rs',
        'acs/v1/user_faces/mod.rs',
        'acs/v1/rule_external/mod.rs',
        'acs/v1/visitors/mod.rs',
        'security_and_compliance/v1/openapi_logs/mod.rs',
        'security_and_compliance/v2/device_apply_records/mod.rs',
        'security_and_compliance/v2/device_records/mod.rs',
    ]

    success_count = 0
    total_count = len(files_to_fix)

    for file_path in files_to_fix:
        full_path = security_dir / file_path
        if full_path.exists():
            try:
                if fix_file(full_path):
                    success_count += 1
            except Exception as e:
                print(f"错误修复 {file_path}: {e}")
        else:
            print(f"文件不存在: {full_path}")

    print(f"\n修复完成: {success_count}/{total_count} 个文件")

if __name__ == '__main__':
    main()