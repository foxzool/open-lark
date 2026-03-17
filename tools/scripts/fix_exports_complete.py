#!/usr/bin/env python3
"""
自动修复通配符导出工具 - 完整版本

将 pub use module::*; 替换为显式导出列表
处理文件模块和目录模块
"""

import os
import re
import subprocess
from pathlib import Path
from typing import Set, Tuple, List, Optional


def extract_public_exports_from_file(rs_file: Path) -> Set[str]:
    """从单个 .rs 文件中提取公共导出"""
    exports = set()
    
    try:
        content = rs_file.read_text(encoding='utf-8')
        
        # 查找所有 pub 导出
        patterns = [
            r'pub\s+(?:async\s+)?fn\s+(\w+)',
            r'pub\s+struct\s+(\w+)',
            r'pub\s+enum\s+(\w+)',
            r'pub\s+type\s+(\w+)',
            r'pub\s+(?:use\s+)?(?:const\s+)?(?:static\s+)?(?:async\s+)?(?:trait\s+)?(?:impl\s+)?(?:mod\s+)?(?:fn\s+)?([A-Z][a-zA-Z0-9]*)',
        ]
        
        # 匹配结构体/枚举定义
        struct_pattern = r'pub\s+struct\s+([A-Z][a-zA-Z0-9_]+)'
        enum_pattern = r'pub\s+enum\s+([A-Z][a-zA-Z0-9_]+)'
        type_pattern = r'pub\s+type\s+([A-Z][a-zA-Z0-9_]+)'
        fn_pattern = r'pub\s+(?:async\s+)?fn\s+([a-z_][a-zA-Z0-9_]*)'
        
        for pattern in [struct_pattern, enum_pattern, type_pattern, fn_pattern]:
            matches = re.findall(pattern, content)
            exports.update(matches)
            
    except Exception as e:
        print(f"  ⚠️  读取 {rs_file} 失败: {e}")
    
    return exports


def extract_public_exports(module_path: Path) -> Set[str]:
    """提取模块中的所有公共导出"""
    exports = set()
    
    if module_path.is_dir():
        # 目录模块 - 扫描目录中的所有 .rs 文件
        for rs_file in module_path.rglob('*.rs'):
            if rs_file.name == 'mod.rs':
                continue
            exports.update(extract_public_exports_from_file(rs_file))
    elif module_path.with_suffix('.rs').exists():
        # 文件模块
        exports.update(extract_public_exports_from_file(module_path.with_suffix('.rs')))
    
    return exports


def find_module_path(mod_file: Path, module_name: str) -> Optional[Path]:
    """查找模块路径（支持目录和文件模块）"""
    parent = mod_file.parent
    
    # 检查是否是目录模块 (module_name/mod.rs)
    dir_module = parent / module_name
    if dir_module.is_dir():
        return dir_module
    
    # 检查是否是文件模块 (module_name.rs)
    file_module = parent / f"{module_name}.rs"
    if file_module.exists():
        return file_module.with_suffix('')  # 返回不带后缀的路径
    
    return None


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
        
        # 查找模块路径
        module_path = find_module_path(mod_file, module_name)
        if not module_path:
            print(f"  ⚠️  模块不存在: {module_name}")
            continue
        
        # 提取导出
        exports = extract_public_exports(module_path)
        
        if not exports:
            print(f"  ⚠️  {module_name}: 没有找到导出")
            # 仍然替换为空导出块
            sorted_exports = []
        else:
            sorted_exports = sorted(exports)
            print(f"  ✓ {module_name}: {len(exports)} 个导出")
        
        # 生成显式导出
        if sorted_exports:
            items = ',\n'.join([f'{indent}    {name}' for name in sorted_exports])
            new_lines = f"{indent}// {module_name} 模块显式导出\n{indent}pub use {module_name}::{{\n{items},\n{indent}}};"
        else:
            new_lines = f"{indent}// {module_name} 模块显式导出\n{indent}pub use {module_name}::{{}};"
        
        old_line = match.group(0)
        content = content.replace(old_line, new_lines, 1)
    
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
        try:
            if fix_module_file(mod_file):
                count += 1
        except Exception as e:
            print(f"  ❌ 处理失败: {mod_file} - {e}")
    
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
