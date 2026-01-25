#!/usr/bin/env python3
"""
自动修复通配符导出工具

将 pub use module::*; 替换为显式导出列表
"""

import os
import re
from pathlib import Path
from typing import Set, Tuple


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


def fix_module_file(mod_file: Path):
    """修复单个模块的 mod.rs 文件"""
    content = mod_file.read_text(encoding='utf-8')
    original = content
    
    # 查找所有 pub use xyz::*; 模式
    glob_pattern = r'pub\s+use\s+(\w+)\s*::\*;'
    
    matches = list(re.finditer(glob_pattern, content))
    if not matches:
        return False
    
    print(f"\n📝 处理: {mod_file.relative_to(Path.cwd())}")
    
    for match in matches:
        module_name = match.group(1)
        
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
        indent = '    '
        items = ',\n'.join([f'{indent}{name}' for name in sorted_exports])
        
        old_line = match.group(0)
        new_lines = f"// {module_name} 模块显式导出\npub use {module_name}::{{\n{items},\n}};"
        
        content = content.replace(old_line, new_lines, 1)
        print(f"  ✓ {module_name}: {len(exports)} 个导出")
    
    if content != original:
        mod_file.write_text(content, encoding='utf-8')
        return True
    
    return False


def main():
    """主函数"""
    base_path = Path.cwd()
    
    print("🔧 修复通配符导出...")
    
    # 高优先级文件（有命名冲突的）
    high_priority_files = [
        'crates/openlark-docs/src/base/bitable/v1/app/mod.rs',
        'crates/openlark-docs/src/base/bitable/v1/app/table/mod.rs',
    ]
    
    count = 0
    for file_path in high_priority_files:
        full_path = base_path / file_path
        if full_path.exists():
            if fix_module_file(full_path):
                count += 1
    
    print(f"\n✅ 完成！修改了 {count} 个文件")


if __name__ == '__main__':
    main()
