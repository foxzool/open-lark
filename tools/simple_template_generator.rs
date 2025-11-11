//! 简化版服务模板生成器
//!
//! 用于快速生成标准化的飞书API服务实现模板。

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("用法: simple_template_generator <service_name> <version> <feature_flag>");
        eprintln!("示例: simple_template_generator contact v1 contact");
        std::process::exit(1);
    }

    let service_name = &args[1];
    let version = &args[2];
    let feature_flag = &args[3];

    println!("🚀 开始生成 {} 服务模板...", to_pascal_case(service_name));

    // 创建目录结构
    let base_path = format!("src/service/{}", service_name);
    let version_path = format!("{}/{}", base_path, version);

    fs::create_dir_all(&base_path).expect("创建基础目录失败");
    fs::create_dir_all(&version_path).expect("创建版本目录失败");

    println!("✅ 目录结构创建成功:");
    println!("   {}", base_path);
    println!("   {}/", version_path);

    // 生成主服务模块文件
    let main_mod_content = generate_main_mod(service_name, version);
    let main_mod_path = format!("{}/mod.rs", base_path);
    fs::write(&main_mod_path, main_mod_content).expect("写入主模块文件失败");
    println!("✅ 主模块文件创建成功: {}", main_mod_path);

    // 生成版本服务模块文件
    let version_mod_content = generate_version_mod(service_name, version);
    let version_mod_path = format!("{}/mod.rs", version_path);
    fs::write(&version_mod_path, version_mod_content).expect("写入版本模块文件失败");
    println!("✅ 版本模块文件创建成功: {}", version_mod_path);

    // 生成数据模型文件
    let models_content = generate_models();
    let models_path = format!("{}/models.rs", version_path);
    fs::write(&models_path, models_content).expect("写入模型文件失败");
    println!("✅ 数据模型文件创建成功: {}", models_path);

    // 生成示例文件
    let example_content = generate_example(service_name, version, feature_flag);
    let example_path = format!("examples/api/{}_demo.rs", feature_flag);
    fs::write(&example_path, example_content).expect("写入示例文件失败");
    println!("✅ 示例文件创建成功: {}", example_path);

    // 输出后续步骤
    println!("\n📋 生成完成！接下来的步骤：");
    println!("1. 在 Cargo.toml 中添加功能标志：");
    println!("   [features]");
    println!("   {} = []", feature_flag);
    println!("\n2. 在 src/client/mod.rs 中添加客户端集成：");
    println!("   // 添加条件导入");
    println!("   #[cfg(feature = \"{}\")]", feature_flag);
    println!(
        "   use crate::service::{}::{}Service;",
        service_name,
        to_pascal_case(service_name)
    );
    println!("\n   // 在 LarkClient 结构体中添加字段");
    println!("   #[cfg(feature = \"{}\")]", feature_flag);
    println!(
        "   pub {}: {}Service,",
        service_name,
        to_pascal_case(service_name)
    );
    println!("\n   // 在构造函数中添加初始化");
    println!("   #[cfg(feature = \"{}\")]", feature_flag);
    println!(
        "   {}: {}Service::new(config.clone()),",
        service_name,
        to_pascal_case(service_name)
    );
    println!("\n3. 在 Cargo.toml 的 [[example]] 部分添加：");
    println!("   [[example]]");
    println!("   name = \"{}_demo\"", feature_flag);
    println!("   path = \"examples/api/{}_demo.rs\"", feature_flag);
    println!("   required-features = [\"{}\"]", feature_flag);
    println!("\n4. 运行测试：");
    println!("   cargo check --features {}", feature_flag);
    println!(
        "   cargo run --example {}_demo --features {}",
        feature_flag, feature_flag
    );
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn generate_main_mod(service_name: &str, version: &str) -> String {
    let pascal_name = to_pascal_case(service_name);
    format!(
        r#"//! {}服务模块
//!
//! 提供飞书{}相关的API功能。

use crate::config::Config;

/// {}服务
#[derive(Debug, Clone)]
pub struct {}Service {{
    pub config: Config,
    pub {}_service: {}::{}Service{},
}}

impl {}Service {{
    pub fn new(config: Config) -> Self {{
        Self {{
            config,
            {}_service: {}::{}Service::new(config),
        }}
    }}
}}

pub mod {};
"#,
        pascal_name,
        "{} API服务",
        pascal_name,
        pascal_name,
        pascal_name,
        version,
        pascal_name,
        version,
        pascal_name,
        version,
        pascal_name,
        version,
        pascal_name
    )
}

fn generate_version_mod(service_name: &str, version: &str) -> String {
    let pascal_name = to_pascal_case(service_name);
    format!(
        r#"//! {} API {}版本
//!
//! 实现{}管理的核心功能。

use crate::config::Config;
use open_lark_core::prelude::*;
use serde::{{Deserialize, Serialize}};

/// {}服务 {}版本
#[derive(Debug, Clone)]
pub struct {}Service {{
    pub config: Config,
}}

impl {}Service {{
    pub fn new(config: Config) -> Self {{
        Self {{ config }}
    }}

    // ==================== 基础API操作 ====================

    /// 创建实体
    pub async fn create(&self, request: &CreateRequest) -> SDKResult<BaseResponse<Entity>> {{
        // 模拟实现
        Ok(BaseResponse {{
            code: 0,
            msg: "success".to_string(),
            data: Some(Entity {{
                id: format!("entity_{}", chrono::Utc::now().timestamp()),
                name: request.name.clone(),
                description: request.description.clone(),
                status: EntityStatus::Active,
                created_time: Some(chrono::Utc::now().to_rfc3339()),
                updated_time: Some(chrono::Utc::now().to_rfc3339()),
            }}),
        }})
    }}

    /// 获取实体详情
    pub async fn get(&self, id: &str) -> SDKResult<BaseResponse<Entity>> {{
        // 模拟实现
        Ok(BaseResponse {{
            code: 0,
            msg: "success".to_string(),
            data: Some(Entity {{
                id: id.to_string(),
                name: "示例实体".to_string(),
                description: Some("这是一个示例实体".to_string()),
                status: EntityStatus::Active,
                created_time: Some("2024-01-01T10:00:00Z".to_string()),
                updated_time: Some("2024-01-01T10:00:00Z".to_string()),
            }}),
        }})
    }}

    /// 更新实体
    pub async fn update(&self, id: &str, request: &UpdateRequest) -> SDKResult<BaseResponse<Entity>> {{
        // 模拟实现
        Ok(BaseResponse {{
            code: 0,
            msg: "success".to_string(),
            data: Some(Entity {{
                id: id.to_string(),
                name: request.name.clone(),
                description: request.description.clone(),
                status: request.status.unwrap_or(EntityStatus::Active),
                created_time: Some("2024-01-01T10:00:00Z".to_string()),
                updated_time: Some(chrono::Utc::now().to_rfc3339()),
            }}),
        }})
    }}

    /// 删除实体
    pub async fn delete(&self, id: &str) -> SDKResult<BaseResponse<bool>> {{
        // 模拟实现
        Ok(BaseResponse {{
            code: 0,
            msg: "success".to_string(),
            data: Some(true),
        }})
    }}

    /// 查询实体列表
    pub async fn list(&self, request: &ListRequest) -> SDKResult<BaseResponse<ListResponse>> {{
        // 模拟实现
        Ok(BaseResponse {{
            code: 0,
            msg: "success".to_string(),
            data: Some(ListResponse {{
                items: vec![
                    Entity {{
                        id: "item_001".to_string(),
                        name: "示例项目1".to_string(),
                        description: Some("第一个示例项目".to_string()),
                        status: EntityStatus::Active,
                        created_time: Some("2024-01-01T10:00:00Z".to_string()),
                        updated_time: Some("2024-01-01T10:00:00Z".to_string()),
                    }},
                    Entity {{
                        id: "item_002".to_string(),
                        name: "示例项目2".to_string(),
                        description: Some("第二个示例项目".to_string()),
                        status: EntityStatus::Inactive,
                        created_time: Some("2024-01-02T10:00:00Z".to_string()),
                        updated_time: Some("2024-01-02T10:00:00Z".to_string()),
                    }},
                ],
                total: 2,
                has_more: false,
                page_token: None,
            }}),
        }})
    }}
}}

// 导入子模块
pub mod models;

// 重新导出
pub use models::*;
"#,
        pascal_name, version, "{} API服务", pascal_name, pascal_name, version, pascal_name, version
    )
}

fn generate_models() -> String {
    r#"//! 服务数据模型
//!
//! 定义服务相关的数据结构。

use serde::{Deserialize, Serialize};

/// 实体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// 实体ID
    pub id: String,
    /// 实体名称
    pub name: String,
    /// 实体描述
    pub description: Option<String>,
    /// 实体状态
    pub status: EntityStatus,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 创建实体请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    /// 实体名称
    pub name: String,
    /// 实体描述
    pub description: Option<String>,
}

/// 更新实体请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRequest {
    /// 实体名称
    pub name: String,
    /// 实体描述
    pub description: Option<String>,
    /// 实体状态
    pub status: Option<EntityStatus>,
}

/// 查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequest {
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面令牌
    pub page_token: Option<String>,
    /// 过滤条件
    pub filter: Option<String>,
}

/// 列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponse {
    /// 项目列表
    pub items: Vec<Entity>,
    /// 总数
    pub total: i32,
    /// 是否有更多数据
    pub has_more: bool,
    /// 下一页令牌
    pub page_token: Option<String>,
}

/// 基础响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<T>,
}

/// 实体状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityStatus {
    /// 激活状态
    Active,
    /// 非激活状态
    Inactive,
    /// 待处理状态
    Pending,
    /// 已删除状态
    Deleted,
}

// Default实现
impl Default for Entity {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: None,
            status: EntityStatus::Active,
            created_time: None,
            updated_time: None,
        }
    }
}

impl Default for EntityStatus {
    fn default() -> Self {
        EntityStatus::Active
    }
}

impl Default for ListRequest {
    fn default() -> Self {
        Self {
            page_size: Some(20),
            page_token: None,
            filter: None,
        }
    }
}
"#
    .to_string()
}

fn generate_example(service_name: &str, version: &str, _feature_flag: &str) -> String {
    let pascal_name = to_pascal_case(service_name);
    format!(
        r#"//! {}模块使用示例
//!
//! 演示如何使用{}模块进行基础操作。

use open_lark::prelude::*;
use open_lark::service::{}::{}::{{CreateRequest, ListRequest, EntityStatus}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    // 初始化日志
    env_logger::init();

    println!("🚀 {}模块示例演示");

    // 创建客户端（这里使用测试配置）
    let client = LarkClient::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("✅ 客户端创建成功");

    // 演示创建实体
    println!("\n📝 创建实体");
    let create_request = CreateRequest {{
        name: "示例实体".to_string(),
        description: Some("这是一个通过API创建的示例实体".to_string()),
    }};

    match client.{}.{}.create(&create_request).await {{
        Ok(response) => {{
            println!("✅ 实体创建成功");
            if let Some(data) = response.data {{
                println!("   实体ID: {{}}", data.id);
                println!("   实体名称: {{}}", data.name);
                println!("   实体状态: {{:?}}", data.status);
            }}
        }}
        Err(e) => {{
            println!("❌ 实体创建失败: {{}}", e);
        }}
    }}

    // 演示获取实体详情
    println!("\n📋 获取实体详情");
    match client.{}.{}.get("entity_001").await {{
        Ok(response) => {{
            println!("✅ 实体详情获取成功");
            if let Some(data) = response.data {{
                println!("   实体ID: {{}}", data.id);
                println!("   实体名称: {{}}", data.name);
                println!("   实体描述: {{:?}}", data.description);
                println!("   实体状态: {{:?}}", data.status);
                println!("   创建时间: {{:?}}", data.created_time);
            }}
        }}
        Err(e) => {{
            println!("❌ 实体详情获取失败: {{}}", e);
        }}
    }}

    // 演示查询实体列表
    println!("\n📋 查询实体列表");
    let list_request = ListRequest {{
        page_size: Some(10),
        page_token: None,
        filter: None,
    }};

    match client.{}.{}.list(&list_request).await {{
        Ok(response) => {{
            println!("✅ 实体列表查询成功");
            if let Some(data) = response.data {{
                println!("   总数: {{}}", data.total);
                println!("   是否有更多: {{}}", data.has_more);
                for (index, item) in data.items.iter().enumerate() {{
                    println!("   [{{}}] {{}} - {{:?}}", index + 1, item.name, item.status);
                }}
            }}
        }}
        Err(e) => {{
            println!("❌ 实体列表查询失败: {{}}", e);
        }}
    }}

    // 演示更新实体
    println!("\n📝 更新实体");
    let update_request = open_lark::service::{}::{}::UpdateRequest {{
        name: "更新后的实体名称".to_string(),
        description: Some("这是更新后的描述".to_string()),
        status: Some(EntityStatus::Inactive),
    }};

    match client.{}.{}.update("entity_001", &update_request).await {{
        Ok(response) => {{
            println!("✅ 实体更新成功");
            if let Some(data) = response.data {{
                println!("   实体ID: {{}}", data.id);
                println!("   新名称: {{}}", data.name);
                println!("   新状态: {{:?}}", data.status);
            }}
        }}
        Err(e) => {{
            println!("❌ 实体更新失败: {{}}", e);
        }}
    }}

    println!("\n🎉 {}模块示例演示完成！");
    Ok(())
}}
"#,
        pascal_name,
        "{} API服务",
        service_name,
        version,
        service_name,
        version,
        service_name,
        version,
        service_name,
        version,
        service_name,
        version,
        service_name,
        version,
        pascal_name,
        service_name
    )
}
