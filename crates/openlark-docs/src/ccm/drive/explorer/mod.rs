//! Drive Explorer API模块
//!
//! 提供云空间资源浏览器相关的功能，包括：
//! - 文件夹和文件的创建、删除
//! - 文档复制
//! - 文件夹内容浏览
//! - 文件夹和文件元数据获取
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::ccm::drive::explorer::{ExplorerService, CreateFileRequest};
//!
//! let service = ExplorerService::new(config);
//!
//! // 创建新的文档
//! let response = service
//!     .create_file_builder()
//!     .folder_token("folder_token_xxx")
//!     .file_type("doc")
//!     .file_name("新文档")
//!     .content("文档内容")
//!     .execute(&service)
//!     .await?;
//!
//! // 获取根文件夹信息
//! let root_meta = service.get_root_folder_meta().await?;
//! if let Some(folder) = root_meta.folder {
//!     println!("根文件夹: {:?}", folder.name);
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{CreateFileRequestBuilder, ExplorerService, GetFolderMetaRequestBuilder};
