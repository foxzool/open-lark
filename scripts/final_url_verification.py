#!/usr/bin/env python3
"""
最终URL验证脚本
检查所有修复后的URL状态
"""

import os
import re
from pathlib import Path
from typing import Dict, List, Tuple

def find_all_doc_urls(root_dir: Path) -> List[Tuple[Path, int, str]]:
    """查找所有文档URL"""
    doc_urls = []
    url_pattern = re.compile(r'https://open\.feishu\.cn/document/[\w\-\./_]+')

    for rust_file in root_dir.rglob("*.rs"):
        if 'target' in str(rust_file) or '.git' in str(rust_file):
            continue

        try:
            with open(rust_file, 'r', encoding='utf-8') as f:
                for line_num, line in enumerate(f, 1):
                    matches = url_pattern.finditer(line)
                    for match in matches:
                        doc_urls.append((rust_file, line_num, match.group()))
        except Exception as e:
            print(f"❌ 读取文件失败 {rust_file}: {e}")

    return doc_urls

def check_url_format(url: str) -> bool:
    """检查URL格式是否正确"""
    # 基本格式检查
    if not url.startswith('https://open.feishu.cn/document/'):
        return False

    # 检查是否包含有效的路径
    path = url.replace('https://open.feishu.cn/document/', '')
    if not path or len(path) < 5:
        return False

    # 检查已知的无效格式模式
    invalid_patterns = [
        'server-docs/',
        'static_doc_v1/',
        'docs-landing/'
    ]

    # ukTMukTMukTM 重复模式是正常的，不算错误
    # 只检查明确的无效模式
    for pattern in invalid_patterns:
        if pattern in url:
            return False

    return True

def analyze_url_quality():
    """分析URL质量"""
    root_dir = Path('/Users/zool/RustroverProjects/open-lark/src')
    doc_urls = find_all_doc_urls(root_dir)

    print(f"🔍 总共找到 {len(doc_urls)} 个文档URL")

    # 按格式分类
    format_correct = 0
    format_errors = []
    duplicate_urls = {}
    url_distribution = {}

    for file_path, line_num, url in doc_urls:
        # 检查格式
        if check_url_format(url):
            format_correct += 1
        else:
            format_errors.append((file_path, line_num, url))

        # 统计重复
        if url not in duplicate_urls:
            duplicate_urls[url] = []
        duplicate_urls[url].append((file_path, line_num))

        # 按模块分布
        module = str(file_path).split('/service/')[1].split('/')[0] if '/service/' in str(file_path) else 'core'
        if module not in url_distribution:
            url_distribution[module] = 0
        url_distribution[module] += 1

    print(f"✅ 格式正确的URL: {format_correct}")
    print(f"❌ 格式错误的URL: {len(format_errors)}")

    # 显示格式错误的URL
    if format_errors:
        print("\n❌ 格式错误的URL详情:")
        for file_path, line_num, url in format_errors[:10]:  # 只显示前10个
            print(f"   {file_path}:{line_num} - {url}")

    # 显示重复URL
    duplicates = {url: refs for url, refs in duplicate_urls.items() if len(refs) > 1}
    print(f"\n🔄 重复的URL: {len(duplicates)}")

    # 显示模块分布
    print(f"\n📊 URL模块分布:")
    sorted_modules = sorted(url_distribution.items(), key=lambda x: x[1], reverse=True)
    for module, count in sorted_modules[:10]:  # 只显示前10个
        print(f"   {module}: {count} URLs")

    return format_correct, len(format_errors), len(duplicates)

def main():
    print("🚀 开始最终URL验证...")
    print("=" * 60)

    try:
        format_correct, format_errors, duplicates = analyze_url_quality()

        print("\n" + "=" * 60)
        print("📈 最终验证结果:")
        print(f"   • 格式正确: {format_correct} URLs")
        print(f"   • 格式错误: {format_errors} URLs")
        print(f"   • 重复URL: {duplicates} URLs")

        success_rate = (format_correct / (format_correct + format_errors)) * 100 if (format_correct + format_errors) > 0 else 100
        print(f"   • 成功率: {success_rate:.1f}%")

        if format_errors == 0:
            print("\n🎉 所有URL格式都已修复完成！")
        elif format_errors <= 5:
            print(f"\n✅ 接近完成！仅剩 {format_errors} 个格式错误需要修复")
        else:
            print(f"\n⚠️  仍有 {format_errors} 个URL需要修复")

    except Exception as e:
        print(f"❌ 验证过程出错: {e}")
        return 1

    return 0

if __name__ == "__main__":
    exit(main())