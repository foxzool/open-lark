#!/usr/bin/env python3
"""
批量为新风格 API 添加 execute_with_options 方法（改进版）

处理策略：
1. 只处理 Builder 风格：execute(self) 没有额外参数
2. 跳过函数风格：execute(self, params: ...) 需要保留 params 参数
"""

import argparse
import re
from pathlib import Path


def transform_execute_method(content: str) -> tuple[str, int]:
    """
    转换 Builder 风格的 execute 方法
    只处理 execute(self)，跳过 execute(self, params)
    """
    count = 0

    def replace_fn(match):
        nonlocal count
        count += 1

        fn_def = match.group(1)      # pub async fn execute(...) -> SDKResult<Type> {
        response_type = match.group(2)  # Type
        fn_body = match.group(3)      # 函数体
        transport_line = match.group(4)  # let response = Transport::request(..., &self.config, None).await?;
        rest = match.group(5)         # extract_response_data...}

        # 移除尾部的闭合大括号
        rest = rest.rstrip()
        if rest.endswith('}'):
            rest = rest[:-1]

        # 构建 execute_with_options
        new_impl = f'''{fn_def}
            self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
        }}

        pub async fn execute_with_options(
            self,
            option: openlark_core::req_option::RequestOption,
        ) -> SDKResult<{response_type}> {{
{fn_body}
            {transport_line.replace('None', 'Some(option)')}
{rest}
        }}'''

        return new_impl

    # 更精确的模式：只匹配 execute(self) 不匹配 execute(self, params)
    pattern = r'(pub async fn execute\(\s*self\s*\)\s*->\s*SDKResult<([^>]+)>\s*\{)([\s\S]*?)(let response = Transport::request\([^)]+,\s*&self\.config,\s+)None(\)\.await\?;)([\s\S]*?extract_response_data\([^)]+\)[;]?[\s\n]*\})'

    new_content = re.sub(pattern, replace_fn, content, flags=re.DOTALL)

    return new_content, count


def main():
    parser = argparse.ArgumentParser(description="批量添加 execute_with_options 方法（仅 Builder 风格）")
    parser.add_argument("--dry-run", action="store_true", help="预览模式，不修改文件")
    parser.add_argument("--path", default="crates/openlark-docs/src", help="源代码路径")
    args = parser.parse_args()

    src_path = Path(args.path)

    # 查找所有需要更新的文件
    import subprocess
    result = subprocess.run(
        ['rg', 'Transport::request\\([^,]+,\\s*&[^,]+,\\s+None\\)\\.await',
         '--files-with-matches', str(src_path)],
        capture_output=True, text=True
    )

    if result.returncode != 0:
        print("未找到需要更新的文件")
        return

    files = [f.strip() for f in result.stdout.split('\n') if f.strip()]

    print(f"找到 {len(files)} 个需要检查的文件")

    total_changes = 0
    for file_path in files:
        path = Path(file_path)
        if not path.exists():
            continue

        try:
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()

            new_content, count = transform_execute_method(content)

            if count > 0:
                total_changes += count
                if args.dry_run:
                    print(f"[预览] {path}: {count} 处更改")
                else:
                    with open(path, 'w', encoding='utf-8') as f:
                        f.write(new_content)
                    print(f"[更新] {path}: {count} 处更改")
        except Exception as e:
            print(f"[错误] {path}: {e}")

    print(f"\n总计: {total_changes} 处更改")


if __name__ == "__main__":
    main()
