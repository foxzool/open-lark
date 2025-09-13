#!/usr/bin/env python3

import os
import glob

def fix_import_error(file_path):
    """修复特定的导入错误"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # 查找并修复错误的导入结构
    lines = content.split('\n')
    in_use_block = False
    
    for i, line in enumerate(lines):
        if 'use crate::{' in line:
            in_use_block = True
        elif in_use_block and '}' in line and 'use' not in line:
            in_use_block = False
        elif in_use_block and 'api_resp:' in line and 'endpoints::Endpoints,' in lines[i+1]:
            # 找到错误的结构，需要修复
            # 获取api_resp块的内容
            j = i + 1
            api_resp_content = []
            endpoints_found = False
            
            while j < len(lines) and not lines[j].strip().endswith('},'):
                if 'endpoints::Endpoints,' in lines[j]:
                    endpoints_found = True
                    j += 1
                    continue
                api_resp_content.append(lines[j].strip())
                j += 1
            
            if endpoints_found:
                # 修复导入结构
                api_resp_line = lines[i].replace(',', '')
                if not api_resp_line.endswith('{'):
                    api_resp_line += '{'
                
                # 清理api_resp内容
                clean_content = []
                for content_line in api_resp_content:
                    if content_line and not content_line.startswith('endpoints'):
                        clean_content.append(content_line)
                
                # 重构
                if clean_content:
                    lines[i] = api_resp_line + ', '.join(clean_content) + '},'
                else:
                    lines[i] = api_resp_line.replace('{', '') + 'BaseResponse},'
                
                # 添加endpoints导入
                lines.insert(i+1, '        endpoints::Endpoints,')
                
                # 删除原来错误的行
                del lines[i+2:j+1]
                break
    
    return '\n'.join(lines)

def process_file(file_path):
    """处理单个文件"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            original_content = f.read()
        
        # 检查是否包含错误的导入
        if 'api_resp:' in original_content and 'endpoints::Endpoints,' in original_content:
            # 使用简单的字符串替换修复
            fixed_content = original_content
            
            # 修复模式1: api_resp:{...endpoints::Endpoints,...}
            import re
            
            pattern = r'api_resp:\{([^}]*?)\s*endpoints::Endpoints,([^}]*?)\},'
            def replace_func(match):
                before = match.group(1).strip()
                after = match.group(2).strip()
                
                # 清理前后内容
                parts = []
                if before:
                    parts.append(before.rstrip(','))
                if after:
                    parts.append(after.lstrip(','))
                
                if parts:
                    return f'api_resp:{{{", ".join(parts)}}},\n        endpoints::Endpoints,'
                else:
                    return 'api_resp::BaseResponse,\n        endpoints::Endpoints,'
            
            fixed_content = re.sub(pattern, replace_func, fixed_content, flags=re.DOTALL)
            
            # 修复其他格式
            fixed_content = re.sub(
                r'api_resp:\{\s*endpoints::Endpoints,([^}]*?)\}',
                r'api_resp:{\1},\n        endpoints::Endpoints',
                fixed_content,
                flags=re.DOTALL
            )
            
            if fixed_content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(fixed_content)
                print(f"✓ 修复: {file_path}")
            else:
                print(f"- 无需修复: {file_path}")
        else:
            print(f"- 跳过: {file_path}")
            
    except Exception as e:
        print(f"✗ 错误: {file_path}: {e}")

def main():
    """主函数"""
    print("批量修复导入错误...")
    
    # 获取所有v2数据操作模块文件
    files = glob.glob('src/service/cloud_docs/sheets/v2/data_operation/*.rs')
    
    print(f"处理 {len(files)} 个文件")
    
    for file_path in files:
        if os.path.isfile(file_path):
            process_file(file_path)
    
    print("修复完成!")

if __name__ == "__main__":
    main()