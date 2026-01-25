#!/usr/bin/env python3
"""
自动生成显式导出列表工具
"""

import os
import re
from pathlib import Path
from typing import Set, Tuple


def extract_public_exports(file_path: Path) -> Set[str]:
    """提取 Rust 文件中的所有公共导出"""
    content = file_path.read_text(encoding='utf-8')
    
    exports = set()
    
    # 查找 pub fn, pub struct, pub enum, pub type, pub mod
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
    
    return exports


def scan_module_exports(module_dir: Path) -> Tuple[str, Set[str]]:
    """扫描模块目录并返回导出列表"""
    if not module_dir.exists():
        return None, set()
    
    exports = set()
    
    # 扫描目录中的所有 .rs 文件
    for rs_file in module_dir.rglob('*.rs'):
        # 跳过 mod.rs 本身
        if rs_file.name == 'mod.rs':
            continue
        
        file_exports = extract_public_exports(rs_file)
        exports.update(file_exports)
    
    # 如果有子模块，也扫描子模块的 mod.rs
    for sub_dir in module_dir.iterdir():
        if sub_dir.is_dir():
            mod_rs = sub_dir / 'mod.rs'
            if mod_rs.exists():
                sub_exports = extract_public_exports(mod_rs)
                exports.update(sub_exports)
    
    return module_dir.name, exports


def generate_export_list(module_name: str, exports: Set[str]) -> str:
    """生成显式导出代码"""
    if not exports:
        return f"// {module_name}: 没有找到导出\n"
    
    sorted_exports = sorted(exports)
    items = ',\n    '.join(sorted_exports)
    
    return f"// {module_name} 模块显式导出（自动生成）\npub use super::{module_name}::{{\n    {items},\n}};\n"


def process_module(module_path: Path, base_path: Path):
    """处理单个模块"""
    try:
        module_name, exports = scan_module_exports(module_path)
        if exports:
            print(f"\n📦 {module_name}: {len(exports)} 个导出")
            return generate_export_list(module_name, exports)
    except Exception as e:
        print(f"❌ 处理 {module_path} 失败: {e}")
        return None


def main():
    """主函数"""
    base_path = Path.cwd()
    src_path = base_path / 'crates/openlark-docs/src'
    
    print("🔍 扫描 openlark-docs 模块...")
    
    # 高优先级模块
    high_priority = [
        'base/bitable/v1/app/table/field',
        'base/bitable/v1/app/table/form',
    ]
    
    all_exports = {}
    
    for rel_path in high_priority:
        module_path = src_path / rel_path
        if module_path.exists():
            exports = process_module(module_path, base_path)
            if exports:
                all_exports[rel_path] = exports
    
    print(f"\n✅ 扫描完成，找到 {len(all_exports)} 个模块")


if __name__ == '__main__':
    main()
