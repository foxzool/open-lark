#!/usr/bin/env python3

import os
import re
import glob

def fix_import_structure(content):
    """修复导入结构中endpoints::Endpoints的位置"""
    
    # 匹配并修复 api_resp 块中错误包含 endpoints::Endpoints 的情况
    # 寻找模式: api_resp:{...endpoints::Endpoints,...}
    pattern = r'api_resp:\s*\{([^}]*?)\s*endpoints::Endpoints,([^}]*?)\},'
    
    def replace_func(match):
        before = match.group(1).strip()
        after = match.group(2).strip()
        
        # 清理前后的内容，移除多余的逗号
        if before.endswith(','):
            before = before[:-1]
        if after.startswith(','):
            after = after[1:]
        
        # 重构导入
        if before and after:
            return f'api_resp:{{{before}, {after}}}, endpoints::Endpoints,'
        elif before:
            return f'api_resp:{{{before}}}, endpoints::Endpoints,'
        elif after:
            return f'api_resp:{{{after}}}, endpoints::Endpoints,'
        else:
            return 'api_resp::BaseResponse, endpoints::Endpoints,'
    
    content = re.sub(pattern, replace_func, content, flags=re.DOTALL)
    
    # 处理其他可能的错误格式
    content = re.sub(
        r'api_resp:\s*\{\s*endpoints::Endpoints,([^}]*?)\}',
        r'api_resp:{\1}, endpoints::Endpoints',
        content,
        flags=re.DOTALL
    )
    
    # 修复可能的重复导入
    content = re.sub(
        r'endpoints::Endpoints,\s*endpoints::Endpoints,',
        'endpoints::Endpoints,',
        content
    )
    
    return content

def process_file(file_path):
    """处理单个文件"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        content = fix_import_structure(content)
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✓ 修复: {file_path}")
        else:
            print(f"- 无需修复: {file_path}")
            
    except Exception as e:
        print(f"✗ 错误处理 {file_path}: {e}")

def main():
    """主函数"""
    print("修复最终的导入结构错误...")
    
    # 获取所有sheets模块的Rust文件
    patterns = [
        'src/service/cloud_docs/sheets/v2/**/*.rs',
        'src/service/cloud_docs/sheets/v3/**/*.rs'
    ]
    
    files = []
    for pattern in patterns:
        files.extend(glob.glob(pattern, recursive=True))
    
    print(f"检查 {len(files)} 个文件")
    
    for file_path in files:
        if os.path.isfile(file_path):
            process_file(file_path)
    
    print("修复完成!")

if __name__ == "__main__":
    main()