#!/usr/bin/env python3

import re
import sys

def fix_api_req_syntax(file_path):
    """修复 api_req.rs 中的语法错误"""
    print(f"修复文件: {file_path}")

    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # 备份原文件
        backup_path = f"{file_path}.backup"
        with open(backup_path, 'w', encoding='utf-8') as bf:
            bf.write(content)

        fixed_content = content

        # 修复模式1: let mut api_req = ApiRequest::default(); 后跟字段赋值
        # 找到这种模式并转换为正确的结构体初始化
        pattern1 = re.compile(
            r'let (mut )?api_req = ApiRequest::default\(\);\s*\n(\s+[a-zA-Z_][a-zA-Z0-9_]*:.*\n)*\s*}',
            re.MULTILINE
        )

        def replace_pattern1(match):
            # 提取所有字段行
            lines = []
            lines.append(match.group(0).split('\n')[0])  # let 语句
            remaining_lines = match.group(0).split('\n')[1:-1]  # 字段行（去掉最后的空行）

            # 处理字段行
            for line in remaining_lines:
                if line.strip():
                    lines.append(f'            {line.strip()}')

            # 构建正确的结构体初始化
            result = 'let ' + ('mut ' if 'mut ' in match.group(0) else '') + 'api_req = ApiRequest {\n'
            result += '\n'.join(lines[1:])  # 跳过第一个let行，从字段开始
            result += '\n        };'
            return result

        # 应用修复1
        fixed_content = pattern1.sub(replace_pattern1, fixed_content)

        # 修复模式2: 简单的 let mut api_req = ApiRequest::default(); 后面紧跟着字段赋值
        lines = fixed_content.split('\n')
        result_lines = []
        i = 0

        while i < len(lines):
            line = lines[i]

            # 检查是否是需要修复的行
            if re.match(r'\s*let (mut )?api_req = ApiRequest::default\(\);', line):
                # 查看接下来的行是否是字段赋值
                next_lines = []
                j = i + 1
                has_fields = False

                while j < len(lines) and re.match(r'\s+[a-zA-Z_][a-zA-Z0-9_]*:', lines[j]):
                    has_fields = True
                    next_lines.append(lines[j])
                    j += 1

                if has_fields:
                    # 修复这一组
                    mut_part = 'mut ' if 'mut ' in line else ''
                    result_lines.append(line.replace('ApiRequest::default();', 'ApiRequest {'))

                    # 添加字段行
                    for field_line in next_lines:
                        result_lines.append(f'            {field_line.strip()}')

                    # 添加结束部分
                    result_lines.append('        };')
                    i = j - 1  # 跳过已处理的字段行
                else:
                    result_lines.append(line)
            else:
                # 修复未闭合的结构体初始化
                if re.search(r'let (mut )?api_req = ApiRequest \{', line) and '..Default::default()' not in line:
                    # 查找这个块的结束
                    block_end = i + 1
                    brace_count = 1
                    while block_end < len(lines) and brace_count > 0:
                        if '{' in lines[block_end]:
                            brace_count += 1
                        if '}' in lines[block_end]:
                            brace_count -= 1
                        block_end += 1

                    if brace_count == 0 and block_end - 1 < len(lines):
                        # 检查结束行是否已经有 ..Default::default()
                        end_line = lines[block_end - 1]
                        if '..Default::default()' not in end_line:
                            # 在 } 前添加 ..Default::default()
                            lines[block_end - 1] = end_line.replace('}', '..Default::default()\\n        }')

                # 修复语句缺少分号
                if re.search(r'\s*}[^\s;]*$', line):
                    if ';' not in line:
                        result_lines.append(line.rstrip() + ';')
                    else:
                        result_lines.append(line)
                else:
                    result_lines.append(line)

            i += 1

        fixed_content = '\n'.join(result_lines)

        # 写入修复后的内容
        if fixed_content != content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(fixed_content)
            print(f"  ✅ 修复完成: {file_path}")
            return True
        else:
            print(f"  ⚠️  无需修复: {file_path}")
            return False

    except Exception as e:
        print(f"  ❌ 修复文件 {file_path} 时出错: {e}")
        return False

def main():
    print("=== Python ApiReq 语法修复工具 ===")

    file_path = "/Users/zool/RustroverProjects/open-lark/crates/open-lark-core/src/core/api_req.rs"

    if fix_api_req_syntax(file_path):
        print("✅ 语法修复成功")

        # 尝试编译验证
        print("验证修复结果...")
        import subprocess
        try:
            result = subprocess.run([
                'rustc', '--edition', '2021', '--crate-type', 'lib',
                file_path, '--allow', 'warnings'
            ], capture_output=True, text=True, timeout=30)

            if result.returncode == 0:
                print("✅ 编译验证成功！")
            else:
                print("❌ 仍有编译错误:")
                print(result.stderr[:1000])  # 显示前1000个字符
        except subprocess.TimeoutExpired:
            print("⚠️  编译验证超时")
        except Exception as e:
            print(f"⚠️  编译验证出错: {e}")
    else:
        print("⚠️  没有进行修复")

if __name__ == "__main__":
    main()