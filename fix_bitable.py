#!/usr/bin/env python3
"""
修复bitable模块中的ApiRequest语法错误
"""

import os
import re
import glob

def fix_imports(content):
    """修复import语句"""
    # 添加ResponseFormat导入
    pattern = r'use openlark_core::\{([^}]+)\};'
    if 'core::ResponseFormat' not in content:
        content = re.sub(
            pattern,
            r'use openlark_core::{\1,\n    core::ResponseFormat,};',
            content
        )
    return content

def fix_api_request_new(content):
    """修复ApiRequest::new()调用"""
    # 匹配模式: ApiRequest::new().method(...).api_path(...).config(...).build()
    pattern = r'ApiRequest::new\(\)\s*\.method\(HttpMethod::(\w+)\)\s*\.api_path\([^)]*\)\s*\.config\([^)]*\)\s*\.build\(\)'

    # 替换为正确的调用
    methods = {
        'GET': 'get',
        'POST': 'post',
        'PUT': 'put',
        'DELETE': 'delete',
        'PATCH': 'patch'
    }

    def replacer(match):
        method = match.group(1).upper()
        method_name = methods.get(method, method.lower())
        return f'ApiRequest::{method_name}("").header("Content-Type", "application/json")'

    return re.sub(pattern, replacer, content)

def fix_api_path_calls(content):
    """修复错误的api_path调用"""
    # 修复 .api_path(format!( .replace({app_token}, &request.app_token) 这类错误
    patterns = [
        # 模式1: api_request.api_path(format!( .replace({app_token}, &request.app_token)
        (r'\.api_path\(format!\(\s*\.replace\(\{[^}]+\}, &[^)]+\)', ''),
        # 模式2: format!(/open-apis/.../{}/, self.app_token)
        (r'format!\(([^/][^,]*), ([^)]+)\)', r'format!(/\1, \2)'),
    ]

    for pattern, replacement in patterns:
        content = re.sub(pattern, replacement, content)

    return content

def fix_execute_method(content):
    """修复execute方法中的错误调用"""
    # 修复Transport::request调用
    patterns = [
        # 模式1: self.api_request.config() 和 &config.clone()
        (r'let config = self\.api_request\.config\(\);[\s\n]*let response = Transport::request\(self\.api_request, &config\.clone\(\), None\)\.await\?;',
         r'let response = Transport::request(self.api_request, config, None).await?;'),
        # 模式2: api_request.body(serde_json::to_vec(&body)?)
        (r'\.body\(serde_json::to_vec\(&([^)]+)\)\?\)', r'.body(\1)'),
    ]

    for pattern, replacement in patterns:
        content = re.sub(pattern, replacement, content)

    return content

def fix_syntax_errors(content):
    """修复其他语法错误"""
    patterns = [
        # 修复重复的ApiResponseTrait导入
        (r'api::\{ApiRequest, ApiResponseTrait, HttpMethod\},\s*api::\{ApiResponseTrait\}', 'api::{ApiRequest, ApiResponseTrait, HttpMethod}'),
        # 修复空的空行
        (r'\n\s*\n\s*\n', '\n\n'),
    ]

    for pattern, replacement in patterns:
        content = re.sub(pattern, replacement, content)

    return content

def process_file(file_path):
    """处理单个文件"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content

        # 应用所有修复
        content = fix_imports(content)
        content = fix_api_request_new(content)
        content = fix_api_path_calls(content)
        content = fix_execute_method(content)
        content = fix_syntax_errors(content)

        # 如果有变化，写回文件
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ 修复: {file_path}")
            return True
        else:
            print(f"⏭️  无需修复: {file_path}")
            return False

    except Exception as e:
        print(f"❌ 错误处理 {file_path}: {e}")
        return False

def main():
    """主函数"""
    bitable_dir = "/Users/zool/RustroverProjects/open-lark/crates/openlark-docs/src/bitable"

    # 查找所有.rs文件（排除.bak文件）
    pattern = os.path.join(bitable_dir, "**", "*.rs")
    rust_files = glob.glob(pattern, recursive=True)
    rust_files = [f for f in rust_files if not f.endswith('.bak')]

    print(f"找到 {len(rust_files)} 个Rust文件")

    fixed_count = 0
    for file_path in rust_files:
        if process_file(file_path):
            fixed_count += 1

    print(f"\n🎉 完成！修复了 {fixed_count} 个文件")

if __name__ == "__main__":
    main()