#!/bin/bash

echo "修复剩余的API结构问题..."

# 修复直接构造ApiRequest结构体的文件
files=$(find crates/openlark-docs/src/bitable -name "*.rs" -type f | xargs grep -l "http_method.*reqwest::Method")

for file in $files; do
    echo "修复API结构: $file"

    # 替换http_method字段为正确的构造方法
    sed -i '' 's/http_method: reqwest::Method::GET,/url: format!(/g' "$file"
    sed -i '' 's/http_method: reqwest::Method::POST,/url: format!(/g' "$file"
    sed -i '' 's/http_method: reqwest::Method::PUT,/url: format!(/g' "$file"
    sed -i '' 's/http_method: reqwest::Method::DELETE,/url: format!(/g' "$file"
    sed -i '' 's/http_method: reqwest::Method::PATCH,/url: format!(/g' "$file"

    # 将构造方式改为使用对应的静态方法
    sed -i '' 's/let api_req = ApiRequest {/let api_req = ApiRequest::/g' "$file"

    # 修复body字段类型
    sed -i '' 's/body: serde_json::to_vec(&body)?,/body: Some(serde_json::to_string(&body)?.into()),/g' "$file"
    sed -i '' 's/body: None,/body: None,/g' "$file"

    # 修复结构体结尾
    sed -i '' 's/\.\.Default::default()/.query(HashMap::new())/g' "$file"
    sed -i '' 's/query: HashMap::new(),/.query(HashMap::new())/g' "$file"
    sed -i '' 's/query_params,/query(),/g' "$file"
done

echo "剩余API结构修复完成"