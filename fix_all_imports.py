#!/usr/bin/env python3

import os
import glob
import re

def fix_import_error(content):
    """修复导入错误"""
    
    # 修复模式：api_resp:{...endpoints::Endpoints,...}
    pattern = r'api_resp:\{([^}]*?)endpoints::Endpoints,([^}]*?)\}'
    
    def replace_func(match):
        before = match.group(1).strip()
        after = match.group(2).strip()
        
        # 清理前后内容
        parts = []
        if before:
            parts.append(before.rstrip(',').strip())
        if after:
            parts.append(after.lstrip(',').strip())
        
        # 过滤空字符串
        parts = [p for p in parts if p]
        
        if parts:
            return f'api_resp:{{{", ".join(parts)}}}'
        else:
            return 'api_resp::BaseResponse'
    
    content = re.sub(pattern, replace_func, content, flags=re.DOTALL)
    
    # 在修复api_resp块后，确保endpoints::Endpoints在正确位置
    # 查找use crate块并添加endpoints导入
    lines = content.split('\n')
    in_use_block = False
    use_block_start = -1
    use_block_end = -1
    has_endpoints = False
    
    for i, line in enumerate(lines):
        if line.strip().startswith('use crate::{'):
            in_use_block = True
            use_block_start = i
        elif in_use_block and line.strip() == '};':
            use_block_end = i
            break
        elif in_use_block and 'endpoints::Endpoints' in line:
            has_endpoints = True
    
    # 如果在use块中找不到endpoints导入，添加它
    if use_block_start != -1 and use_block_end != -1 and not has_endpoints:
        # 找到core块的位置
        core_start = -1
        core_end = -1
        for i in range(use_block_start, use_block_end):
            if 'core::{' in lines[i]:
                core_start = i
                # 找到对应的}
                brace_count = 1
                for j in range(i + 1, use_block_end):
                    if '{' in lines[j]:
                        brace_count += lines[j].count('{')
                    if '}' in lines[j]:
                        brace_count -= lines[j].count('}')
                        if brace_count == 0:
                            core_end = j
                            break
                break
        
        # 在core块后添加endpoints导入
        if core_start != -1 and core_end != -1:
            lines.insert(core_end + 1, '        endpoints::Endpoints,')
    
    return '\n'.join(lines)

def process_file(file_path):
    """处理单个文件"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # 检查是否包含导入错误
        if 'api_resp:' in content and 'endpoints::Endpoints' in content:
            original_content = content
            fixed_content = fix_import_error(content)
            
            if fixed_content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(fixed_content)
                print(f"✓ 修复: {file_path}")
                return True
            else:
                print(f"- 无需修复: {file_path}")
                return False
        else:
            print(f"- 跳过: {file_path}")
            return False
            
    except Exception as e:
        print(f"✗ 错误: {file_path}: {e}")
        return False

def main():
    """主函数"""
    print("批量修复所有导入错误...")
    
    # 获取所有sheets模块的Rust文件
    patterns = [
        'src/service/cloud_docs/sheets/v2/**/*.rs',
        'src/service/cloud_docs/sheets/v3/**/*.rs'
    ]
    
    files = []
    for pattern in patterns:
        files.extend(glob.glob(pattern, recursive=True))
    
    print(f"检查 {len(files)} 个文件")
    
    fixed_count = 0
    for file_path in files:
        if os.path.isfile(file_path):
            if process_file(file_path):
                fixed_count += 1
    
    print(f"修复完成! 共修复 {fixed_count} 个文件")

if __name__ == "__main__":
    main()