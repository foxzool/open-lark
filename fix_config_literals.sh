#!/bin/bash

# 批量修复 Config 结构体字面量的脚本

for file in $(find src/service -name "*.rs" -exec grep -l "Config {" {} \;); do
    echo "Processing $file..."
    
    # 备份原文件
    cp "$file" "$file.bak"
    
    # 使用 sed 进行基本的替换模式
    # 这里我们需要手动处理，因为每个文件的结构可能不同
    # 先查看文件内容决定如何处理
    echo "File needs manual inspection: $file"
done
