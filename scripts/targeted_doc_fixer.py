#!/usr/bin/env python3
"""
针对性文档URL修复器
专门处理分析发现的缺少文档的API方法
"""

import re
from pathlib import Path
from typing import Dict, List, Tuple

# 针对特定服务和方法的URL映射
TARGETED_URL_MAPPING = {
    # contact服务
    ('contact', 'batch'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/contact-v3/user/batch',
    ('contact', 'import'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/contact-v3/user/import',
    ('contact', 'export'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/contact-v3/user/export',

    # im服务
    ('im', 'delete'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/delete',

    # search服务
    ('search', 'next'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v1/user/search',

    # authentication服务
    ('authentication', 'login'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/authentication/user/login',
    ('authentication', 'logout'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/authentication/user/logout',

    # cloud_docs (权限管理)
    ('cloud_docs', 'update_result'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive-v1/permission/update',
    ('cloud_docs', 'password_info'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive-v1/permission/password',
    ('cloud_docs', 'deletion_info'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive-v1/permission/delete',
}

# 基础URL映射
BASE_URL_MAPPING = {
    'contact': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/contact-v3',
    'im': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/im-v1',
    'search': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v1',
    'authentication': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/authentication',
    'cloud_docs': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive/v1',
}

def find_target_api_methods(root_dir: Path) -> List[Tuple[Path, int, str, str]]:
    """查找特定的缺少文档的API方法"""
    target_methods = []

    # 根据分析结果，重点检查这些文件
    target_files = [
        'src/service/authentication/v1/auth.rs',
        'src/service/cloud_docs/permission/public_v2/patch.rs',
        'src/service/cloud_docs/permission/public_v1/patch.rs',
        'src/service/cloud_docs/permission/public_v1/password/create.rs',
        'src/service/cloud_docs/permission/public_v1/password/delete.rs',
        'src/service/cloud_docs/permission/public_v1/password/update.rs',
        'src/service/im/v1/message/send.rs',
        'src/service/search/v1/user.rs',
    ]

    # 查找contact服务中的方法
    contact_dir = root_dir / "src" / "service" / "contact"
    if contact_dir.exists():
        for rs_file in contact_dir.rglob("*.rs"):
            if 'models.rs' not in str(rs_file) and 'mod.rs' not in str(rs_file):
                target_files.append(str(rs_file.relative_to(root_dir)))

    api_patterns = [
        r'pub\s+async\s+fn\s+(\w+)\s*\([^)]*\)\s*->',
        r'pub\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*[^{]*SDKResult',
    ]

    for relative_file_path in target_files:
        file_path = root_dir / relative_file_path
        if not file_path.exists():
            continue

        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()

            for line_num, line in enumerate(lines, 1):
                for pattern in api_patterns:
                    matches = re.finditer(pattern, line)
                    for match in matches:
                        method_name = match.group(1)

                        # 跳过私有方法和非API方法
                        if (method_name.startswith('_') or
                            method_name in ['new', 'default', 'from', 'into', 'validate']):
                            continue

                        # 检查是否已有文档
                        has_doc = check_existing_doc(lines, line_num)
                        if not has_doc:
                            service_name = determine_service_from_file(file_path)
                            target_methods.append((file_path, line_num, method_name, service_name))

        except Exception as e:
            print(f"❌ 处理文件失败 {file_path}: {e}")

    return target_methods

def determine_service_from_file(file_path: Path) -> str:
    """确定服务名称"""
    path_str = str(file_path)

    if 'contact' in path_str:
        return 'contact'
    elif 'im' in path_str:
        return 'im'
    elif 'search' in path_str:
        return 'search'
    elif 'authentication' in path_str:
        return 'authentication'
    elif 'cloud_docs' in path_str:
        return 'cloud_docs'

    return 'unknown'

def check_existing_doc(lines: List[str], line_num: int) -> bool:
    """检查是否已有文档URL"""
    doc_pattern = re.compile(r'https://open\.feishu\.cn/document/')

    start = max(0, line_num - 20)
    end = min(len(lines), line_num + 5)

    for i in range(start, end):
        if doc_pattern.search(lines[i]):
            return True

    return False

def generate_doc_url(service_name: str, method_name: str, file_path: Path) -> str:
    """生成文档URL"""
    # 首先尝试精确匹配
    key = (service_name, method_name)
    if key in TARGETED_URL_MAPPING:
        return TARGETED_URL_MAPPING[key]

    # 根据方法名推断
    method_lower = method_name.lower()

    if 'batch' in method_lower or 'import' in method_lower or 'export' in method_lower:
        return BASE_URL_MAPPING.get(service_name, f"https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/{service_name}") + '/user'
    elif 'delete' in method_lower:
        return BASE_URL_MAPPING.get(service_name, f"https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/{service_name}") + '/delete'
    elif 'update' in method_lower:
        return BASE_URL_MAPPING.get(service_name, f"https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/{service_name}") + '/update'
    elif 'password' in method_lower:
        return BASE_URL_MAPPING.get(service_name, f"https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive/v1") + '/permission/password'
    elif 'permission' in str(file_path):
        return BASE_URL_MAPPING.get('cloud_docs', "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive/v1") + '/permission'

    # 默认返回基础URL
    return BASE_URL_MAPPING.get(service_name, f"https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/{service_name}")

def add_docs_to_method(file_path: Path, line_num: int, method_name: str, doc_url: str) -> bool:
    """为方法添加文档"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()

        target_line_index = line_num - 1

        # 寻找合适的插入位置
        insert_pos = target_line_index
        for i in range(target_line_index - 1, max(-1, target_line_index - 10), -1):
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
    print("🎯 开始针对性API文档修复...")
    print("=" * 80)

    root_dir = Path('/Users/zool/RustroverProjects/open-lark')

    # 查找目标API方法
    print("🔍 查找目标API方法...")
    target_methods = find_target_api_methods(root_dir)
    print(f"   找到 {len(target_methods)} 个缺少文档的API方法")

    if not target_methods:
        print("✅ 所有目标API方法都已有文档！")
        return 0

    # 显示找到的方法
    print("\n📋 将要处理的API方法:")
    for file_path, line_num, method_name, service_name in target_methods:
        relative_path = file_path.relative_to(root_dir)
        print(f"   • {relative_path}:{line_num} - {method_name}() ({service_name})")

    # 处理每个方法
    print(f"\n🔧 开始添加文档URL...")
    success_count = 0
    fail_count = 0

    for file_path, line_num, method_name, service_name in target_methods:
        doc_url = generate_doc_url(service_name, method_name, file_path)

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