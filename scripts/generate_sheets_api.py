#!/usr/bin/env python3
"""
Sheets API 代码生成脚本

基于模板自动生成符合 rust-api-mapper 识别标准的API实现
"""

import os
import sys
from pathlib import Path
from typing import Dict, List, Optional

class SheetsAPIGenerator:
    def __init__(self):
        self.template_path = Path("templates/sheets_api_template.rs")
        self.service_dir = Path("src/service/sheets")

        # API配置
        self.apis = [
            {
                "name": "单个范围读取",
                "method": "GET",
                "path": "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values/:range",
                "filename": "single_range_read",
                "service_name": "SingleRangeReadService",
                "request_struct": "SingleRangeReadRequest",
                "response_struct": "SingleRangeReadResponse",
                "method_name": "read_range",
                "version": "2",
                "http_method": "GET"
            },
            {
                "name": "多个范围读取",
                "method": "GET",
                "path": "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_get",
                "filename": "batch_range_read",
                "service_name": "BatchRangeReadService",
                "request_struct": "BatchRangeReadRequest",
                "response_struct": "BatchRangeReadResponse",
                "method_name": "read_ranges",
                "version": "2",
                "http_method": "GET"
            },
            {
                "name": "电子表格信息",
                "method": "GET",
                "path": "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token",
                "filename": "spreadsheet_info",
                "service_name": "SpreadsheetInfoService",
                "request_struct": "SpreadsheetInfoRequest",
                "response_struct": "SpreadsheetInfoResponse",
                "method_name": "get_spreadsheet_info",
                "version": "3",
                "http_method": "GET"
            }
        ]

    def read_template(self) -> str:
        """读取模板文件"""
        with open(self.template_path, 'r', encoding='utf-8') as f:
            return f.read()

    def generate_api_file(self, api_config: Dict) -> str:
        """生成单个API文件"""
        template = self.read_template()

        # 替换模板变量
        content = template.replace("{{ServiceName}}", api_config["name"])
        content = content.replace("{{filename}}", api_config["filename"])
        content = content.replace("{{ServiceStruct}}", api_config["service_name"])
        content = content.replace("{{RequestStruct}}", api_config["request_struct"])
        content = content.replace("{{ResponseStruct}}", api_config["response_struct"])
        content = content.replace("{{ResponseBodyStruct}}", api_config["response_struct"] + "Body")
        content = content.replace("{{method_name}}", api_config["method_name"])
        content = content.replace("{{version}}", api_config["version"])
        content = content.replace("{{HttpMethod}}", api_config["http_method"])
        content = content.replace("{{api_path}}", api_config["path"])
        content = content.replace("{{builder_param}}", "param")
        content = content.replace("{{method_name}}_builder", f"{api_config['method_name']}_builder")
        content = content.replace("{{service_struct}}", api_config["service_name"].lower())

        return content

    def write_api_file(self, version: str, filename: str, content: str):
        """写入API文件"""
        version_dir = self.service_dir / f"v{version}"
        version_dir.mkdir(parents=True, exist_ok=True)

        file_path = version_dir / f"{filename}.rs"

        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)

        print(f"✅ 生成文件: {file_path}")

    def update_mod_file(self, version: str, filename: str):
        """更新mod.rs文件"""
        version_dir = self.service_dir / f"v{version}"
        mod_file = version_dir / "mod.rs"

        # 读取现有的mod.rs内容
        if mod_file.exists():
            with open(mod_file, 'r', encoding='utf-8') as f:
                content = f.read()
        else:
            content = f"//! Sheets电子表格服务 v{version}\n\n"

        # 添加模块声明
        module_line = f"pub mod {filename};\n"
        if module_line not in content:
            # 找到合适的位置插入模块声明
            lines = content.split('\n')
            insert_pos = -1

            for i, line in enumerate(lines):
                if line.startswith("pub mod") and filename < line.split()[1].replace(';', ''):
                    insert_pos = i
                    break

            if insert_pos == -1:
                # 找到最后一个pub mod的位置
                for i, line in enumerate(lines):
                    if line.startswith("pub mod"):
                        insert_pos = i + 1

            if insert_pos == -1:
                # 如果没有找到pub mod，在空行后添加
                for i, line in enumerate(lines):
                    if line.strip() == "" and i < len(lines) - 1:
                        insert_pos = i + 1
                        break

            if insert_pos != -1:
                lines.insert(insert_pos, module_line)
                content = '\n'.join(lines)
            else:
                content += f"\n{module_line}"

        # 添加导出声明
        export_line = f"pub use {filename}::*;"
        if export_line not in content:
            # 找到导出部分
            lines = content.split('\n')
            insert_pos = -1

            for i, line in enumerate(lines):
                if line.startswith("// 重新导出所有服务类型"):
                    insert_pos = i + 1
                    break

            if insert_pos == -1:
                # 如果没有找到导出部分，在文件末尾添加
                insert_pos = len(lines)

            # 找到合适的插入位置（按字母顺序）
            for i in range(insert_pos, len(lines)):
                if lines[i].startswith("pub use") and filename < lines[i].split("::")[1].replace(';', '').replace('*', ''):
                    insert_pos = i
                    break

            lines.insert(insert_pos, f"    {export_line}")
            content = '\n'.join(lines)

        # 写回文件
        with open(mod_file, 'w', encoding='utf-8') as f:
            f.write(content)

        print(f"✅ 更新模块: {mod_file}")

    def generate_all_apis(self):
        """生成所有API文件"""
        print("🚀 开始生成Sheets API实现文件...")

        for api_config in self.apis:
            print(f"\n📝 生成 {api_config['name']} API...")

            # 生成文件内容
            content = self.generate_api_file(api_config)

            # 写入文件
            self.write_api_file(api_config["version"], api_config["filename"], content)

            # 更新mod.rs
            self.update_mod_file(api_config["version"], api_config["filename"])

            print(f"✅ {api_config['name']} API 生成完成")

    def create_usage_examples(self):
        """创建使用示例"""
        examples_dir = Path("examples/api/sheets")
        examples_dir.mkdir(parents=True, exist_ok=True)

        for api_config in self.apis:
            example_content = f'''//! {api_config['name']} API 使用示例
//!
//! 演示如何使用 {api_config['service_name']} 进行操作

use open_lark::prelude::*;
use open_lark::service::sheets::v{api_config['version']}::{api_config['service_name']}, {api_config['request_struct']};

#[tokio::main]
async fn main() -> SDKResult<()> {{
    // 创建配置
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build();

    // 创建服务实例
    let service = {api_config['service_name']}::new(config);

    // 创建请求
    let request = {api_config['request_struct']}::new(
        "your_spreadsheet_token"
        // 其他参数...
    );

    // 执行API调用
    match service.{api_config['method_name']}(request).await {{
        Ok(response) => {{
            println!("✅ {api_config['name']}成功");
            println!("响应: {{:#?}}", response);
        }}
        Err(error) => {{
            println!("❌ {api_config['name']}失败: {{}}", error);
        }}
    }}

    Ok(())
}}
'''

            example_file = examples_dir / f"{api_config['filename']}_example.rs"
            with open(example_file, 'w', encoding='utf-8') as f:
                f.write(example_content)

            print(f"✅ 创建示例: {example_file}")

def main():
    """主函数"""
    if len(sys.argv) > 1 and sys.argv[1] == "--help":
        print("用法: python generate_sheets_api.py")
        print("生成Sheets API的标准实现文件")
        return

    generator = SheetsAPIGenerator()

    try:
        # 检查模板文件是否存在
        if not generator.template_path.exists():
            print(f"❌ 模板文件不存在: {generator.template_path}")
            return 1

        # 生成所有API
        generator.generate_all_apis()

        # 创建使用示例
        generator.create_usage_examples()

        print("\n🎉 所有API文件生成完成!")
        print("\n📋 下一步操作:")
        print("1. 检查生成的文件")
        print("2. 实现具体的数据结构和方法")
        print("3. 运行 cargo check 验证编译")
        print("4. 运行 rust-api-mapper 验证识别")

        return 0

    except Exception as e:
        print(f"❌ 生成过程中出错: {e}")
        return 1

if __name__ == "__main__":
    sys.exit(main())