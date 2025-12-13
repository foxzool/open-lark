#!/usr/bin/env python3
import subprocess
import re
import os

# 获取编译错误
result = subprocess.run(
    ["cargo", "check", "-p", "openlark-docs"],
    capture_output=True,
    text=True,
    cwd="/Users/zool/RustroverProjects/open-lark"
)

# 找出需要修复的文件
files = set()
current_file = None

for line in result.stderr.split('\n'):
    if "cannot find derive macro" in line:
        # 从上一行或当前行获取文件路径
        if "-->" in line:
            file_path = line.split("-->")[1].split(":")[0].strip()
            if file_path.endswith(".rs"):
                files.add(file_path)
        elif current_file:
            files.add(current_file)
    elif "-->" in line:
        current_file = line.split("-->")[1].split(":")[0].strip()
    else:
        current_file = None

# 打印结果
for file_path in sorted(files):
    # 检查文件是否需要导入 serde
    if os.path.exists(file_path):
        with open(file_path, 'r') as f:
            content = f.read()
            if ("#\[derive.*Serialize\]" in content or "#\[derive.*Deserialize\]" in content) and \
               "use serde::" not in content and \
               "use crate::prelude" not in content:
                print(file_path)