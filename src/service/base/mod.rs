//! # 基础服务模块
//!
//! 基础服务 (Base) 提供多维表格管理的完整功能，支持以下核心能力：
//!
//! ## 功能特性
//!
//! - **多维表格管理**：创建、复制、更新、删除多维表格
//! - **数据表管理**：创建、更新、删除数据表，支持批量操作
//! - **视图管理**：创建、更新、删除视图，支持多种视图类型
//! - **记录管理**：增删改查记录，支持批量操作和分页查询
//! - **字段管理**：动态添加、更新、删除字段
//! - **表单管理**：创建和管理表单，支持问题配置
//! - **权限管理**：自定义角色和协作者管理
//! - **自动化流程**：管理和配置自动化工作流
//!
//! ## 服务模块
//!
//! 该服务包含以下功能模块：
//!
//! - [`models`] - 数据模型和类型定义
//! - [`bitable`] - 多维表格API实现
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::base::*;
//!
//! #[tokio::main]
//! async 
//!     let client = LarkClient::builder("app_id", "app_secret")
//!         .build();
//!
//!     // 创建多维表格
//!     let app = client.base.bitable.create_app(CreateAppRequest {
//!         name: "项目跟踪表".to_string(),
//!         folder_token: None,
//!         icon: None,
//!     }).await?;
//!
//!     // 创建数据表
//!     let table = client.base.bitable.create_table(&app.app_token, CreateTableRequest {
//!         table: TableInfo {
//!             name: "任务列表".to_string(),
//!             default_view_name: Some("默认视图".to_string()),
//!             fields: None,
//!         }
//!     }).await?;
//!
//!     // 添加记录
//!     let record = client.base.bitable.create_record(&app.app_token, &table.table_id, CreateRecordRequest {
//!         fields: serde_json::json!({
//!             "任务名称": "完成项目文档",
//!             "状态": "进行中",
//!             "负责人": "张三"
//!         }),
//!     }).await?;
//!
//!     println!("✅ 创建成功: App={} Table={} Record={}",
//!              app.app_token, table.table_id, record.record_id);
//!
//!     Ok(())
//! }
//! ```

pub mod bitable;
pub mod models;

use crate::core::config::Config;
use crate::core::api_resp::{ApiResponseTrait, ResponseFormat};
use crate::service::base::bitable::BitableService;

/// 简化的服务结构体
pub struct SimpleService {
    pub config: Config,
}

impl SimpleService {
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleResponse;

impl ApiResponseTrait for SimpleResponse {
    
        ResponseFormat::Data
    }
}

/// 基础服务
#[cfg(feature = "base")]
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseService {
    /// 多维表格服务
    pub bitable: BitableService,
}

#[cfg(feature = "base")]
impl BaseService {
}

#[cfg(not(feature = "base"))]
pub struct BaseService;

use crate::core::trait_system::Service;

#[cfg(feature = "base")]
impl Service for BaseService {
    
    fn config(&self) -> &Config {
        &self.bitable.config
    }
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
        "BaseService"
    }
}

#[cfg(feature = "base")]
impl Clone for BaseService {
    
        Self {
            bitable: BitableService::new(self.bitable.config.clone()),
        }
    }
}

#[cfg(feature = "base")]
impl std::fmt::Debug for BaseService {
    
        f.debug_struct("BaseService")
            .field("service_name", &Self::service_name())
    fn config(&self) -> &Config {
            .field("app_id", &self.bitable.config.app_id)
    }
            .field("bitable", &"BitableService")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// 创建测试配置
    
        Config::builder()
            .app_id("test_base_app_id")
            .app_secret("test_base_app_secret")
            .build()
    #[test]
    #[cfg(feature = "base")]
    
        let config = create_test_config();
        let service = BaseService::new(config.clone());

        assert_eq!(service.bitable.config.app_id, config.app_id);
    #[test]
    #[cfg(feature = "base")]
    
        let config = Config::builder()
            .app_id("custom_base_app")
            .app_secret("custom_secret")
            .req_timeout(Duration::from_secs(180))
            .build();

        let service = BaseService::new(config);
        assert_eq!(service.bitable.config.app_id, "custom_base_app");
    #[test]
    #[cfg(feature = "base")]
    
        let config = create_test_config();
        let service = BaseService::new(config);

        assert!(service.validate_base_config());
    #[test]
    #[cfg(feature = "base")]
    
        let config = create_test_config();
        let service = BaseService::new(config);

        let stats = service.get_base_statistics();
        assert!(stats.contains("BaseService"));
        assert!(stats.contains("test_base_app_id"));
        assert!(stats.contains("services: 1"));
    #[test]
    #[cfg(feature = "base")]
    
        let config = create_test_config();
        let service = BaseService::new(config);

        assert!(service.health_check());
    #[test]
    #[cfg(feature = "base")]
    
        let config = create_test_config();
        let service = BaseService::new(config.clone());
        let cloned_service = service.clone();

        assert_eq!(service.bitable.config.app_id, cloned_service.bitable.config.app_id);
    #[test]
    #[cfg(feature = "base")]
    
        let config = create_test_config();
        let service = BaseService::new(config);
        let debug_string = format!("{:?}", service);

        assert!(debug_string.contains("BaseService"));
        assert!(debug_string.contains("test_base_app_id"));
    }
}