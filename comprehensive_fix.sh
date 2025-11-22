#!/bin/bash

echo "全面修复API请求语法错误..."

# 查找所有有问题的文件并修复
files=$(find crates/openlark-docs/src/bitable -name "*.rs" -type f | xargs grep -l "ApiRequest::[^;]*$")

for file in $files; do
    echo "修复文件: $file"

    # 修复不完整的ApiRequest构造
    sed -i '' '/let api_req = ApiRequest::/,/};$/{
        s/let api_req = ApiRequest::$/let api_req = ApiRequest::post("",
        s/url: format!(/"/g
        s/),$/"),/g
        s/body: Some(serde_json::to_string(body: serde_json::to_vec(&body)?,body)?.into()),/.body(serde_json::to_string(\&body)?),/g
        s/\.query(HashMap::new())/.query(HashMap::new())/g
        s/^\s*};$//
    }' "$file"

    # 修复重复的query调用
    sed -i '' 's/\.query(HashMap::new())\.query(HashMap::new())/.query(HashMap::new())/g' "$file"

    # 确保每个API请求都正确结束
    sed -i '' '/let api_req = ApiRequest::post/,/\.query(HashMap::new());$/{
        /\.query(HashMap::new());$/!{
            /$/{
                a\
            .query(HashMap::new());
            }
        }
    }' "$file"
done

echo "全面修复完成"