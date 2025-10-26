#!/bin/bash

echo "=== 修复剩余的 api_req.rs 语法错误 ==="

FILE="/Users/zool/RustroverProjects/open-lark/crates/open-lark-core/src/core/api_req.rs"

# 模式1：修复 let mut api_req = ApiRequest::default(); 后跟字段赋值
sed -i '' -E '
# 查找有问题的测试函数
/let (mut )?api_req = ApiRequest::default\(\);/,/^[[:space:]]*}/ {
    # 如果行以字段赋值开始（有缩进），将其转换为正确的结构体初始化
    s/^[[:space:]]+([a-zA-Z_][a-zA-Z0-9_]*):/            \1: /

    # 如果下一行看起来像字段，继续转换
    n
    /^[[:space:]]+[a-zA-Z_][a-zA-Z0-9_]*:/ {
        s/^[[:space:]]+([a-zA-Z_][a-zA-Z0-9_]*):/            \1: /
        n
    }
}
' "$FILE"

# 模式2：修复将 ApiRequest::default(); 改为 ApiRequest {
sed -i '' -E '
s/let (mut )?api_req = ApiRequest::default\(\);/let \1api_req = ApiRequest {/
g
' "$FILE"

# 模式3：在适当位置添加 ..Default::default()
sed -i '' -E '
# 查找需要修复的结构体初始化
/let (mut )?api_req = ApiRequest \{/,/^[[:space:]]*}/ {
    # 如果看到字段但没有 ..Default::default()，在最后的 } 前添加
    /^[[:space:]]*}/ {
        # 检查前一行是否有逗号
        N
        s/^[[:space:]]}$/,/            ..Default::default()\
        }/
        P
        D
    }
}
' "$FILE"

# 模式4：确保所有语句都有分号
sed -i '' -E '
# 查找没有分号的语句结尾
s/^[[:space:]]*}[[:space:]]*$/};/
g
' "$FILE"

echo "检查语法修复结果..."

# 尝试编译检查
if rustc --edition 2021 --crate-type lib "$FILE" --extern reqwest --extern serde_json --allow warnings > /dev/null 2>&1; then
    echo "✅ 语法修复成功！"
else
    echo "⚠️  仍有语法错误，显示错误详情："
    rustc --edition 2021 --crate-type lib "$FILE" --extern reqwest --extern serde_json --allow warnings 2>&1 | head -20
fi

echo "=== 修复完成 ==="