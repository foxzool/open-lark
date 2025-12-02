#!/bin/bash

echo "修复剩余的序列化结构体..."

# 需要添加Serialize/Deserialize的结构体列表
structs=(
    "PatchFormQuestionResponse"
    "ListFormFieldQuestionResponse"
    "UpdateFormFieldQuestionResponse"
    "Record"
    "BatchCreateRecordResponse"
    "BatchDeleteRecordResponse"
    "BatchUpdateRecordResponse"
    "CreateRecordResponse"
    "DeleteRecordResponse"
    "SearchRecordResponse"
    "UpdateRecordResponse"
    "CreateViewResponse"
    "DeleteViewResponse"
    "UpdateViewResponse"
    "GetViewResponse"
    "ListViewsResponse"
)

# 为每个结构体查找并添加Serialize, Deserialize
for struct_name in "${structs[@]}"; do
    echo "处理结构体: $struct_name"
    find crates/openlark-docs/src -name "*.rs" -exec grep -l "pub struct $struct_name" {} \; | while read file; do
        echo "  在文件中修复: $file"

        # 添加Serialize, Deserialize derive宏
        sed -i '' "s/#\[derive(Debug, Clone)\]pub struct $struct_name/\#[derive(Debug, Clone, Serialize, Deserialize)]pub struct $struct_name/g" "$file"
        sed -i '' "s/#\[derive(Debug)\]pub struct $struct_name/\#[derive(Debug, Serialize, Deserialize)]pub struct $struct_name/g" "$file"
        sed -i '' "s/#\[derive(Clone)\]pub struct $struct_name/\#[derive(Clone, Serialize, Deserialize)]pub struct $struct_name/g" "$file"

        # 处理带缩进的情况
        sed -i '' "s/#\[derive(Debug, Clone)\]    pub struct $struct_name/\#[derive(Debug, Clone, Serialize, Deserialize)]    pub struct $struct_name/g" "$file"
        sed -i '' "s/#\[derive(Debug)\]    pub struct $struct_name/\#[derive(Debug, Serialize, Deserialize)]    pub struct $struct_name/g" "$file"
        sed -i '' "s/#\[derive(Clone)\]    pub struct $struct_name/\#[derive(Clone, Serialize, Deserialize)]    pub struct $struct_name/g" "$file"
    done
done

echo "剩余结构体修复完成!"