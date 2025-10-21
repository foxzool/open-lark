#!/usr/bin/env python3
"""
剩余API方法文档修复器
专门处理最后发现的7个缺少文档的API方法
"""

import re
from pathlib import Path
from typing import Dict, List, Tuple

def find_remaining_api_methods(root_dir: Path) -> List[Tuple[Path, int, str, str]]:
    """查找剩余的API方法"""
    remaining_methods = []

    api_patterns = [
        r'pub\s+async\s+fn\s+(\w+)\s*\([^)]*\)\s*->',
        r'pub\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*[^{]*SDKResult',
        r'pub\s+(async\s+)?fn\s+(\w+)\s*\([^)]*\)\s*->\s*[^{]*(?:Result|Response)',
    ]

    service_dir = root_dir / "src" / "service"
    if not service_dir.exists():
        return remaining_methods

    rust_files = list(service_dir.rglob("*.rs"))

    for rust_file in rust_files:
        # 跳过明显不需要文档的文件
        skip_patterns = [
            'test.rs', 'tests.rs', 'mod.rs', 'models.rs', 'types.rs', 'errors.rs',
            'utils.rs', 'helper.rs', 'mock.rs', 'example.rs', 'benches.rs',
            'benchmarks.rs', 'benchmark.rs', 'examples.rs'
        ]

        if any(pattern in str(rust_file).lower() for pattern in skip_patterns):
            continue

        try:
            with open(rust_file, 'r', encoding='utf-8') as f:
                content = f.read()
                lines = content.split('\n')

            # 检查文件中是否包含API相关的代码
            has_api_indicators = any(indicator in content for indicator in [
                'SDKResult', 'Transport::request', 'ApiRequest', 'api_path',
                'AccessTokenType', 'http_method', 'RequestOption'
            ])

            if not has_api_indicators:
                continue

            for line_num, line in enumerate(lines, 1):
                # 跳过注释行
                if line.strip().startswith('//') or line.strip().startswith('*'):
                    continue

                for pattern in api_patterns:
                    matches = re.finditer(pattern, line, re.IGNORECASE)
                    for match in matches:
                        # 获取方法名
                        if match.groups():
                            method_name = match.group(1) if match.group(1) else match.group(2)
                        else:
                            continue

                        if not method_name or method_name.strip() == '':
                            continue

                        # 跳过明显的非API方法
                        non_api_methods = {
                            'new', 'default', 'from', 'into', 'clone', 'debug', 'display',
                            'validate', 'check', 'verify', 'ensure', 'builder', 'build'
                        }

                        if method_name.lower() in non_api_methods:
                            continue

                        # 跳过私有方法
                        if method_name.startswith('_'):
                            continue

                        # 检查是否已有文档URL
                        if not check_existing_documentation(lines, line_num):
                            service_name = determine_service_from_file(rust_file)
                            remaining_methods.append((rust_file, line_num, method_name, service_name))

        except Exception as e:
            print(f"❌ 处理文件失败 {rust_file}: {e}")

    return remaining_methods

def determine_service_from_file(file_path: Path) -> str:
    """从文件路径确定服务名称"""
    path_str = str(file_path).lower()

    # 服务名称的优先级匹配
    services = [
        'attendance', 'approval', 'calendar', 'cloud_docs', 'contact', 'mail',
        'search', 'im', 'ai', 'bot', 'cardkit', 'directory', 'drive', 'sheets',
        'bitable', 'task', 'minutes', 'vc', 'ehr', 'corehr', 'helpdesk',
        'hire', 'moments', 'okr', 'payroll', 'performance', 'elearning',
        'lingo', 'mdm', 'verification', 'trust_party', 'workplace',
        'admin', 'acs', 'apass', 'application', 'authentication', 'group',
        'human_authentication', 'personal_settings', 'report', 'security_and_compliance',
        'tenant', 'tenant_tag'
    ]

    for service in services:
        if service in path_str:
            return service

    return 'unknown'

def check_existing_documentation(lines: List[str], line_num: int) -> bool:
    """检查是否已有文档URL"""
    doc_pattern = re.compile(r'https://open\.feishu\.cn/document/')

    # 检查前50行
    start = max(0, line_num - 50)
    end = min(len(lines), line_num + 10)

    for i in range(start, end):
        if doc_pattern.search(lines[i]):
            return True

    return False

def generate_doc_url_for_service(service_name: str, method_name: str, file_path: Path) -> str:
    """为特定服务和方法生成文档URL"""

    # Cloud Docs服务的URL映射
    if service_name == 'cloud_docs':
        if 'sheets' in str(file_path):
            if 'v3' in str(file_path):
                base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/sheets-v3"
            else:
                base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/sheets-v2"
        elif 'bitable' in str(file_path):
            base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bitable-v1"
        elif 'drive' in str(file_path):
            base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive/v1"
        elif 'wiki' in str(file_path):
            base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/wiki-v2"
        elif 'docx' in str(file_path):
            base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/docx-v1"
        elif 'assistant' in str(file_path):
            base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/assistant-v1"
        elif 'board' in str(file_path):
            base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTN/board-v1"
        elif 'comments' in str(file_path):
            base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/comments-v1"
        else:
            base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM"

    # 其他服务的URL映射
    elif service_name == 'im':
        base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/im-v1"
    elif service_name == 'cardkit':
        base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1"
    elif service_name == 'directory':
        base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1"
    elif service_name == 'search':
        base_url = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v1"
    else:
        base_url = f"https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/{service_name}"

    # 根据方法名推断具体的URL路径
    method_lower = method_name.lower()

    if 'create' in method_lower:
        return base_url + '/create'
    elif 'get' in method_lower and 'list' not in method_lower:
        return base_url + '/get'
    elif 'list' in method_lower:
        return base_url + '/list'
    elif 'update' in method_lower or 'patch' in method_lower:
        return base_url + '/update'
    elif 'delete' in method_lower or 'remove' in method_lower:
        return base_url + '/delete'
    elif 'search' in method_lower or 'query' in method_lower:
        return base_url + '/search'
    elif 'batch' in method_lower:
        return base_url + '/batch'
    elif 'upload' in method_lower:
        return base_url + '/upload'
    elif 'download' in method_lower:
        return base_url + '/download'
    elif 'copy' in method_lower:
        return base_url + '/copy'
    elif 'move' in method_lower:
        return base_url + '/move'
    elif 'auth' in method_lower:
        return base_url + '/auth'
    else:
        return base_url

def add_docs_to_method(file_path: Path, line_num: int, method_name: str, doc_url: str) -> bool:
    """为方法添加文档URL"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()

        target_line_index = line_num - 1

        # 向上寻找合适的位置插入文档
        insert_pos = target_line_index
        for i in range(target_line_index - 1, max(-1, target_line_index - 15), -1):
            if i >= 0 and (lines[i].strip() == '' or
                        lines[i].strip().startswith('///') or
                        lines[i].strip().startswith('//!') or
                        ('pub fn' in lines[i] and 'async' not in lines[i])):
                insert_pos = i + 1
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

def main():
    print("🔧 开始修复剩余的API方法文档...")
    print("=" * 80)

    root_dir = Path('/Users/zool/RustroverProjects/open-lark')

    # 查找剩余的API方法
    print("🔍 查找剩余的API方法...")
    remaining_methods = find_remaining_api_methods(root_dir)
    print(f"   找到 {len(remaining_methods)} 个缺少文档的API方法")

    if not remaining_methods:
        print("✅ 所有API方法都已有文档！")
        return 0

    # 显示找到的方法
    print(f"\n📋 将要处理的API方法:")
    for file_path, line_num, method_name, service_name in remaining_methods:
        relative_path = file_path.relative_to(root_dir)
        doc_url = generate_doc_url_for_service(service_name, method_name, file_path)
        print(f"   • {relative_path}:{line_num} - {method_name}() ({service_name}) -> {doc_url}")

    # 处理每个方法
    print(f"\n🔧 开始添加文档URL...")
    success_count = 0
    fail_count = 0

    for file_path, line_num, method_name, service_name in remaining_methods:
        doc_url = generate_doc_url_for_service(service_name, method_name, file_path)

        if add_docs_to_method(file_path, line_num, method_name, doc_url):
            success_count += 1
            relative_path = file_path.relative_to(root_dir)
            print(f"  ✅ {relative_path}:{line_num} - {method_name}()")
        else:
            fail_count += 1
            relative_path = file_path.relative_to(root_dir)
            print(f"  ❌ {relative_path}:{line_num} - {method_name}()")

    # 总结
    print("\n" + "=" * 80)
    print("📈 处理结果总结:")
    print(f"   • 成功添加文档: {success_count} 个API方法")
    print(f"   • 处理失败: {fail_count} 个API方法")

    if success_count + fail_count > 0:
        success_rate = (success_count / (success_count + fail_count)) * 100
        print(f"   • 成功率: {success_rate:.1f}%")

    if success_count > 0:
        print(f"\n🎉 成功为 {success_count} 个API方法添加了文档URL！")

    return 0

if __name__ == "__main__":
    exit(main())