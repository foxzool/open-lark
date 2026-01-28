//! 知识库分类模块
//!
//! 提供客服知识库分类相关的 API。

pub mod list;
pub mod create;
pub mod get;
pub mod patch;
pub mod delete;

use openlark_core::config::Config;
use std::sync::Arc;

/// 知识库分类服务
#[derive(Clone)]
pub struct Category {
    config: Arc<Config>,
}

impl Category {
    /// 创建新的知识库分类服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取知识库分类列表
    pub fn list(&self) -> list::ListCategoryRequest {
        list::ListCategoryRequest::new(self.config.clone())
    }

    /// 创建知识库分类
    pub fn create(&self) -> create::CreateCategoryRequest {
        create::CreateCategoryRequest::new(self.config.clone())
    }

    /// 获取指定知识库分类
    pub fn get(&self, id: impl Into<String>) -> get::GetCategoryRequest {
        get::GetCategoryRequest::new(self.config.clone(), id.into())
    }

    /// 更新指定知识库分类
    pub fn patch(&self, id: impl Into<String>) -> patch::PatchCategoryRequest {
        patch::PatchCategoryRequest::new(self.config.clone(), id.into())
    }

    /// 删除指定知识库分类
    pub fn delete(&self, id: impl Into<String>) -> delete::DeleteCategoryRequest {
        delete::DeleteCategoryRequest::new(self.config.clone(), id.into())
    }
}

pub use list::{ListCategoryRequest, ListCategoryRequestBuilder};
pub use create::{CreateCategoryRequest, CreateCategoryRequestBuilder};
pub use get::{GetCategoryRequest, GetCategoryRequestBuilder};
pub use patch::{PatchCategoryRequest, PatchCategoryRequestBuilder};
pub use delete::{DeleteCategoryRequest, DeleteCategoryRequestBuilder};
