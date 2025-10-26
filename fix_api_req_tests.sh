#!/bin/bash

echo "=== 修复 api_req.rs 测试语法错误 ==="

FILE="/Users/zool/RustroverProjects/open-lark/crates/open-lark-core/src/core/api_req.rs"

# 备份文件
cp "$FILE" "${FILE}.backup"

echo "修复测试函数中的 ApiRequest 语法错误..."

# 修复模式：ApiRequest::default(); 后跟字段赋值 -> ApiRequest { ..Default::default() }
sed -i '' -E '
# 模式1：处理简单的情况
s|let (mut )?api_req = ApiRequest::default\(\);|let \1api_req = ApiRequest {|
g
' "$FILE"

# 现在修复字段结构，将字段赋值转换为正确的字段初始化
sed -i '' -E '
# 处理字段行 - 移除前导空格并添加正确的字段标识
s|^            ([a-zA-Z_][a-zA-Z0-9_]*): |            \1: |
g
' "$FILE"

# 在字段块结束后添加正确的结尾
sed -i '' -E '
# 查找需要修复的函数并在适当位置添加结尾
/let (mut )?api_req = ApiRequest \{/,/^        }/ {
    # 如果没有找到 ..Default::default()，则在最后的 } 前添加
    /^        }$/ {
        # 检查前一行是否有逗号
        N
        /,$/! s/^        }$/            ..Default::default()\
        }/
        P
        D
    }
}
g
' "$FILE"

echo "修复缺少分号的问题..."

# 修复缺少分号的问题
sed -i '' -E '
s/^        };$/        };/
g
' "$FILE"

# 验证修复结果
echo "验证语法修复结果..."
if rustc --edition 2021 --crate-type lib "$FILE" > /dev/null 2>&1; then
    echo "✅ 语法修复成功！"
    rm -f "${FILE}.backup"
else
    echo "❌ 仍有语法错误，恢复备份文件"
    mv "${FILE}.backup" "$FILE"
    rustc --edition 2021 --crate-type lib "$FILE"
fi

echo "=== 修复完成 ==="