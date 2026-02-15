#!/usr/bin/env python3
"""批量创建 openlark-application API"""

import os

BASE = "crates/openlark-application/src/application/application/v1"

# 简化的 API 定义 - 创建基本的骨架
endpoints = [
    ("app", "create"),
    ("app", "delete"),
    ("app", "get"),
    ("app", "list"),
    ("app", "patch"),
    ("app", "update"),
    ("app/visibility", "patch"),
    ("app/visibility", "get"),
    ("app_version", "create"),
    ("app_version", "delete"),
    ("app_version", "get"),
    ("app_version", "list"),
    ("app_version", "patch"),
]

template = '''pub struct {name}Request;
pub struct {name}Response;
'''

for resource, action in endpoints:
    dir_path = os.path.join(BASE, resource)
    file_path = os.path.join(dir_path, f"{action}.rs")
    
    os.makedirs(dir_path, exist_ok=True)
    
    api_name = "".join([p.capitalize() for p in resource.replace("/", "_").split("_")]) + action.capitalize()
    
    with open(file_path, 'w') as f:
        f.write(f"//! {resource} {action}\n\n")
        f.write(f"pub struct {api_name}Request;\n")
        f.write(f"pub struct {api_name}Response;\n")
    
    print(f"✅ {file_path}")

print(f"\n✅ openlark-application API 骨架已创建")
