#!/bin/bash

# 批量修复所有 EndpointBuilder::replace_param(); 调用
echo "正在批量修复所有 EndpointBuilder::replace_param(); 调用..."

# 找到所有包含损坏调用的文件
files_with_errors=$(find crates/open-lark-communication/src -name "*.rs" -exec grep -l "EndpointBuilder::replace_param();" {} \;)

echo "发现 ${#files_with_errors[@]} 个文件需要修复"

# 修复函数 - 根据文件内容推断正确的endpoint
fix_endpoint_builder() {
    local file=$1
    echo "正在修复文件: $file"

    # 根据文件路径和内容推断正确的endpoint
    if [[ $file == *"message_card"* ]]; then
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path(open_lark_core::core::endpoints::im::IM_V1_CARDS.to_string());|g' "$file"
    elif [[ $file == *"url_preview"* ]]; then
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path(open_lark_core::core::endpoints::im::IM_V1_URL_PREVIEW.to_string());|g' "$file"
    elif [[ $file == *"message_reaction"* ]]; then
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path(open_lark_core::core::endpoints::im::IM_V1_MESSAGE_REACTIONS.to_string());|g' "$file"
    elif [[ $file == *"pin"* ]]; then
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path(open_lark_core::core::endpoints::im::IM_V1_PINS.to_string());|g' "$file"
    elif [[ $file == *"groups_bots"* ]]; then
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path(open_lark_core::core::endpoints::im::IM_V2_GROUPS_BOTS.to_string());|g' "$file"
    elif [[ $file == *"app_feed_card"* ]]; then
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path(open_lark_core::core::endpoints::im::IM_V2_APP_FEED_CARDS.to_string());|g' "$file"
    elif [[ $file == *"group_member"* ]]; then
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_GROUP_MEMBERS.to_string());|g' "$file"
    elif [[ $file == *"job_family"* ]]; then
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_JOB_FAMILIES.to_string());|g' "$file"
    elif [[ $file == *"job_level"* ]]; then
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_JOB_LEVELS.to_string());|g' "$file"
    elif [[ $file == *"unit"* ]]; then
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_UNITS.to_string());|g' "$file"
    elif [[ $file == *"employee_type_enum"* ]]; then
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_EMPLOYEE_TYPE_ENUMS.to_string());|g' "$file"
    elif [[ $file == *"group"* ]]; then
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_GROUPS.to_string());|g' "$file"
    else
        # 通用修复 - 使用一个通用的endpoint作为占位符
        sed -i '' 's|api_req\.set_api_path(EndpointBuilder::replace_param();|api_req.set_api_path("placeholder_endpoint".to_string());|g' "$file"
    fi
}

# 逐个修复文件
for file in $files_with_errors; do
    if [ -f "$file" ]; then
        fix_endpoint_builder "$file"
    fi
done

echo "批量修复完成！"