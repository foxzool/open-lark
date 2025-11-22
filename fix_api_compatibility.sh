#!/bin/bash

echo "开始修复API兼容性问题..."

# 1. 修复http_method字段 -> 使用正确的HTTP方法
find crates/openlark-docs/src/bitable -name "*.rs" -type f | while read file; do
    if grep -q "http_method.*reqwest::Method" "$file"; then
        echo "修复HTTP方法: $file"
        # 将错误的http_method字段改为使用正确的构造方法
        sed -i '' 's/ApiRequest::post("")\.method(reqwest::Method::GET)/ApiRequest::get("")/g' "$file"
        sed -i '' 's/ApiRequest::post("")\.method(reqwest::Method::POST)/ApiRequest::post("")/g' "$file"
        sed -i '' 's/ApiRequest::post("")\.method(reqwest::Method::PUT)/ApiRequest::put("")/g' "$file"
        sed -i '' 's/ApiRequest::post("")\.method(reqwest::Method::DELETE)/ApiRequest::delete("")/g' "$file"
        sed -i '' 's/ApiRequest::post("")\.method(reqwest::Method::PATCH)/ApiRequest::put("")/g' "$file"
    fi
done

# 2. 修复api_path字段 -> 使用url字段
find crates/openlark-docs/src/bitable -name "*.rs" -type f | while read file; do
    if grep -q "api_path:" "$file"; then
        echo "修复API路径: $file"
        # 将api_path改为url
        sed -i '' 's/api_path:/url:/g' "$file"
    fi
done

# 3. 删除不存在的字段
find crates/openlark-docs/src/bitable -name "*.rs" -type f | while read file; do
    if grep -q "supported_access_token_types:" "$file"; then
        echo "删除不存在的字段: $file"
        # 删除这些不存在的字段
        sed -i '' '/supported_access_token_types:/d' "$file"
    fi

    if grep -q "query_params:" "$file"; then
        echo "修复query参数: $file"
        # 将query_params改为query
        sed -i '' 's/query_params:/query:/g' "$file"
    fi
done

# 4. 修复body字段类型
find crates/openlark-docs/src/bitable -name "*.rs" -type f | while read file; do
    if grep -q "body: Vec::new()" "$file"; then
        echo "修复body字段: $file"
        # 将空的Vec改为None
        sed -i '' 's/body: Vec::new()/body: None/g' "$file"
    fi
done

echo "API兼容性修复完成"