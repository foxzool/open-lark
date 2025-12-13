#!/usr/bin/env python3
"""
批量修复 openlark-docs crate 中的常见编译错误
"""

import os
import re
import subprocess
from pathlib import Path

def run_command(cmd):
    """运行命令并返回输出"""
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd="/Users/zool/RustroverProjects/open-lark")
    return result.stdout, result.stderr

def get_all_rs_files():
    """获取所有需要修复的 Rust 文件"""
    files = []
    for root, dirs, filenames in os.walk("crates/openlark-docs/src"):
        for filename in filenames:
            if filename.endswith(".rs"):
                files.append(os.path.join(root, filename))
    return files

def fix_serde_imports(file_path):
    """修复缺少的 serde 导入"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # 检查是否需要修复
    has_derive = "#[derive" in content and ("Serialize" in content or "Deserialize" in content)
    has_import = "use serde::" in content

    if has_derive and not has_import:
        # 找到最后一个 use 语句
        use_pattern = r'^\s*use\s+[^;]+;'
        uses = re.findall(use_pattern, content, re.MULTILINE)

        if uses:
            # 在最后一个 use 后添加
            last_use_idx = max(m.start() for m in re.finditer(use_pattern, content, re.MULTILINE))
            line_end = content.find('\n', last_use_idx) + 1

            new_import = "\n// 导入序列化支持\nuse serde::{Deserialize, Serialize};\n"
            content = content[:line_end] + new_import + content[line_end:]
        else:
            # 如果没有 use 语句，在文件开头添加
            lines = content.split('\n')
            insert_idx = 0
            for i, line in enumerate(lines):
                if line.strip() and not line.strip().startswith('//') and not line.strip().startswith("///"):
                    insert_idx = i
                    break

            lines.insert(insert_idx, "// 导入序列化支持")
            lines.insert(insert_idx + 1, "use serde::{Deserialize, Serialize};")
            lines.insert(insert_idx + 2, "")
            content = '\n'.join(lines)

        # 写回文件
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        return True

    return False

def fix_api_response_imports(file_path):
    """修复缺少的 API 响应相关导入"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # 检查是否需要添加 Response 导入
    if "Response<" in content and "use openlark_core::api::Response" not in content:
        # 查找现有的 openlark_core 导入
        if "use openlark_core::" in content:
            # 在现有导入中添加 Response
            content = re.sub(
                r'use openlark_core::\{([^}]*)\}',
                lambda m: f'use openlark_core::{{{m.group(1).rstrip(",")}, Response}}' if m.group(1).strip() else 'use openlark_core::{Response}',
                content
            )
        else:
            # 添加新的导入
            use_pattern = r'^\s*use\s+[^;]+;'
            uses = re.findall(use_pattern, content, re.MULTILINE)

            if uses:
                last_use_idx = max(m.start() for m in re.finditer(use_pattern, content, re.MULTILINE))
                line_end = content.find('\n', last_use_idx) + 1
                content = content[:line_end] + "\nuse openlark_core::api::Response;\n" + content[line_end:]
            else:
                # 在文件开头添加
                lines = content.split('\n')
                lines.insert(0, "use openlark_core::api::Response;")
                lines.insert(1, "")
                content = '\n'.join(lines)

        # 写回文件
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        return True

    return False

def main():
    print("开始批量修复 openlark-docs 编译错误...")

    # 获取所有 Rust 文件
    rs_files = get_all_rs_files()
    print(f"找到 {len(rs_files)} 个 Rust 文件")

    # 1. 修复 serde 导入
    print("\n1. 修复 serde 导入...")
    serde_fixed = 0
    for file_path in rs_files:
        if fix_serde_imports(file_path):
            print(f"  修复: {file_path}")
            serde_fixed += 1
    print(f"  修复了 {serde_fixed} 个文件的 serde 导入")

    # 2. 修复 Response 导入
    print("\n2. 修复 Response 导入...")
    response_fixed = 0
    for file_path in rs_files:
        if fix_api_response_imports(file_path):
            print(f"  修复: {file_path}")
            response_fixed += 1
    print(f"  修复了 {response_fixed} 个文件的 Response 导入")

    # 3. 运行编译检查
    print("\n3. 运行编译检查...")
    _, stderr = run_command("cargo check -p openlark-docs 2>&1")

    # 统计错误
    errors = stderr.count("error[")
    warnings = stderr.count("warning[")

    print(f"\n编译结果:")
    print(f"  错误: {errors}")
    print(f"  警告: {warnings}")

    # 显示最常见的错误类型
    error_types = {}
    for line in stderr.split('\n'):
        if "error[E" in line:
            error_code = re.search(r'error\[E(\d+)\]', line)
            if error_code:
                error_types[error_code.group(1)] = error_types.get(error_code.group(1), 0) + 1

    print("\n最常见的错误类型:")
    for error_code, count in sorted(error_types.items(), key=lambda x: x[1], reverse=True)[:10]:
        print(f"  E{error_code}: {count} 次")

if __name__ == "__main__":
    os.chdir("/Users/zool/RustroverProjects/open-lark")
    main()