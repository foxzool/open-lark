#!/usr/bin/env python3
"""
自动修复通配符导出工具 - 批量版本

将 pub use module::*; 替换为显式导出列表
"""

import os
import re
import subprocess
from pathlib import Path
from typing import Set, Tuple, List


def extract_public_exports(module_dir: Path) -> Set[str]:
    """提取模块中的所有公共导出"""
    exports = set()
    
    # 扫描目录中的所有 .rs 文件
    for rs_file in module_dir.rglob('*.rs'):
        if rs_file.name == 'mod.rs':
            continue
        
        try:
            content = rs_file.read_text(encoding='utf-8')
            
            # 查找所有 pub 导出
            patterns = [
                r'pub\s+(?:async\s+)?fn\s+(\w+)',
                r'pub\s+struct\s+(\w+)',
                r'pub\s+enum\s+(\w+)',
                r'pub\s+type\s+(\w+)',
                r'pub\s+mod\s+(\w+)',
            ]
            
            for pattern in patterns:
                matches = re.findall(pattern, content)
                exports.update(matches)
        except Exception as e:
            print(f"  ⚠️  读取 {rs_file} 失败: {e}")
    
    return exports


def fix_module_file(mod_file: Path) -> bool:
    """修复单个模块的 mod.rs 文件"""
    content = mod_file.read_text(encoding='utf-8')
    original = content
    
    # 查找所有 pub use xyz::*; 模式 (排除注释行)
    glob_pattern = r'^(\s*)pub\s+use\s+(\w+)\s*::\*;'
    
    matches = list(re.finditer(glob_pattern, content, re.MULTILINE))
    if not matches:
        return False
    
    print(f"\n📝 处理: {mod_file.relative_to(Path.cwd())}")
    
    for match in matches:
        indent = match.group(1)
        module_name = match.group(2)
        
        # 查找子模块路径
        module_dir = mod_file.parent / module_name
        if not module_dir.exists():
            print(f"  ⚠️  子模块目录不存在: {module_dir}")
            continue
        
        # 提取导出
        exports = extract_public_exports(module_dir)
        
        if not exports:
            print(f"  ⚠️  {module_name}: 没有找到导出")
            continue
        
        # 生成显式导出
        sorted_exports = sorted(exports)
        items = ',\n'.join([f'{indent}    {name}' for name in sorted_exports])
        
        old_line = match.group(0)
        new_lines = f"{indent}// {module_name} 模块显式导出\n{indent}pub use {module_name}::{{\n{items},\n{indent}}};"
        
        content = content.replace(old_line, new_lines, 1)
        print(f"  ✓ {module_name}: {len(exports)} 个导出")
    
    if content != original:
        mod_file.write_text(content, encoding='utf-8')
        return True
    
    return False


def find_mod_files_with_glob_exports(base_path: Path) -> List[Path]:
    """查找所有包含通配符导出的 mod.rs 文件"""
    result = []
    
    # 使用 grep 查找
    try:
        output = subprocess.run(
            ['grep', '-rl', '--include=mod.rs', 'pub use .*::\*', 'crates/'],
            capture_output=True,
            text=True,
            cwd=base_path
        )
        
        for line in output.stdout.strip().split('\n'):
            if line:
                result.append(base_path / line)
    except Exception as e:
        print(f"⚠️  grep 查找失败: {e}")
    
    return result


def main():
    """主函数"""
    base_path = Path.cwd()
    
    print("🔧 修复通配符导出...")
    print("=" * 60)
    
    # 查找所有包含通配符导出的文件
    mod_files = find_mod_files_with_glob_exports(base_path)
    print(f"\n📁 找到 {len(mod_files)} 个包含通配符导出的 mod.rs 文件")
    
    count = 0
    for mod_file in mod_files:
        if fix_module_file(mod_file):
            count += 1
    
    print("\n" + "=" * 60)
    print(f"✅ 完成！修改了 {count} 个文件")
    
    # 统计剩余的通配符导出
    print("\n📊 验证剩余通配符导出...")
    result = subprocess.run(
        ['grep', '-r', '--include=mod.rs', 'pub use .*::\*', 'crates/'],
        capture_output=True,
        text=True,
        cwd=base_path
    )
    
    remaining = len(result.stdout.strip().split('\n')) if result.stdout.strip() else 0
    print(f"   剩余通配符导出: {remaining} 个")


if __name__ == '__main__':
    main()
