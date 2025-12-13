#!/bin/bash

# 修复 Rust 文件中 inner attribute 位置的脚本
# 将 #![...] 移动到文档注释之前

# 需要修复的文件列表
files=(
    "crates/openlark-docs/src/ccm/sheets/v2/single_write.rs"
    "crates/openlark-docs/src/ccm/sheets/v2/batch_read.rs"
    "crates/openlark-docs/src/ccm/sheets/v2/sheet_cells.rs"
    "crates/openlark-docs/src/ccm/sheets/v2/single_read.rs"
    "crates/openlark-docs/src/ccm/sheets/v2/range.rs"
    "crates/openlark-docs/src/ccm/sheets/v2/style.rs"
    "crates/openlark-docs/src/ccm/sheets/v2/spreadsheet.rs"
    "crates/openlark-docs/src/ccm/sheets/v2/batch_write.rs"
    "crates/openlark-docs/src/ccm/sheets/v2/image_write.rs"
    "crates/openlark-docs/src/ccm/sheets/v2/worksheet.rs"
    "crates/openlark-docs/src/ccm/sheets/v3/mod.rs"
    "crates/openlark-docs/src/ccm/wiki/v1/mod.rs"
    "crates/openlark-docs/src/ccm/wiki/v2/space_node/mod.rs"
    "crates/openlark-docs/src/ccm/wiki/v2/space_setting/mod.rs"
    "crates/openlark-docs/src/ccm/wiki/v2/mod.rs"
    "crates/openlark-docs/src/ccm/wiki/v2/task/mod.rs"
    "crates/openlark-docs/src/ccm/wiki/v2/space_member/mod.rs"
    "crates/openlark-docs/src/ccm/wiki/v2/space/mod.rs"
)

for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "Fixing $file..."

        # 使用 sed 脚本来修复文件
        # 1. 提取所有 inner attributes
        # 2. 删除它们
        # 3. 在文件开头插入它们

        # 创建临时文件
        tmp_file=$(mktemp)

        # 提取 inner attributes
        grep '^#!\[' "$file" > "$tmp_file.attrs" 2>/dev/null || true

        # 如果找到了 inner attributes
        if [ -s "$tmp_file.attrs" ]; then
            # 删除所有 inner attributes 行
            grep -v '^#!\[' "$file" > "$tmp_file.noattrs"

            # 找到第一个非文档注释、非空行
            awk '
            BEGIN { printed_attrs = 0 }
            /^[[:space:]]*$/ { print; next }
            /^[[:space:]]*\/\// { print; next }
            /^[[:space:]]*\/\/!/ { print; next }
            /^[[:space:]]*\/\/[\/!]/ { print; next }
            /^[[:space:]]*\/\*\*/ {
                # 处理块注释
                in_block_comment = 1
                print
                next
            }
            in_block_comment == 1 {
                if (/\*\//) {
                    in_block_comment = 0
                }
                print
                next
            }
            /^[[:space:]]*#/ && !/^#![[]/ { print; next }
            /^[[:space:]]*\/\*\*/ { print; next }
            /^[[:space:]]*\/\// { print; next }
            {
                if (printed_attrs == 0) {
                    # 打印之前先输出所有 inner attributes
                    while ((getline line < "'$tmp_file.attrs'") > 0) {
                        print line
                    }
                    close("'$tmp_file.attrs'")
                    printed_attrs = 1
                    if (line !~ /^[[:space:]]*$/) {
                        print ""
                    }
                }
                print
            }
            ' "$tmp_file.noattrs" > "$tmp_file"

            # 替换原文件
            mv "$tmp_file" "$file"
            echo "  Fixed!"
        else
            rm -f "$tmp_file"
        fi

        # 清理
        rm -f "$tmp_file.attrs" "$tmp_file.noattrs"
    else
        echo "File not found: $file"
    fi
done

echo "Done!"