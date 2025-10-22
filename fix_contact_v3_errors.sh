#!/bin/bash

echo "批量修复 contact v3 文件的编译错误..."

# 定义要修复的文件列表
files=(
    "group_member.rs"
    "job_family.rs"
    "job_level.rs"
    "scope.rs"
    "job_title.rs"
    "unit.rs"
    "work_city.rs"
)

# 修复 resp 变量未定义的问题
for file in "${files[@]}"; do
    filepath="/Users/zool/RustroverProjects/open-lark/crates/open-lark-communication/src/contact/v3/$file"
    if [ -f "$filepath" ]; then
        echo "修复 $file 中的 resp 变量问题..."

        # 在 Transport::request 调用前添加 let resp =
        sed -i '' 's/let api_req = ApiRequest {$/let resp = Transport::<ResponseType>::request(api_req, \&self.config, None).await?;/g' "$filepath"

        # 修复 ApiRequest 构造问题 - 添加缺失的字段
        sed -i '' 's/api_path: EndpointBuilder::replace_param(/api_path: EndpointBuilder::replace_param(/g' "$filepath"
    fi
done

echo "批量修复完成!"