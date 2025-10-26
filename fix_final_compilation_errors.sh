#!/bin/bash

echo "=== 批量修复剩余的关键编译错误 ==="

# 1. 修复所有的重复 macro 调用问题
echo "修复重复的 macro 调用..."
find crates/open-lark-collaboration -name "*.rs" -type f -exec grep -l "impl_executable_builder_owned!" {} \; | while read file; do
    echo "修复文件: $file"
    # 删除重复的 macro 调用行
    sed -i '' '/^    AppService$/d' "$file"
    sed -i '' '/^);$/N; /^);\n    AppService$/d' "$file"
done

# 2. 修复所有缺少大括号的函数定义
echo "修复缺少大括号的函数定义..."
find crates/open-lark-communication -name "*.rs" -type f -exec grep -l "pub async fn create(" {} \; | while read file; do
    echo "修复文件: $file"
    # 在函数参数后面添加缺少的 {
        sed -i '' 's/req: &CreateRoleMemberRequest,$/req: \&CreateRoleMemberRequest,/' "$file"
    sed -i '' '/pub async fn create(/,/req: &CreateRoleMemberRequest,/{
        /req: &CreateRoleMemberRequest,$/s/$/\n    ) -> open_lark_core::core::SDKResult<CreateRoleMemberResponse> {/
    }' "$file"
done

# 3. 删除重复的导入行
echo "清理重复的导入..."
find crates -name "*.rs" -type f -exec sed -i '' '/use open_lark_core::core::{$/N; s/use open_lark_core::core::{$[[:space:]]*}.*$/use open_lark_core::core::{\n&/' {} \;

# 4. 修复所有 "use serde_json;    constants" 这种错误的导入格式
echo "修复错误的导入格式..."
find crates -name "*.rs" -type f -exec sed -i '' 's/use serde_json;[[:space:]]*constants:/use serde_json;/' {} \;
find crates -name "*.rs" -type f -exec sed -i '' 's/use serde_json;[[:space:]]*/use serde_json;/' {} \;

echo "批量修复完成！"