//! Baike 知识库 v1 服务入口
//!
//! 说明：API 实现位于 `crates/openlark-docs/src/baike/baike/v1/**`。
//! 这里仅聚合服务方法，返回具体的请求对象（可继续设置 query/body 等参数）。

use crate::baike::baike::v1::{
    classification::ListClassificationRequest,
    draft::{CreateDraftReq, CreateDraftRequest, UpdateDraftReq, UpdateDraftRequest},
    entity::{
        CreateEntityReq, CreateEntityRequest, ExtractEntityRequest, GetEntityRequest,
        HighlightEntityRequest, ListEntityRequest, MatchEntityRequest, SearchEntityRequest,
        UpdateEntityReq, UpdateEntityRequest,
    },
    file::{DownloadFileRequest, UploadFileRequest},
};

/// Baike v1 服务
pub struct BaikeV1Service<'a> {
    config: &'a openlark_core::config::Config,
}

impl<'a> BaikeV1Service<'a> {
    /// 创建新的 Baike v1 服务实例
    pub fn new(config: &'a openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &'a openlark_core::config::Config {
        self.config
    }

    // ===== 草稿管理 =====
    /// 创建草稿（返回请求对象，可继续设置 query 参数）
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/draft/create
    pub fn create_draft(&self, body: CreateDraftReq) -> CreateDraftRequest {
        CreateDraftRequest::new(self.config.clone(), body)
    }

    /// 更新草稿（返回请求对象，可继续设置 query 参数）
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/draft/update
    pub fn update_draft(
        &self,
        draft_id: impl Into<String>,
        body: UpdateDraftReq,
    ) -> UpdateDraftRequest {
        UpdateDraftRequest::new(self.config.clone(), draft_id, body)
    }

    // ===== 词条管理 =====
    /// 创建免审词条（返回请求对象，可继续设置 query 参数）
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/create
    pub fn create_entity(&self, body: CreateEntityReq) -> CreateEntityRequest {
        CreateEntityRequest::new(self.config.clone(), body)
    }

    /// 更新免审词条（返回请求对象，可继续设置 query 参数）
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/update
    pub fn update_entity(
        &self,
        entity_id: impl Into<String>,
        body: UpdateEntityReq,
    ) -> UpdateEntityRequest {
        UpdateEntityRequest::new(self.config.clone(), entity_id, body)
    }

    /// 获取词条详情（返回请求对象，可继续设置 query 参数）
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/get
    pub fn get_entity(&self, entity_id: impl Into<String>) -> GetEntityRequest {
        GetEntityRequest::new(self.config.clone(), entity_id)
    }

    /// 获取词条列表（返回请求对象，可继续设置 query 参数）
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/list
    pub fn list_entities(&self) -> ListEntityRequest {
        ListEntityRequest::new(self.config.clone())
    }

    /// 精准搜索词条
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/match
    pub fn match_entities(&self, word: impl Into<String>) -> MatchEntityRequest {
        MatchEntityRequest::new(self.config.clone(), word)
    }

    /// 模糊搜索词条（返回请求对象，可继续设置 query/body 参数）
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/search
    pub fn search_entities(&self) -> SearchEntityRequest {
        SearchEntityRequest::new(self.config.clone())
    }

    /// 词条高亮
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/highlight
    pub fn highlight_entities(&self, text: impl Into<String>) -> HighlightEntityRequest {
        HighlightEntityRequest::new(self.config.clone(), text)
    }

    /// 提取潜在的词条
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/extract
    pub fn extract_entities(&self, text: impl Into<String>) -> ExtractEntityRequest {
        ExtractEntityRequest::new(self.config.clone(), text)
    }

    // ===== 分类管理 =====
    /// 获取词典分类
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/classification/list
    pub fn list_classifications(&self) -> ListClassificationRequest {
        ListClassificationRequest::new(self.config.clone())
    }

    // ===== 文件管理 =====
    /// 上传图片
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/file/upload
    pub fn upload_file(&self, name: impl Into<String>, file: Vec<u8>) -> UploadFileRequest {
        UploadFileRequest::new(self.config.clone(), name, file)
    }

    /// 下载图片
    /// docPath: https://open.feishu.cn/document/server-docs/baike-v1/file/download
    pub fn download_file(&self, file_token: impl Into<String>) -> DownloadFileRequest {
        DownloadFileRequest::new(self.config.clone(), file_token)
    }
}
