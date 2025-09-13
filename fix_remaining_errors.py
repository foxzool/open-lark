#!/usr/bin/env python3

import os
import re
import glob

def fix_import_errors(file_path):
    """修复导入错误"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # 修复错误的导入结构：api_resp:{..., endpoints::Endpoints,}
    content = re.sub(
        r'api_resp:\{([^}]*),\s*endpoints::Endpoints,([^}]*)\}',
        r'api_resp:{\1\2}, endpoints::Endpoints',
        content
    )
    
    # 修复另一种错误格式
    content = re.sub(
        r'api_resp:\{([^}]*)\s*endpoints::Endpoints,([^}]*)\}',
        r'api_resp:{\1\2}, endpoints::Endpoints',
        content
    )
    
    return content

def fix_constant_names(file_path):
    """修复错误的常量名称"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # 修复v2常量名称
    content = re.sub(
        r'Endpoints::SHEETS_V\.replace\([^,]+,\s*&2_([A-Z_]+)\)',
        r'Endpoints::SHEETS_V2_\1',
        content
    )
    
    # 修复v3常量名称
    content = re.sub(
        r'Endpoints::SHEETS_V\.replace\([^,]+,\s*&3_([A-Z_]+)\)',
        r'Endpoints::SHEETS_V3_\1',
        content
    )
    
    # 修复其他格式的错误
    patterns = [
        (r'&2_SPREADSHEET_VALUES_APPEND', 'Endpoints::SHEETS_V2_SPREADSHEET_VALUES_APPEND'),
        (r'&3_SPREADSHEET_VALUES_APPEND', 'Endpoints::SHEETS_V3_SPREADSHEET_VALUES_APPEND'),
        (r'&2_SPREADSHEET_VALUES', 'Endpoints::SHEETS_V2_SPREADSHEET_VALUES'),
        (r'&3_SPREADSHEET_VALUES', 'Endpoints::SHEETS_V3_SPREADSHEET_VALUES'),
        (r'&2_SPREADSHEET_MERGE_CELLS', 'Endpoints::SHEETS_V2_SPREADSHEET_MERGE_CELLS'),
        (r'&3_SPREADSHEET_MERGE_CELLS', 'Endpoints::SHEETS_V3_SPREADSHEET_MERGE_CELLS'),
        (r'&2_SPREADSHEET_UNMERGE_CELLS', 'Endpoints::SHEETS_V2_SPREADSHEET_UNMERGE_CELLS'),
        (r'&3_SPREADSHEET_UNMERGE_CELLS', 'Endpoints::SHEETS_V3_SPREADSHEET_UNMERGE_CELLS'),
        (r'&2_SPREADSHEET_STYLES_BATCH_UPDATE', 'Endpoints::SHEETS_V2_SPREADSHEET_STYLES_BATCH_UPDATE'),
        (r'&3_SPREADSHEET_STYLES_BATCH_UPDATE', 'Endpoints::SHEETS_V3_SPREADSHEET_STYLES_BATCH_UPDATE'),
        (r'&2_SPREADSHEET_STYLES_UPDATE', 'Endpoints::SHEETS_V2_SPREADSHEET_STYLES_UPDATE'),
        (r'&3_SPREADSHEET_STYLES_UPDATE', 'Endpoints::SHEETS_V3_SPREADSHEET_STYLES_UPDATE'),
        (r'&2_SPREADSHEET_FIND', 'Endpoints::SHEETS_V2_SPREADSHEET_FIND'),
        (r'&3_SPREADSHEET_FIND', 'Endpoints::SHEETS_V3_SPREADSHEET_FIND'),
        (r'&2_SPREADSHEET_REPLACE', 'Endpoints::SHEETS_V2_SPREADSHEET_REPLACE'),
        (r'&3_SPREADSHEET_REPLACE', 'Endpoints::SHEETS_V3_SPREADSHEET_REPLACE'),
        (r'&3_SPREADSHEET_CONDITION_FORMAT', 'Endpoints::SHEETS_V3_SPREADSHEET_CONDITION_FORMAT'),
        (r'&3_SPREADSHEET_DATA_VALIDATION', 'Endpoints::SHEETS_V3_SPREADSHEET_DATA_VALIDATION'),
        (r'&3_SPREADSHEET_FILTER', 'Endpoints::SHEETS_V3_SPREADSHEET_FILTER'),
        (r'&3_SPREADSHEET_FLOAT_IMAGE', 'Endpoints::SHEETS_V3_SPREADSHEET_FLOAT_IMAGE'),
        (r'&3_SPREADSHEET_SHEET', 'Endpoints::SHEETS_V3_SPREADSHEET_SHEET'),
        (r'&3_SPREADSHEET_PROTECT_RANGE', 'Endpoints::SHEETS_V3_SPREADSHEET_PROTECT_RANGE'),
        (r'&3_SPREADSHEET_GET', 'Endpoints::SHEETS_V3_SPREADSHEET_GET'),
        (r'&2_SPREADSHEET_GET', 'Endpoints::SHEETS_V2_SPREADSHEET_GET'),
    ]
    
    for pattern, replacement in patterns:
        content = re.sub(pattern, replacement, content)
    
    return content

def process_file(file_path):
    """处理单个文件"""
    try:
        print(f"处理文件: {file_path}")
        
        # 修复导入错误
        content = fix_import_errors(file_path)
        
        # 修复常量名称
        content = fix_constant_names(file_path)
        
        # 写回文件
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
            
        print(f"✓ 完成: {file_path}")
        
    except Exception as e:
        print(f"✗ 错误处理 {file_path}: {e}")

def main():
    """主函数"""
    print("修复剩余的导入和语法错误...")
    
    # 获取所有sheets模块的Rust文件
    patterns = [
        'src/service/cloud_docs/sheets/v2/**/*.rs',
        'src/service/cloud_docs/sheets/v3/**/*.rs'
    ]
    
    files = []
    for pattern in patterns:
        files.extend(glob.glob(pattern, recursive=True))
    
    print(f"找到 {len(files)} 个文件")
    
    for file_path in files:
        if os.path.isfile(file_path):
            process_file(file_path)
    
    print("修复完成!")

if __name__ == "__main__":
    main()