#!/usr/bin/env python3

import os

# 需要修复的文件列表
files_to_fix = [
    "src/service/cloud_docs/sheets/v2/sheet_row_col/delete_dimension_range.rs",
    "src/service/cloud_docs/sheets/v2/sheet_row_col/insert_dimension_range.rs", 
    "src/service/cloud_docs/sheets/v2/sheet_row_col/update_dimension_range.rs",
    "src/service/cloud_docs/sheets/v2/spreadsheet_sheet/operate_sheets.rs",
    "src/service/cloud_docs/sheets/v3/condition_format/create.rs",
    "src/service/cloud_docs/sheets/v3/condition_format/delete.rs",
    "src/service/cloud_docs/sheets/v3/condition_format/get.rs", 
    "src/service/cloud_docs/sheets/v3/condition_format/update.rs",
    "src/service/cloud_docs/sheets/v3/data_operation/batch_set_cell_style.rs",
    "src/service/cloud_docs/sheets/v3/data_operation/find_cells.rs",
    "src/service/cloud_docs/sheets/v3/data_operation/merge_cells.rs",
    "src/service/cloud_docs/sheets/v3/data_operation/replace_cells.rs",
    "src/service/cloud_docs/sheets/v3/data_operation/set_cell_style.rs",
    "src/service/cloud_docs/sheets/v3/data_operation/split_cells.rs",
    "src/service/cloud_docs/sheets/v3/data_validation/create.rs",
    "src/service/cloud_docs/sheets/v3/data_validation/delete.rs",
    "src/service/cloud_docs/sheets/v3/data_validation/query.rs",
    "src/service/cloud_docs/sheets/v3/data_validation/update.rs",
    "src/service/cloud_docs/sheets/v3/float_image/create.rs",
    "src/service/cloud_docs/sheets/v3/float_image/delete.rs",
    "src/service/cloud_docs/sheets/v3/float_image/get.rs",
    "src/service/cloud_docs/sheets/v3/float_image/patch.rs",
    "src/service/cloud_docs/sheets/v3/float_image/query.rs"
]

def fix_import_in_file(file_path):
    """修复单个文件的导入错误"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # 查找并修复错误的导入模式
        # 模式1: api_resp::{...endpoints::Endpoints,...}
        if 'api_resp:' in content and 'endpoints::Endpoints,' in content:
            lines = content.split('\n')
            
            for i, line in enumerate(lines):
                if 'api_resp:' in line and 'endpoints::Endpoints,' in lines[i+1]:
                    # 找到错误的导入结构，修复它
                    api_resp_line = line
                    endpoints_line = lines[i+1]
                    
                    # 从api_resp行中提取其他导入
                    if api_resp_line.strip().endswith('{'):
                        # api_resp:{
                        # endpoints::Endpoints,
                        # ...
                        # }
                        j = i + 2
                        imports = []
                        while j < len(lines) and not lines[j].strip().startswith('}'):
                            import_line = lines[j].strip()
                            if import_line and not import_line.startswith('endpoints'):
                                imports.append(import_line)
                            j += 1
                        
                        # 重构导入
                        if imports:
                            lines[i] = api_resp_line.replace('{', f'{{{", ".join(imports)}}},')
                        else:
                            lines[i] = line.replace('api_resp:', 'api_resp::BaseResponse,')
                        
                        lines[i+1] = '        endpoints::Endpoints,'
                        
                        # 删除多余的行
                        del lines[i+2:j+1]
                        break
            
            content = '\n'.join(lines)
        
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        
        print(f"✓ 修复: {file_path}")
        return True
        
    except Exception as e:
        print(f"✗ 错误: {file_path}: {e}")
        return False

def main():
    """主函数"""
    print("修复特定文件的导入错误...")
    
    fixed_count = 0
    
    for file_path in files_to_fix:
        if os.path.exists(file_path):
            if fix_import_in_file(file_path):
                fixed_count += 1
        else:
            print(f"- 文件不存在: {file_path}")
    
    print(f"修复完成! 共修复 {fixed_count} 个文件")

if __name__ == "__main__":
    main()