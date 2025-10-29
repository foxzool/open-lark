//! # 基础服务模块,
//!,
//! 基础服务 (Base) 提供多维表格管理的完整功能，支持以下核心能力：,
//!,
//! ## 功能特性,
//!,
//! - **多维表格管理**：创建、复制、更新、删除多维表格,
//! - **数据表管理**：创建、更新、删除数据表，支持批量操作,
//! - **视图管理**：创建、更新、删除视图，支持多种视图类型,
//! - **记录管理**：增删改查记录，支持批量操作和分页查询,
//! - **字段管理**：动态添加、更新、删除字段,
//! - **表单管理**：创建和管理表单，支持问题配置,
//! - **权限管理**：自定义角色和协作者管理,
//! - **自动化流程**：管理和配置自动化工作流,
//!
//! ## 服务模块,
//!,
//! 该服务包含以下功能模块：,
//!,
//! - [`models`] - 数据模型和类型定义,
//! - [`bitable`] - 多维表格API实现,
//!
//! ## 使用示例,
//!,
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::base::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder("app_id", "app_secret"),
//!         .build();
//!
//!     // 创建多维表格
//!     let app = client.base.bitable.create_app(CreateAppRequest {,
//!         name: "项目跟踪表".to_string(),
//!         folder_token: None,
//!         icon: None,
//!     }).await?;
//!,
//!     // 创建数据表
//!     let table = client.base.bitable.create_table(&app.app_token, CreateTableRequest {,
//!         table: TableInfo {,
//!             name: "任务列表".to_string(),
//!             default_view_name: Some("默认视图".to_string()),
//!             fields: None,
//!         }
//!     }).await?;
//!,
//!     // 添加记录
//!     let record = client.base.bitable.create_record(&app.app_token, &table.table_id, CreateRecordRequest {,
//!         fields: serde_json::json!({,
//!             "任务名称": "完成项目文档",
//!             "状态": "进行中",
//!             "负责人": "张三"
//!         }),
//!     }).await?;
//!,
//!     println!("✅ 创建成功: App={} Table={} Record={}",
//!              app.app_token, table.table_id, record.record_id);
//!,
//!     Ok(())
//! }
//! ```,
pub mod bitable;
pub mod models;
use crate::core::config::Config;
use crate::service::base::bitable::BitableService;
/// 基础服务
#[cfg(feature = "base")]
#[derive(Debug, Clone)],
pub struct BaseService {
    /// 多维表格服务
    pub bitable: BitableService,
}
#[cfg(feature = "base")]
impl BaseService {
/// 创建新的基础服务实例
    pub fn new() -> Self {
Self {,
            bitable: BitableService::new(config.clone()),
        }
}
}
#[cfg(not(feature = "base"))],
pub struct BaseService;
#[cfg(test)]
mod tests {,
use super::*;
    #[test],
fn test_base_service_creation() {,
        let config = crate::core::config::Config::default();
let service = BaseService::new(config.clone());
        #[cfg(feature = "base")]
{,
            // 如果启用base功能，服务应该能正常创建
            assert_eq!(service.bitable.config.app_id, config.app_id);
}
    }
#[test],
    fn test_base_service_with_custom_config() {,
let config = crate::core::config::Config::builder()
            .app_id()
.app_secret()
            .build();
let service = BaseService::new(config.clone());
        #[cfg(feature = "base")]
{,
            assert_eq!(service.bitable.config.app_id, "base_test_app");
            assert_eq!(service.bitable.config.app_secret, "base_test_secret");
}
    }
}