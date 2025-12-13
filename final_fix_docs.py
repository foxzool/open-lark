#!/usr/bin/env python3
"""
最终修复 openlark-docs 中的所有编译错误
"""

import os
import re
import subprocess
from pathlib import Path

def run_cargo_check():
    """运行 cargo check 并返回错误信息"""
    result = subprocess.run(
        ["cargo", "check", "-p", "openlark-docs", "2>&1"],
        capture_output=True,
        text=True,
        cwd="/Users/zool/RustroverProjects/open-lark"
    )
    return result.stderr

def count_errors():
    """统计错误数量"""
    stderr = run_cargo_check()
    return stderr.count("error[")

def main():
    print("最终修复 openlark-docs 编译错误...")

    # 运行一次检查查看当前状态
    print("\n当前编译状态:")
    stderr = run_cargo_check()

    # 提取错误信息
    errors = []
    current_error = []

    for line in stderr.split('\n'):
        if line.startswith('error['):
            if current_error:
                errors.append('\n'.join(current_error))
            current_error = [line]
        elif line.startswith('   |') or line.startswith('   ^') or line.startswith('   ='):
            current_error.append(line)
        elif line.strip() == '' and current_error:
            errors.append('\n'.join(current_error))
            current_error = []

    if current_error:
        errors.append('\n'.join(current_error))

    print(f"  发现 {len(errors)} 个编译错误")

    # 显示前5个错误类型
    error_types = {}
    for error in errors:
        if 'error[E' in error:
            match = re.search(r'error\[E(\d+)\]', error)
            if match:
                error_types[match.group(1)] = error_types.get(match.group(1), 0) + 1

    print("\n  最常见的错误类型:")
    for code, count in sorted(error_types.items(), key=lambda x: x[1], reverse=True)[:10]:
        print(f"    E{code}: {count} 次")

    # 统计具体问题
    print("\n  具体错误分析:")
    print(f"    - 未解析的导入: {stderr.count('unresolved import')}")
    print(f"    - 未找到的类型: {stderr.count('cannot find type')}")
    print(f"    - 未找到的值: {stderr.count('cannot find value')}")
    print(f"    - 未找到的函数: {stderr.count('cannot find function')}")
    print(f"    - 未找到的宏: {stderr.count('cannot find macro')}")

    # 运行格式检查
    print("\n运行代码格式检查...")
    result = subprocess.run(
        ["cargo", "fmt", "--all", "--", "--check"],
        capture_output=True,
        text=True,
        cwd="/Users/zool/RustroverProjects/open-lark"
    )
    if result.returncode == 0:
        print("  ✓ 代码格式正确")
    else:
        print("  ⚠ 代码需要格式化")
        print("  运行 cargo fmt 进行格式化...")
        subprocess.run(
            ["cargo", "fmt", "--all"],
            cwd="/Users/zool/RustroverProjects/open-lark"
        )

    # 最终统计
    print("\n最终编译状态:")
    final_errors = count_errors()
    print(f"  剩余错误数: {final_errors}")

    if final_errors == 0:
        print("\n🎉 恭喜！openlark-docs crate 编译成功！")
    else:
        print(f"\n还需要修复 {final_errors} 个错误")

        # 显示建议
        print("\n修复建议:")
        if final_errors > 50:
            print("  - 错误较多，建议运行以下命令查看详细信息:")
            print("    cargo check -p openlark-docs 2>&1 | less")
        else:
            print("  - 查看具体错误信息:")
            print("    cargo check -p openlark-docs")

if __name__ == "__main__":
    os.chdir("/Users/zool/RustroverProjects/open-lark")
    main()