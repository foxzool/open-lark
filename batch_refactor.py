#!/usr/bin/env python3
"""
批量应用ExecutableBuilder trait重构
"""
import os
import re
import glob

def find_files_with_execute_methods():
    """查找包含execute方法的Builder文件"""
    files_to_process = []
    
    # 查找所有相关的Rust文件
    pattern = "src/service/cloud_docs/bitable/v1/**/*.rs"
    for file_path in glob.glob(pattern, recursive=True):
        if file_path.endswith('/mod.rs'):
            continue
            
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                
                # 检查是否包含execute方法的Builder
                if ('pub async fn execute(' in content and 
                    'RequestBuilder' in content and
                    'pub fn build(' in content):
                    files_to_process.append(file_path)
        except Exception as e:
            print(f"Error reading {file_path}: {e}")
    
    return files_to_process

def extract_builder_info(content):
    """提取Builder信息"""
    # 查找Builder类型名
    builder_match = re.search(r'pub struct (\w+RequestBuilder)', content)
    if not builder_match:
        return None
        
    builder_name = builder_match.group(1)
    
    # 查找Request类型名
    request_match = re.search(r'pub struct (\w+Request)', content)
    if not request_match:
        return None
        
    request_name = request_match.group(1)
    
    # 查找Response类型名
    response_match = re.search(r'BaseResponse<(\w+Response)>', content)
    if not response_match:
        return None
        
    response_name = response_match.group(1)
    
    # 推断Service名称和方法名
    if 'list' in request_name.lower():
        method_name = 'list'
    elif 'create' in request_name.lower():
        method_name = 'create'
    elif 'update' in request_name.lower():
        method_name = 'update'
    elif 'delete' in request_name.lower():
        method_name = 'delete'
    elif 'search' in request_name.lower():
        method_name = 'search'
    else:
        method_name = 'unknown'
    
    return {
        'builder': builder_name,
        'request': request_name,
        'response': response_name,
        'method': method_name
    }

def apply_refactor(file_path):
    """应用重构到单个文件"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        info = extract_builder_info(content)
        if not info:
            print(f"Could not extract info from {file_path}")
            return False
        
        # 1. 添加导入
        if 'impl_executable_builder' not in content:
            content = re.sub(
                r'(use crate::core::\{[^}]+\});',
                r'use crate::{\n    core::{\g<1>\n    },\n    impl_executable_builder,\n};',
                content
            )
            content = content.replace('use crate::core::{', 'use crate::{\n    core::{')
            content = content.replace('};', '\n    },\n    impl_executable_builder,\n};')
        
        # 2. 添加Clone trait
        content = re.sub(
            rf'(#\[derive\([^)]*\))\s*\npub struct {info["request"]}',
            rf'\1, Clone)]\npub struct {info["request"]}',
            content
        )
        if ', Clone, Clone' in content:
            content = content.replace(', Clone, Clone', ', Clone')
        
        # 3. 删除execute方法
        execute_pattern = r'\s*/// [^/]*execute[^/]*\n\s*pub async fn execute\([^}]+\}\s*\n\s*/// [^/]*execute[^/]*\n\s*pub async fn execute_with_options\([^}]+\}'
        content = re.sub(execute_pattern, '', content, flags=re.DOTALL)
        
        # 更简单的删除方式
        lines = content.split('\n')
        new_lines = []
        skip_lines = False
        
        for i, line in enumerate(lines):
            if 'pub async fn execute(' in line:
                skip_lines = True
                continue
            elif skip_lines and line.strip() == '}':
                # 检查下一个execute_with_options
                if i + 1 < len(lines) and 'execute_with_options' in ''.join(lines[i+1:i+5]):
                    continue
                else:
                    skip_lines = False
                    continue
            elif not skip_lines:
                new_lines.append(line)
        
        content = '\n'.join(new_lines)
        
        # 4. 添加trait实现
        trait_impl = f'''
// 应用ExecutableBuilder trait到{info["builder"]}
impl_executable_builder!(
    {info["builder"]},
    super::AppService,  // 需要根据实际情况调整
    {info["request"]},
    BaseResponse<{info["response"]}>,
    {info["method"]}
);'''
        
        # 在合适位置插入trait实现
        if '// 应用ExecutableBuilder trait' not in content:
            # 在最后一个impl块后插入
            impl_end = content.rfind('}', 0, content.rfind('mod tests') if 'mod tests' in content else len(content))
            if impl_end != -1:
                content = content[:impl_end+1] + trait_impl + content[impl_end+1:]
        
        # 写回文件
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        
        print(f"✓ Refactored {file_path}")
        return True
        
    except Exception as e:
        print(f"✗ Error processing {file_path}: {e}")
        return False

def main():
    print("🔍 Finding files with execute methods...")
    files = find_files_with_execute_methods()
    
    print(f"📋 Found {len(files)} files to process:")
    for f in files[:10]:  # 显示前10个
        print(f"  - {f}")
    if len(files) > 10:
        print(f"  ... and {len(files) - 10} more")
    
    print(f"\n🔧 Starting batch refactor...")
    success_count = 0
    
    for file_path in files:
        if apply_refactor(file_path):
            success_count += 1
    
    print(f"\n📊 Refactor complete: {success_count}/{len(files)} files processed successfully")

if __name__ == '__main__':
    main()