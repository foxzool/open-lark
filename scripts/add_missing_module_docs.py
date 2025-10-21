#!/usr/bin/env python3
"""
为缺少文档的模块添加URL
基于官方API数据为没有文档URL的模块添加文档
"""

import re
from pathlib import Path
from typing import Dict, List, Tuple

# 导入模块URL映射
exec(open('/Users/zool/RustroverProjects/open-lark/scripts/module_url_mapping.py').read())

def find_api_methods_without_docs(root_dir: Path, target_module: str) -> List[Tuple[Path, int, str, str]]:
    """查找特定模块中没有文档的API方法"""
    apis_without_docs = []

    # 匹配异步API方法的模式
    api_patterns = [
        r'pub\s+async\s+fn\s+(\w+)\s*\(',
        r'pub\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*SDKResult',
        r'pub\s+async\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*SDKResult'
    ]

    module_dir = root_dir / "src" / "service" / target_module
    if not module_dir.exists():
        return apis_without_docs

    for rust_file in module_dir.rglob("*.rs"):
        try:
            with open(rust_file, 'r', encoding='utf-8') as f:
                lines = f.readlines()

            for line_num, line in enumerate(lines, 1):
                # 检查是否包含API方法定义
                for pattern in api_patterns:
                    match = re.search(pattern, line)
                    if match:
                        method_name = match.group(1)

                        # 检查前后几行是否有API文档
                        has_doc = False
                        doc_url_pattern = re.compile(r'https://open\.feishu\.cn/document/')

                        # 检查前20行是否有文档URL
                        start = max(0, line_num - 20)
                        end = min(len(lines), line_num + 5)

                        for i in range(start, end):
                            if doc_url_pattern.search(lines[i]):
                                has_doc = True
                                break

                        if not has_doc:
                            apis_without_docs.append((rust_file, line_num, method_name, line))
                            break

        except Exception as e:
            print(f"❌ 读取文件失败 {rust_file}: {e}")

    return apis_without_docs

def generate_api_docs_url(module_name: str, method_name: str, module_info: Dict) -> str:
    """根据API方法名生成可能的文档URL"""
    base_url = module_info['base_url']

    # 基于常见API命名模式推断URL路径
    method_patterns = {
        # 列表相关
        'list': '/list',
        'get_list': '/list',
        'query': '/list',
        'search': '/list',

        # 获取单个资源
        'get': '/get',
        'detail': '/get',
        'info': '/get',

        # 创建
        'create': '/create',
        'add': '/create',
        'new': '/create',
        'post': '/create',

        # 更新
        'update': '/update',
        'modify': '/update',
        'edit': '/update',
        'put': '/update',
        'patch': '/patch',

        # 删除
        'delete': '/delete',
        'remove': '/delete',
        'del': '/delete',

        # 其他常见操作
        'start': '/start',
        'stop': '/stop',
        'close': '/close',
        'open': '/open',
        'send': '/send',
        'upload': '/upload',
        'download': '/download',
        'batch': '/batch',
        'sync': '/sync',
    }

    # 查找匹配的模式
    for pattern, suffix in method_patterns.items():
        if pattern in method_name.lower():
            return base_url + suffix

    # 如果没有匹配，返回基础URL
    return base_url

def add_docs_to_file(file_path: Path, line_num: int, method_name: str, doc_url: str) -> bool:
    """为API方法添加文档URL"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()

        # 找到API方法的定义行
        target_line_index = line_num - 1  # 转换为0基索引

        # 向上寻找合适的位置插入文档（通常是方法定义前的空行或注释行）
        insert_pos = target_line_index
        for i in range(target_line_index - 1, max(-1, target_line_index - 10), -1):
            if i >= 0 and (lines[i].strip() == '' or lines[i].strip().startswith('///') or lines[i].strip().startswith('//!')):
                insert_pos = i
            else:
                break

        # 准备文档注释
        doc_lines = [
            "    /// # API文档\n",
            "    ///\n",
            f"    /// {doc_url}\n",
            "\n"
        ]

        # 插入文档
        lines[insert_pos:insert_pos] = doc_lines

        # 写回文件
        with open(file_path, 'w', encoding='utf-8') as f:
            f.writelines(lines)

        return True

    except Exception as e:
        print(f"❌ 添加文档失败 {file_path}: {e}")
        return False

def process_module_docs(module_name: str, module_info: Dict) -> Tuple[int, int]:
    """处理单个模块的文档添加"""
    print(f"\n🔧 处理模块: {module_name}")
    print(f"   基础URL: {module_info['base_url']}")
    print(f"   API数量: {module_info['api_count']}")

    root_dir = Path('/Users/zool/RustroverProjects/open-lark')

    # 查找没有文档的API方法
    apis_without_docs = find_api_methods_without_docs(root_dir, module_name)

    if not apis_without_docs:
        print(f"   ✅ 所有API方法都有文档")
        return 0, 0

    print(f"   ⚠️  发现 {len(apis_without_docs)} 个缺少文档的API方法")

    success_count = 0
    fail_count = 0

    for file_path, line_num, method_name, line_content in apis_without_docs:
        # 生成文档URL
        doc_url = generate_api_docs_url(module_name, method_name, module_info)

        # 添加文档
        if add_docs_to_file(file_path, line_num, method_name, doc_url):
            success_count += 1
            relative_path = file_path.relative_to(root_dir)
            print(f"     ✅ {relative_path}:{line_num} - {method_name}() -> {doc_url}")
        else:
            fail_count += 1
            relative_path = file_path.relative_to(root_dir)
            print(f"     ❌ {relative_path}:{line_num} - {method_name}()")

    return success_count, fail_count

def main():
    print("🚀 开始为缺少文档的模块添加URL...")
    print("=" * 80)

    # 获取有官方数据的模块列表
    modules_with_data = list(MODULE_URL_MAPPING.keys())

    print(f"📊 准备处理 {len(modules_with_data)} 个模块")

    total_success = 0
    total_fail = 0
    processed_modules = 0

    # 按模块名排序处理
    for module_name in sorted(modules_with_data):
        if module_name in MODULES_WITHOUT_OFFICIAL_DATA:
            print(f"\n⏭️  跳过模块 {module_name} (没有官方数据)")
            continue

        module_info = MODULE_URL_MAPPING[module_name]
        success, fail = process_module_docs(module_name, module_info)

        total_success += success
        total_fail += fail
        processed_modules += 1

    print("\n" + "=" * 80)
    print("📈 处理结果总结:")
    print(f"   • 处理模块数: {processed_modules}")
    print(f"   • 成功添加文档: {total_success} 个API方法")
    print(f"   • 处理失败: {total_fail} 个API方法")
    print(f"   • 成功率: {(total_success / (total_success + total_fail) * 100):.1f}%" if (total_success + total_fail) > 0 else "N/A")

    if total_fail > 0:
        print(f"\n⚠️  有 {total_fail} 个API方法添加文档失败，可能需要手动处理")

    if total_success > 0:
        print(f"\n🎉 成功为 {total_success} 个API方法添加了文档URL！")

    return 0

if __name__ == "__main__":
    exit(main())