#!/usr/bin/env python3
"""
批量修复serde属性错误的脚本
为使用#[serde]属性但没有derive宏的结构体添加Serialize, Deserialize
"""

import os
import re
import sys

def fix_serde_attributes(file_path):
    """修复单个文件中的serde属性错误"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content

        # 模式1: 匹配使用serde属性但没有derive Serialize, Deserialize的结构体
        # 查找struct定义，前一行有#[derive(...)]但不包含Serialize或Deserialize
        # 但结构体内部有#[serde(...)]属性

        # 更简单的策略：如果文件包含#[serde(...)]且包含struct定义
        # 检查是否有struct使用了serde属性但没有derive
        if '#[serde(' in content:
            lines = content.split('\n')
            new_lines = []
            i = 0
            while i < len(lines):
                line = lines[i]

                # 检查是否是结构体定义行
                struct_match = re.match(r'^#\[derive\(([^)]+)\)\]\s*$', line)
                if struct_match and i + 1 < len(lines):
                    next_line = lines[i + 1]
                    if 'pub struct' in next_line:
                        # 检查后续几行是否有serde属性
                        has_serde_attr = False
                        for j in range(i + 2, min(i + 10, len(lines))):
                            if '#[serde(' in lines[j]:
                                has_serde_attr = True
                                break
                            elif lines[j].strip() and not lines[j].strip().startswith('///'):
                                break

                        # 如果有serde属性但derive中没有Serialize/Deserialize，则添加
                        if has_serde_attr:
                            derive_content = struct_match.group(1)
                            if 'Serialize' not in derive_content or 'Deserialize' not in derive_content:
                                # 添加缺失的trait
                                new_derive_content = derive_content
                                if 'Serialize' not in new_derive_content:
                                    if new_derive_content:
                                        new_derive_content += ', Serialize'
                                    else:
                                        new_derive_content = 'Serialize'
                                if 'Deserialize' not in new_derive_content:
                                    if new_derive_content:
                                        new_derive_content += ', Deserialize'
                                    else:
                                        new_derive_content = 'Deserialize'

                                # 替换derive行
                                line = line.replace(derive_content, new_derive_content)

                new_lines.append(line)
                i += 1

            content = '\n'.join(new_lines)

        # 模式2: 处理没有derive但有serde属性的结构体
        # 查找pub struct开头，后面有#[serde(...)]但没有derive
        lines = content.split('\n')
        new_lines = []
        i = 0

        while i < len(lines):
            line = lines[i]

            # 查找结构体定义（包括同一行的情况）
            if re.match(r'^\s*#\[derive\(.*\)\]\s*pub struct\s+\w+', line) or re.match(r'^\s*pub struct\s+\w+', line):
                # 向前查找是否有derive
                has_derive = False
                has_serialize_deserialize = False
                for j in range(i - 1, -1, -1):
                    if lines[j].strip() == '':
                        continue
                    if '#[derive(' in lines[j]:
                        has_derive = True
                        if 'Serialize' in lines[j] and 'Deserialize' in lines[j]:
                            has_serialize_deserialize = True
                        break
                    else:
                        break

                # 向后查找是否有serde属性
                has_serde_attr = False
                for j in range(i + 1, min(i + 15, len(lines))):
                    if '#[serde(' in lines[j]:
                        has_serde_attr = True
                        break
                    elif lines[j].strip() and not lines[j].strip().startswith('#[') and not lines[j].strip().startswith('///'):
                        break

                # 如果有serde属性但没有正确的derive，添加derive
                if has_serde_attr and (not has_derive or not has_serialize_deserialize):
                    # 在结构体前添加derive
                    indent = len(line) - len(line.lstrip())
                    derive_line = ' ' * indent + '#[derive(Debug, Clone, Serialize, Deserialize)]'
                    new_lines.append(derive_line)

            new_lines.append(line)
            i += 1

        content = '\n'.join(new_lines)

        # 只有内容发生变化时才写入文件
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ 修复了: {file_path}")
            return True
        else:
            print(f"⏭️  无需修复: {file_path}")
            return False

    except Exception as e:
        print(f"❌ 修复失败 {file_path}: {e}")
        return False

def main():
    if len(sys.argv) != 2:
        print("用法: python fix_serde.py <文件路径>")
        sys.exit(1)

    file_path = sys.argv[1]

    if not os.path.exists(file_path):
        print(f"❌ 文件不存在: {file_path}")
        sys.exit(1)

    fix_serde_attributes(file_path)

if __name__ == "__main__":
    main()