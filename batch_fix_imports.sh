#!/bin/bash

# 获取所有有导入错误的文件
files=(
    "src/service/cloud_docs/sheets/v2/sheet_row_col/add_dimension_range.rs"
    "src/service/cloud_docs/sheets/v2/sheet_row_col/delete_dimension_range.rs"
    "src/service/cloud_docs/sheets/v2/sheet_row_col/insert_dimension_range.rs"
    "src/service/cloud_docs/sheets/v2/sheet_row_col/update_dimension_range.rs"
    "src/service/cloud_docs/sheets/v2/spreadsheet_sheet/operate_sheets.rs"
)

# 批量修复v2文件
for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "修复 $file"
        # 使用sed修复导入错误
        sed -i '' 's/api_resp:{\([^}]*\)endpoints::Endpoints,\([^}]*\)}/api_resp:{\1\2}, endpoints::Endpoints,/g' "$file"
        sed -i '' 's/api_resp:{\s*endpoints::Endpoints,\([^}]*\)}/api_resp:{\1}, endpoints::Endpoints,/g' "$file"
        # 修复空的api_resp块
        sed -i '' 's/api_resp:{\s*}/api_resp::BaseResponse/g' "$file"
        sed -i '' 's/api_resp::BaseResponse, endpoints::Endpoints,/api_resp::BaseResponse, endpoints::Endpoints,/g' "$file"
    fi
done

echo "批量修复完成"