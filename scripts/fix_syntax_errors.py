#!/usr/bin/env python3
"""
修复添加文档过程中引入的语法错误
"""

import re
from pathlib import Path
from typing import List, Tuple

def find_and_fix_syntax_errors(root_dir: Path) -> List[Tuple[Path, str]]:
    """查找并修复语法错误"""
    fixed_files = []

    for rust_file in root_dir.rglob("*.rs"):
        if 'target' in str(rust_file) or '.git' in str(rust_file):
            continue

        try:
            with open(rust_file, 'r', encoding='utf-8') as f:
                content = f.read()

            original_content = content

            # 修复问题1: 文档注释放在了错误的位置（比如在分号后面）
            # 模式: 分号后面跟着文档注释
            content = re.sub(
                r'};\s*/// # API文档\s*\n\s*///\s*\n\s*/// (https://open\.feishu\.cn/document/[^\n]+)\n',
                r'}\n    /// # API文档\n    ///\n    /// \1\n',
                content
            )

            # 修复问题2: 文档注释插入到表达式中间
            # 移除错误位置的文档注释
            content = re.sub(
                r'\n\s*/// # API文档\s*\n\s*///\s*\n\s*/// (https://open\.feishu\.cn/document/[^\n]+)\n\s*\n',
                r'\n',
                content
            )

            # 修复问题3: 不完整的ApiRequest结构体初始化
            content = re.sub(
                r'let api_req = ApiRequest \{\s*http_method: reqwest::Method::([A-Z_]+),\s*api_path: ([^,}]+),\s*supported_access_token_types: vec!\[([^\]]+)\],\s*\.\.\.Default::default\(\)',
                r'''let api_req = ApiRequest {
                http_method: reqwest::Method::\1,
                api_path: \2,
                supported_access_token_types: vec![\3],
                body: Vec::new(),
                query_params: std::collections::HashMap::new(),
                path_params: std::collections::HashMap::new(),
                file: None,
                ..Default::default()''',
                content,
                flags=re.MULTILINE | re.DOTALL
            )

            # 修复问题4: 错误的函数参数列表
            content = re.sub(
                r'pub async fn \w+\(\s*\) -> crate::core::SDKResult<[^>]+> \{\s*/// # API文档',
                r'pub async fn get() -> crate::core::SDKResult<ListResponse> {\n        /// # API文档',
                content
            )

            # 修复问题5: 移除孤立的文档注释
            lines = content.split('\n')
            fixed_lines = []
            i = 0

            while i < len(lines):
                line = lines[i]

                # 检查是否是孤立的文档注释
                if ('/// # API文档' in line and
                    i > 0 and
                    not (lines[i-1].strip() == '' or
                         lines[i-1].strip().startswith('///') or
                         lines[i-1].strip().startswith('//!') or
                         'pub fn' in lines[i-1] or
                         'pub async fn' in lines[i-1] or
                         '{' in lines[i-1])):

                    # 跳过这个孤立的文档注释
                    i += 1
                    # 跳过相关的文档行
                    while i < len(lines) and (lines[i].strip().startswith('///') or lines[i].strip() == ''):
                        i += 1
                    continue

                fixed_lines.append(line)
                i += 1

            content = '\n'.join(fixed_lines)

            # 只有内容发生变化时才写回文件
            if content != original_content:
                with open(rust_file, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixed_files.append((rust_file, "语法错误修复"))

        except Exception as e:
            print(f"❌ 修复文件失败 {rust_file}: {e}")

    return fixed_files

def main():
    print("🚀 开始修复语法错误...")
    print("=" * 60)

    root_dir = Path('/Users/zool/RustroverProjects/open-lark/src')

    fixed_files = find_and_fix_syntax_errors(root_dir)

    print(f"\n📊 修复结果:")
    print(f"   • 修复文件数: {len(fixed_files)}")

    if fixed_files:
        print(f"   • 修复的文件:")
        for file_path, reason in fixed_files:
            relative_path = file_path.relative_to('/Users/zool/RustroverProjects/open-lark')
            print(f"     • {relative_path} - {reason}")

    print(f"\n✅ 语法错误修复完成！")

    return 0

if __name__ == "__main__":
    exit(main())