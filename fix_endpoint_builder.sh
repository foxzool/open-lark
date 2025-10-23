#!/bin/bash

# 批量修复 EndpointBuilder::replace_param(); 调用
echo "正在修复 EndpointBuilder::replace_param() 调用..."

# 需要修复的文件列表
files=(
    "crates/open-lark-communication/src/contact/v3/group_member.rs"
    "crates/open-lark-communication/src/contact/v3/job_family.rs"
    "crates/open-lark-communication/src/contact/v3/job_level.rs"
    "crates/open-lark-communication/src/contact/v3/unit.rs"
    "crates/open-lark-communication/src/contact/v3/employee_type_enum.rs"
    "crates/open-lark-communication/src/contact/v3/group.rs"
    "crates/open-lark-communication/src/im/v1/pin/mod.rs"
    "crates/open-lark-communication/src/im/v1/buzz_messages/mod.rs"
    "crates/open-lark-communication/src/im/v1/file/mod.rs"
    "crates/open-lark-communication/src/im/v1/message_card/mod.rs"
    "crates/open-lark-communication/src/im/v1/url_preview/mod.rs"
    "crates/open-lark-communication/src/im/v1/image/mod.rs"
    "crates/open-lark-communication/src/im/v1/message_reaction/mod.rs"
    "crates/open-lark-communication/src/im/v2/groups_bots/mod.rs"
    "crates/open-lark-communication/src/im/v2/app_feed_card/mod.rs"
)

for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "检查文件: $file"

        # 检查是否包含损坏的调用
        if grep -q "EndpointBuilder::replace_param();" "$file"; then
            echo "发现损坏的调用，正在手动修复: $file"
            # 这里需要手动修复，因为每个文件的上下文不同
            echo "需要手动修复文件: $file"
        else
            echo "文件正常: $file"
        fi
    else
        echo "文件不存在: $file"
    fi
done

echo "检查完成！需要手动修复包含损坏调用的文件。"

# 为 buzz_messages/mod.rs 提供快速修复方案
echo "正在快速修复 buzz_messages/mod.rs..."

# 临时修复 buzz_messages/mod.rs 中的常见模式
if [ -f "crates/open-lark-communication/src/im/v1/buzz_messages/mod.rs" ]; then
    sed -i '' 's/api_req\.set_api_path(EndpointBuilder::replace_param();/api_req.set_api_path(open_lark_core::core::endpoints::im::IM_V1_URGENT_MESSAGES.to_string());/g' \
        crates/open-lark-communication/src/im/v1/buzz_messages/mod.rs
    echo "已快速修复 buzz_messages/mod.rs"
fi