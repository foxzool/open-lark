# Open-Lark Crate 模板指南

## 概述

本文档提供了创建Open-Lark业务模块Crate的标准模板和最佳实践。

## 标准Crate结构

### 目录结构

```
crates/openlark-{domain}/
├── Cargo.toml               # 模块配置文件
├── README.md                # 模块说明文档
├── src/
│   ├── lib.rs               # 库入口文件
│   ├── models/              # 共享数据模型
│   │   ├── mod.rs
│   │   ├── common.rs        # 通用模型定义
│   │   ├── requests.rs      # 请求模型
│   │   └── responses.rs     # 响应模型
│   └── {project}/           # 按项目分组
│       ├── {version}/       # 按版本分组
│       │   ├── mod.rs
│       │   ├── {resource}.rs # 具体资源API实现
│       │   └── models.rs    # 版本特定模型
│       └── models.rs        # 项目级模型定义
├── tests/                   # 模块测试
│   ├── unit/               # 单元测试
│   │   ├── mod.rs
│   │   ├── models_tests.rs
│   │   ├── client_tests.rs
│   │   └── api_tests.rs
│   ├── integration/        # 集成测试
│   │   ├── mod.rs
│   │   ├── service_tests.rs
│   │   └── workflow_tests.rs
│   └── e2e/               # 端到端测试
│       ├── mod.rs
│       ├── full_workflow.rs
│       └── error_handling.rs
└── examples/               # 使用示例
    ├── basic_usage.rs
    ├── advanced_usage.rs
    └── error_handling.rs
```

## Cargo.toml 模板

### 基础配置模板

```toml
[package]
name = "openlark-{domain}"
version = "0.1.0"
edition = "2021"
authors = ["Open-Lark Team <team@open-lark.com>"]
description = "{业务模块描述} for Open-Lark SDK"
license = "MIT OR Apache-2.0"
repository = "https://github.com/open-lark/open-lark"
homepage = "https://open-lark.com"
documentation = "https://docs.rs/openlark-{domain}"
keywords = ["lark", "feishu", "{domain}", "sdk", "api"]
categories = ["api-bindings", "web-programming"]
readme = "README.md"
rust-version = "1.75"

[dependencies]
# 核心依赖
openlark-core = { workspace = true }
openlark-protocol = { workspace = true, optional = true }

# 序列化相关
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# HTTP客户端
reqwest = { workspace = true }

# 错误处理
thiserror = { workspace = true }
anyhow = { workspace = true }

# 异步运行时
tokio = { workspace = true, features = ["rt", "rt-multi-thread", "macros"] }

# 时间处理
chrono = { workspace = true, features = ["serde"] }

# UUID处理
uuid = { workspace = true, features = ["v4", "serde"] }

# URL处理
url = { workspace = true, features = ["serde"] }

# 正则表达式
regex = { workspace = true }

# 日志记录
log = { workspace = true }
tracing = { workspace = true }

# 可选依赖
async-trait = { version = "0.1", optional = true }

[dev-dependencies]
# 测试框架
tokio-test = "0.4"
wiremock = "0.6"
mockall = { workspace = true }
rstest = { workspace = true }
tempfile = { workspace = true }
tracing-test = { workspace = true }

# 测试工具
serde_test = "1.0"
pretty_assertions = "1.0"

[features]
default = []

# WebSocket支持
websocket = ["openlark-protocol"]

# 异步特征支持
async = ["async-trait"]

# 开发模式功能
dev = []

# 测试模式功能
test = []

[lints.rust]
unsafe_code = "forbid"
unused_extern_crates = "warn"
unused_import_braces = "warn"
unused_qualifications = "warn"

[lints.clippy]
all = "warn"
nursery = "warn"
pedantic = "warn"
```

### Feature配置说明

**必须定义的features**:
- `default`: 默认功能集，通常为空
- `websocket`: WebSocket支持，依赖openlark-protocol
- `async`: 异步特征支持

**可选features**:
- `dev`: 开发专用功能
- `test`: 测试专用功能

## 源代码模板

### lib.rs 模板

```rust
//! # Open-Lark {业务模块名称} 模块
//!
//! 本模块提供飞书开放平台{业务模块描述}相关的API接口。
//!
//! ## 功能特性
//!
//! - {功能特性1}
//! - {功能特性2}
//! - {功能特性3}
//!
//! ## 快速开始
//!
//! ```rust,no_run
//! use openlark_client::LarkClient;
//! use openlark_{domain}::{ServiceName, RequestType};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .feature("{domain}")
//!         .build()?;
//!
//!     let service = client.{domain}();
//!     let response = service.{project}.{version}.{api_name}(request).await?;
//!
//!     println!("Response: {:?}", response);
//!     Ok(())
//! }
//! ```

// 模块声明
pub mod models;

// 导入所有项目模块
// pub mod {project1};
// pub mod {project2};

// 重新导出常用类型
pub use models::*;

use openlark_core::{Config, SDKResult};

/// {业务模块名称}服务
///
/// 提供{业务模块描述}相关的所有API功能。
///
/// # 示例
///
/// ```rust
/// use openlark_{domain}::{ServiceName};
/// use openlark_core::Config;
///
/// let config = Config::new("app_id", "app_secret");
/// let service = ServiceName::new(config);
/// ```
pub struct {ServiceName} {
    pub config: Config,
}

impl {ServiceName} {
    /// 创建新的服务实例
    ///
    /// # 参数
    ///
    /// * `config` - Open-Lark配置对象
    ///
    /// # 示例
    ///
    /// ```rust
    /// use openlark_{domain}::{ServiceName};
    /// use openlark_core::Config;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = ServiceName::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取{project1}服务
    pub fn {project1}(&self) -> {project1}::{ProjectName}Service {
        {project1}::{ProjectName}Service::new(self.config.clone())
    }

    /// 获取{project2}服务
    pub fn {project2}(&self) -> {project2}::{ProjectName}Service {
        {project2}::{ProjectName}Service::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let config = Config::new("test_app_id", "test_app_secret");
        let service = {ServiceName}::new(config);
        assert_eq!(service.config.app_id, "test_app_id");
    }
}
```

### 项目模块模板

```rust
// src/{project}/mod.rs
//! {项目描述}模块

pub mod v1;
pub mod v2;
pub mod v3;
pub mod models;

use openlark_core::{Config, SDKResult};

/// {项目名称}服务
pub struct {ProjectName}Service {
    pub config: Config,
}

impl {ProjectName}Service {
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取v1版本API服务
    pub fn v1(&self) -> v1::{ProjectName}V1Service {
        v1::{ProjectName}V1Service::new(self.config.clone())
    }

    /// 获取v2版本API服务
    pub fn v2(&self) -> v2::{ProjectName}V2Service {
        v2::{ProjectName}V2Service::new(self.config.clone())
    }

    /// 获取v3版本API服务
    pub fn v3(&self) -> v3::{ProjectName}V3Service {
        v3::{ProjectName}V3Service::new(self.config.clone())
    }
}
```

### 版本化API服务模板

```rust
// src/{project}/v1/mod.rs
//! {项目名称} v1版本API

pub mod {resource1};
pub mod {resource2};
pub mod models;

use openlark_core::{Config, SDKResult};

/// {项目名称} v1版本服务
pub struct {ProjectName}V1Service {
    pub config: Config,
}

impl {ProjectName}V1Service {
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取{resource1}相关API
    pub fn {resource1}(&self) -> {resource1}::{ResourceName}Service {
        {resource1}::{ResourceName}Service::new(self.config.clone())
    }

    /// 获取{resource2}相关API
    pub fn {resource2}(&self) -> {resource2}::{ResourceName}Service {
        {resource2}::{ResourceName}Service::new(self.config.clone())
    }
}
```

### API实现模板

```rust
// src/{project}/v1/{resource}.rs
//! {资源名称}相关API实现

use openlark_core::{Config, SDKResult, HttpClient};
use crate::models::{
    {ResourceName}CreateRequestV1,
    {ResourceName}CreateResponseV1,
    {ResourceName}GetRequestV1,
    {ResourceName}GetResponseV1,
    {ResourceName},
};

/// {资源名称}服务
pub struct {ResourceName}Service {
    config: Config,
}

impl {ResourceName}Service {
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建{资源名称}
    ///
    /// 创建新的{资源名称}实例。
    ///
    /// # 参数
    ///
    /// * `request` - 创建{资源名称}的请求参数
    ///
    /// # 返回值
    ///
    /// 返回创建成功的{资源名称}信息
    ///
    /// # 错误
    ///
    /// * `SDKError::Authentication` - 认证失败
    /// * `SDKError::InvalidRequest` - 请求参数错误
    /// * `SDKError::Network` - 网络请求失败
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use openlark_{domain}::models::{ResourceName}CreateRequestV1;
    ///
    /// let request = {ResourceName}CreateRequestV1 {
    ///     name: "示例{资源名称}".to_string(),
    ///     // 其他必需字段...
    /// };
    ///
    /// let response = service.{resource}_create(request).await?;
    /// println!("创建成功，ID: {}", response.{resource}.id);
    /// ```
    pub async fn create(&self, request: {ResourceName}CreateRequestV1) -> SDKResult<{ResourceName}CreateResponseV1> {
        let client = HttpClient::new(&self.config)?;
        let url = format!("{}/{}/v1/{resource}s", self.config.api_base_url, "{project}");

        let response = client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        let result: {ResourceName}CreateResponseV1 = response.json().await?;
        Ok(result)
    }

    /// 获取{资源名称}
    ///
    /// 根据ID获取{资源名称}详细信息。
    ///
    /// # 参数
    ///
    /// * `request` - 获取{资源名称}的请求参数
    ///
    /// # 返回值
    ///
    /// 返回{资源名称}的详细信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use openlark_{domain}::models::{ResourceName}GetRequestV1;
    ///
    /// let request = {ResourceName}GetRequestV1 {
    ///     {resource}_id: "res_123".to_string(),
    /// };
    ///
    /// let response = service.{resource}_get(request).await?;
    /// println!("{资源名称}名称: {}", response.{resource}.name);
    /// ```
    pub async fn get(&self, request: {ResourceName}GetRequestV1) -> SDKResult<{ResourceName}GetResponseV1> {
        let client = HttpClient::new(&self.config)?;
        let url = format!(
            "{}/{}/v1/{resource}s/{}",
            self.config.api_base_url,
            "{project}",
            request.{resource}_id
        );

        let response = client
            .get(&url)
            .send()
            .await?;

        let result: {ResourceName}GetResponseV1 = response.json().await?;
        Ok(result)
    }
}
```

## 测试模板

### 单元测试模板

```rust
// tests/unit/api_tests.rs
use openlark_core::Config;
use openlark_{domain}::{ServiceName};
use openlark_{domain}::models::{
    {ResourceName}CreateRequestV1,
    {ResourceName}GetRequestV1,
};

#[tokio::test]
async fn test_{resource}_create_request_serialization() {
    let request = {ResourceName}CreateRequestV1 {
        name: "测试{资源名称}".to_string(),
        // 其他字段...
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("测试{资源名称}"));
}

#[tokio::test]
async fn test_service_creation() {
    let config = Config::new("test_app_id", "test_app_secret");
    let service = ServiceName::new(config);

    assert_eq!(service.config.app_id, "test_app_id");
    assert_eq!(service.config.app_secret, "test_app_secret");
}

#[tokio::test]
async fn test_{resource}_get_request_validation() {
    let request = {ResourceName}GetRequestV1 {
        {resource}_id: "".to_string(), // 空ID应该无效
    };

    // 测试参数验证逻辑
    assert!(request.{resource}_id.is_empty());
}
```

### 集成测试模板

```rust
// tests/integration/service_tests.rs
use openlark_client::LarkClient;
use openlark_{domain}::{ServiceName};
use openlark_{domain}::models::{
    {ResourceName}CreateRequestV1,
    {ResourceName}GetRequestV1,
};

#[tokio::test]
async fn test_full_{resource}_workflow() {
    // 设置测试客户端
    let client = LarkClient::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .feature("{domain}")
        .build()
        .unwrap();

    let service = client.{domain}();
    let {project}_service = service.{project}();
    let {resource}_service = {project}_service.v1().{resource}();

    // 创建{资源名称}
    let create_request = {ResourceName}CreateRequestV1 {
        name: "测试{资源名称}".to_string(),
        // 其他必需字段...
    };

    // 注意：这里使用mock服务器进行测试
    let create_response = {resource}_service.create(create_request).await.unwrap();
    assert!(!create_response.{resource}.id.is_empty());

    // 获取{资源名称}
    let get_request = {ResourceName}GetRequestV1 {
        {resource}_id: create_response.{resource}.id.clone(),
    };

    let get_response = {resource}_service.get(get_request).await.unwrap();
    assert_eq!(get_response.{resource}.name, "测试{资源名称}");
    assert_eq!(get_response.{resource}.id, create_response.{resource}.id);
}
```

## 示例代码模板

### 基础使用示例

```rust
// examples/basic_usage.rs
//! 基础使用示例

use openlark_client::LarkClient;
use openlark_{domain}::{ServiceName};
use openlark_{domain}::models::{ResourceName}CreateRequestV1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化环境变量
    dotenvy::dotenv().ok();
    env_logger::init();

    // 创建客户端
    let client = LarkClient::builder()
        .app_id(std::env::var("APP_ID")?)
        .app_secret(std::env::var("APP_SECRET")?)
        .feature("{domain}")
        .build()?;

    // 获取服务
    let service = client.{domain}();
    let {resource}_service = service.{project}().v1().{resource}();

    // 创建{资源名称}
    let request = {ResourceName}CreateRequestV1 {
        name: "示例{资源名称}".to_string(),
        // 其他字段...
    };

    match {resource}_service.create(request).await {
        Ok(response) => {
            println!("✅ 创建成功");
            println!("ID: {}", response.{resource}.id);
            println!("名称: {}", response.{resource}.name);
        }
        Err(e) => {
            eprintln!("❌ 创建失败: {}", e);
        }
    }

    Ok(())
}
```

## README.md 模板

```markdown
# Open-Lark {业务模块名称} 模块

飞书开放平台{业务模块描述}相关的Rust SDK模块。

## 功能特性

- ✅ {功能特性1}
- ✅ {功能特性2}
- ✅ {功能特性3}
- ✅ 完整的错误处理
- ✅ 类型安全的API接口
- ✅ 异步/等待支持

## 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
open-lark = { version = "0.15", features = ["{domain}"] }
```

## 快速开始

```rust,no_run
use openlark_client::LarkClient;
use openlark_{domain}::{ServiceName};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .feature("{domain}")
        .build()?;

    let service = client.{domain}();

    // 使用服务...

    Ok(())
}
```

## API参考

详细的API文档请参考：
- [官方API文档](https://open.feishu.cn/document/)
- [模块文档](https://docs.rs/openlark-{domain})

## 示例

查看 `examples/` 目录获取更多使用示例：

- `basic_usage.rs` - 基础使用示例
- `advanced_usage.rs` - 高级功能示例
- `error_handling.rs` - 错误处理示例

## 运行示例

```bash
# 设置环境变量
export APP_ID="your_app_id"
export APP_SECRET="your_app_secret"

# 运行基础示例
cargo run --example basic_usage --features "{domain}"
```

## 开发

### 运行测试

```bash
# 运行所有测试
cargo test -p openlark-{domain}

# 运行特定测试
cargo test -p openlark-{domain} test_name
```

### 代码格式化

```bash
cargo fmt -p openlark-{domain}
```

### 代码检查

```bash
cargo clippy -p openlark-{domain}
```

## 许可证

本项目采用 MIT 或 Apache-2.0 双重许可证。
```

## 开发工具

### API分类工具

```python
# tools/classify_apis.py
import csv
import json
from collections import defaultdict
from typing import Dict, List, Tuple

def classify_apis(csv_file: str) -> Dict:
    """
    分析API数据并按业务领域分类
    """
    apis = []
    with open(csv_file, 'r', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        for row in reader:
            apis.append(row)

    # 按bizTag分类
    by_biztag = defaultdict(list)
    for api in apis:
        biztag = api.get('bizTag', 'unknown')
        if biztag:
            by_biztag[biztag].append(api)

    # 按meta.Project分类
    by_project = defaultdict(list)
    for api in apis:
        project = api.get('meta.Project', 'unknown')
        if project:
            by_project[project].append(api)

    # 按版本分类
    by_version = defaultdict(list)
    for api in apis:
        version = api.get('meta.Version', 'unknown')
        if version:
            by_version[version].append(api)

    return {
        'total_apis': len(apis),
        'by_biztag': dict(by_biztag),
        'by_project': dict(by_project),
        'by_version': dict(by_version),
    }

def generate_module_mapping(classification: Dict) -> str:
    """生成模块映射报告"""
    report = []
    report.append("# API分类报告\n")
    report.append(f"总API数量: {classification['total_apis']}\n")

    report.append("## 按bizTag分类\n")
    for biztag, apis in sorted(classification['by_biztag'].items(),
                               key=lambda x: len(x[1]), reverse=True):
        report.append(f"- {biztag}: {len(apis)} APIs")

    report.append("\n## 按项目分类\n")
    for project, apis in sorted(classification['by_project'].items(),
                               key=lambda x: len(x[1]), reverse=True):
        report.append(f"- {project}: {len(apis)} APIs")

    report.append("\n## 按版本分类\n")
    for version, apis in sorted(classification['by_version'].items(),
                               key=lambda x: len(x[1]), reverse=True):
        report.append(f"- {version}: {len(apis)} APIs")

    return '\n'.join(report)

if __name__ == "__main__":
    classification = classify_apis("analysis/data/api_list_export.csv")
    report = generate_module_mapping(classification)

    with open("api_classification_report.md", "w", encoding="utf-8") as f:
        f.write(report)

    print("API分类报告已生成: api_classification_report.md")
```

---

遵循此模板可以确保Open-Lark项目的所有模块保持一致的结构和质量标准。