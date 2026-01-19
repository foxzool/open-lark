#!/usr/bin/env python3
"""
批量为新风格 API 添加 execute_with_options 方法

用法：
    python tools/add_execute_with_options.py --dry-run  # 预览
    python tools/add_execute_with_options.py            # 执行
"""

import argparse
import re
from pathlib import Path


def transform_execute_method(content: str) -> tuple[str, int]:
    """
    转换文件内容，添加 execute_with_options 方法

    处理两种风格：
    1. Builder 风格：execute(self) - 参数在 self 中
    2. 函数风格：execute(self, params: Type) - 参数通过函数传入
    """
    count = 0
    results = []

    # 匹配 execute 方法定义的函数
    def replace_fn(match):
        nonlocal count
        count += 1

        # 提取匹配的各个部分
        full_fn_def = match.group(0)  # 完整的函数定义
        fn_open = match.group(1)      # pub async fn execute(...) -> SDKResult<Type> {
        response_type = match.group(2)  # Type (响应类型)
        fn_body = match.group(3)      # 函数体（验证逻辑）
        transport_call = match.group(4)  # let response = Transport::request(..., &self.config, None).await?;
        extract_call = match.group(5)     # extract_response_data(...);
        fn_close = match.group(6)         # }

        # 检查是否有额外的参数（如 params）
        params_match = re.search(r'pub async fn execute\(\s*self\s*,\s*(\w+\s*:\s*[^)]+)\s*\)', fn_open)
        has_params = params_match is not None

        if has_params:
            # 函数风格：execute(self, params: Type)
            params_decl = params_match.group(1)  # 例如 "params: BatchUpdateParams"

            # 构建 execute_with_options，保留 params 参数
            new_execute = fn_open.replace(
                f') -> SDKResult<{response_type}> {{',
                f') -> SDKResult<{response_type}> {{\n            self.execute_with_options({params_decl.split(":")[0].strip()}, openlark_core::req_option::RequestOption::default()).await\n        }}'
            )

            new_impl = f'''{new_execute}

        pub async fn execute_with_options(
            self,
            {params_decl},
            option: openlark_core::req_option::RequestOption,
        ) -> SDKResult<{response_type}> {{
{fn_body}
            {transport_call.replace('None', 'Some(option)')}
{extract_call}
{fn_close}'''
        else:
            # Builder 风格：execute(self)
            new_execute = f'''{fn_open}
            self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
        }}'''

            new_impl = f'''{new_execute}

        pub async fn execute_with_options(
            self,
            option: openlark_core::req_option::RequestOption,
        ) -> SDKResult<{response_type}> {{
{fn_body}
            {transport_call.replace('None', 'Some(option)')}
{extract_call}
{fn_close}'''

        return new_impl

    # 匹配模式：
    #   pub async fn execute(...) -> SDKResult<Type> {
    #       ...验证逻辑...
    #       let response = Transport::request(..., &self.config, None).await?;
    #       extract_response_data(...);
    #   }
    pattern = r'(pub async fn execute\([^)]*\)\s*->\s*SDKResult<([^>]+)>\s*\{)([\s\S]*?)(let response = Transport::request\([^)]+,\s*&self\.config,\s+)None(\)\.await\?;)([\s\S]*?extract_response_data\([^)]+\)[;]?)(\s*\})'

    new_content = re.sub(pattern, replace_fn, content, flags=re.DOTALL)

    return new_content, count


def main():
    parser = argparse.ArgumentParser(description="批量添加 execute_with_options 方法")
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

    print(f"找到 {len(files)} 个需要更新的文件")

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
