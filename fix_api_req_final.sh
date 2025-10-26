#!/bin/bash

echo "=== 最终修复 api_req.rs 语法错误 ==="

FILE="/Users/zool/RustroverProjects/open-lark/crates/open-lark-core/src/core/api_req.rs"

# 批量修复模式：let mut api_req = ApiRequest::default(); 后跟字段赋值
echo "修复测试函数中的ApiRequest初始化..."

# 模式：将 let mut api_req = ApiRequest::default(); 替换为 let api_req = ApiRequest {
sed -i '' -E '
s/let (mut )?api_req = ApiRequest::default\(\);/let api_req = ApiRequest {/g
' "$FILE"

# 修复字段赋值格式
sed -i '' -E '
# 移除 ApiRequest::default() 后面的字段行
/let api_req = ApiRequest \{/,/^[[:space:]]*}/ {
    # 移除错误的字段行
    /^[[:space:]]+[a-zA-Z_][a-zA-Z0-9_]*:/d
}
' "$FILE"

# 在所有测试函数中添加必要的 ..Default::default()
echo "添加缺失的 ..Default::default()..."

# 查找缺少 ..Default::default() 的结构体初始化
awk '
/let api_req = ApiRequest \{/ {
    brace_count = 1
    line_num = NR
    in_block = 1

    # 读取直到找到对应的结束
    while (getline > 0 && in_block) {
        if (/\{/) brace_count++
        if (/\}/) {
            brace_count--
            if (brace_count == 0) {
                # 检查这一行是否有 ..Default::default()
                if (!/\.\.Default::default\(\)/) {
                    # 在 } 前添加 ..Default::default()
                    sub(/}[[:space:]]*$/, "..Default::default()\\n        }")
                }
                in_block = 0
            }
        }
    }
    print
}
' "$FILE" > "${FILE}.tmp"

mv "${FILE}.tmp" "$FILE"

echo "验证修复结果..."

# 尝试编译
if cargo check --package open-lark-core > /dev/null 2>&1; then
    echo "✅ 语法修复成功！"
else
    echo "❌ 仍有语法错误，显示详情："
    cargo check --package open-lark-core 2>&1 | head -20
fi

echo "=== 修复完成 ==="