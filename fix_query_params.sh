#!/bin/bash

# 批量修复查询参数类型错误脚本
# 该脚本解决从 HashMap<String, String> 到 HashMap<&'static str, String> 的迁移

set -e

echo "开始批量修复查询参数类型错误..."

# 备份当前状态
echo "创建备份分支..."
git add .
git stash push -m "Before query params fix"

# 1. 修复最常见的查询参数，移除键上的 .to_string() 调用
echo "第一阶段：移除常见查询参数键的 .to_string() 调用..."

# 修复 page_size
find src -name "*.rs" -type f -exec sed -i '' 's/"page_size"\.to_string()/"page_size"/g' {} \;

# 修复 page_token  
find src -name "*.rs" -type f -exec sed -i '' 's/"page_token"\.to_string()/"page_token"/g' {} \;

# 修复 user_id_type
find src -name "*.rs" -type f -exec sed -i '' 's/"user_id_type"\.to_string()/"user_id_type"/g' {} \;

# 修复 user_id
find src -name "*.rs" -type f -exec sed -i '' 's/"user_id"\.to_string()/"user_id"/g' {} \;

# 修复其他常见参数
find src -name "*.rs" -type f -exec sed -i '' 's/"start_time"\.to_string()/"start_time"/g' {} \;
find src -name "*.rs" -type f -exec sed -i '' 's/"end_time"\.to_string()/"end_time"/g' {} \;
find src -name "*.rs" -type f -exec sed -i '' 's/"department_id"\.to_string()/"department_id"/g' {} \;
find src -name "*.rs" -type f -exec sed -i '' 's/"department_id_type"\.to_string()/"department_id_type"/g' {} \;
find src -name "*.rs" -type f -exec sed -i '' 's/"file_type"\.to_string()/"file_type"/g' {} \;
find src -name "*.rs" -type f -exec sed -i '' 's/"image_type"\.to_string()/"image_type"/g' {} \;
find src -name "*.rs" -type f -exec sed -i '' 's/"receive_id_type"\.to_string()/"receive_id_type"/g' {} \;
find src -name "*.rs" -type f -exec sed -i '' 's/"chat_id"\.to_string()/"chat_id"/g' {} \;
find src -name "*.rs" -type f -exec sed -i '' 's/"message_id"\.to_string()/"message_id"/g' {} \;

echo "第一阶段完成。"

# 2. 使用 QueryParams 常量替换可能的情况（更复杂，需要导入模块）
echo "第二阶段：替换为 QueryParams 常量..."

# 创建临时的查找和替换脚本
cat > /tmp/replace_with_constants.py << 'EOF'
#!/usr/bin/env python3
import os
import re
import sys

# 定义常量映射
PARAM_CONSTANTS = {
    '"page_size"': 'crate::core::query_params::QueryParams::PAGE_SIZE',
    '"page_token"': 'crate::core::query_params::QueryParams::PAGE_TOKEN', 
    '"user_id"': 'crate::core::query_params::QueryParams::USER_ID',
    '"user_id_type"': 'crate::core::query_params::QueryParams::USER_ID_TYPE',
    '"department_id"': 'crate::core::query_params::QueryParams::DEPARTMENT_ID',
    '"department_id_type"': 'crate::core::query_params::QueryParams::DEPARTMENT_ID_TYPE',
    '"start_time"': 'crate::core::query_params::QueryParams::START_TIME',
    '"end_time"': 'crate::core::query_params::QueryParams::END_TIME',
    '"file_type"': 'crate::core::query_params::QueryParams::FILE_TYPE',
    '"chat_id"': 'crate::core::query_params::QueryParams::CHAT_ID',
    '"message_id"': 'crate::core::query_params::QueryParams::MESSAGE_ID',
}

def should_add_import(file_content):
    """检查是否需要添加 QueryParams 导入"""
    return 'use crate::core::query_params::QueryParams' not in file_content

def add_import_if_needed(file_path, content):
    """如果需要且文件被修改，添加 QueryParams 导入"""
    if should_add_import(content):
        # 查找合适的导入位置
        lines = content.split('\n')
        import_line = 'use crate::core::query_params::QueryParams;'
        
        # 找到最后一个 use crate::core 导入语句的位置
        insert_index = 0
        for i, line in enumerate(lines):
            if line.strip().startswith('use crate::core::'):
                insert_index = i + 1
        
        # 如果没找到 core 导入，找其他 use 语句
        if insert_index == 0:
            for i, line in enumerate(lines):
                if line.strip().startswith('use '):
                    insert_index = i + 1
        
        lines.insert(insert_index, import_line)
        return '\n'.join(lines)
    
    return content

def process_file(file_path):
    """处理单个文件"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        modified = False
        
        # 替换查询参数常量
        for old_param, new_const in PARAM_CONSTANTS.items():
            pattern = f'\.insert\({re.escape(old_param)}, '
            replacement = f'.insert({new_const}, '
            if re.search(pattern, content):
                content = re.sub(pattern, replacement, content)
                modified = True
        
        # 如果文件被修改，添加必要的导入
        if modified:
            content = add_import_if_needed(file_path, content)
            
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            
            print(f"已处理: {file_path}")
    
    except Exception as e:
        print(f"处理 {file_path} 时出错: {e}")

def main():
    # 遍历 src 目录下所有 .rs 文件
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                process_file(file_path)

if __name__ == "__main__":
    main()
EOF

# 运行 Python 脚本（但这里我们先跳过，因为可能会导致编译问题）
# python3 /tmp/replace_with_constants.py

echo "第二阶段跳过，避免复杂的导入问题。"

# 3. 修复 request_executor 中的类型问题
echo "第三阶段：修复 request_executor.rs..."

# 这个需要手动修复，因为涉及参数类型更改

echo "修复完成！开始编译测试..."

# 测试编译
if cargo check --quiet; then
    echo "✅ 编译成功！"
    echo "修复的文件数量："
    git diff --name-only | wc -l
else
    echo "❌ 还有编译错误，需要手动修复剩余问题"
    echo "查看错误详情："
    cargo check
fi

echo "脚本执行完成。如果还有错误，请手动检查剩余的 .to_string() 调用。"