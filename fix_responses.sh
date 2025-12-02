#!/bin/bash

echo "批量修复响应结构体的serde derive..."

# 查找包含响应结构体的文件
find crates/openlark-docs/src -name "*.rs" -exec grep -l "pub struct.*Response" {} \; > response_files.txt

while IFS= read -r file; do
  echo "修复响应结构体: $file"

  # 为响应结构体添加 Serialize, Deserialize (查找以Response结尾的结构体)
  sed -i '' 's/#\[derive(Debug, Clone)\]pub struct \([A-Za-z0-9_]*Response\)/#[derive(Debug, Clone, Serialize, Deserialize)]pub struct \1/g' "$file"
  sed -i '' 's/#\[derive(Debug, Clone)\]    pub struct \([A-Za-z0-9_]*Response\)/#[derive(Debug, Clone, Serialize, Deserialize)]    pub struct \1/g' "$file"

  # 处理可能的其他格式
  sed -i '' 's/#\[derive(Debug)\]pub struct \([A-Za-z0-9_]*Response\)/#[derive(Debug, Serialize, Deserialize)]pub struct \1/g' "$file"
  sed -i '' 's/#\[derive(Clone)\]pub struct \([A-Za-z0-9_]*Response\)/#[derive(Clone, Serialize, Deserialize)]pub struct \1/g' "$file"

  # 处理Record结构体（在Record相关的错误中提到）
  sed -i '' 's/#\[derive(Debug, Clone)\]pub struct Record/#[derive(Debug, Clone, Serialize, Deserialize)]pub struct Record/g' "$file"
  sed -i '' 's/#\[derive(Debug)\]pub struct Record/#[derive(Debug, Serialize, Deserialize)]pub struct Record/g' "$file"

done < response_files.txt

# 清理临时文件
rm -f response_files.txt

echo "响应结构体修复完成!"