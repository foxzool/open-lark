#![allow(ambiguous_glob_reexports)]

/// Lingo语言服务 v1
///
/// 提供智能语言处理功能。

use openlark_core::config::Config;

// 导出子模块
pub mod classification;
pub mod draft;
pub mod entity;
pub mod file;
pub mod models;
pub mod repo;

// 重新导出所有服务类型
pub use classification::*;
pub use draft::*;
pub use entity::*;
pub use file::*;
pub use models::*;
pub use repo::*;

/// Lingo 语言服务主结构
pub struct LingoServiceV1 {
    /// 配置信息
    #[allow(dead_code)]
    config: Config,
}

impl LingoServiceV1 {
    /// 创建新的 Lingo 服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// Lingo v1 服务
pub struct LingoV1Service<'a> {
    config: &'a openlark_core::config::Config,
}

impl<'a> LingoV1Service<'a> {
    /// 创建新的 Lingo v1 服务实例
    pub fn new(config: &'a openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &'a openlark_core::config::Config {
        self.config
    }

    // ===== 草稿管理 =====
    /// 创建草稿（返回请求对象，可继续设置 query 参数）
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/draft/create
    pub fn create_draft(&self, body: DraftEntityInput) -> CreateDraftRequest {
        CreateDraftRequest::new(self.config.clone(), body)
    }

    /// 更新草稿（返回请求对象，可继续设置 query 参数）
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/draft/update
    pub fn update_draft(&self, draft_id: impl Into<String>, body: DraftUpdateEntityInput) -> UpdateDraftRequest {
        UpdateDraftRequest::new(self.config.clone(), draft_id, body)
    }

    // ===== 词条管理 =====
    /// 创建免审词条
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/create
    pub fn create_entity(&self, body: EntityInput) -> CreateEntityRequest {
        CreateEntityRequest::new(self.config.clone(), body)
    }

    /// 更新免审词条
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/update
    pub fn update_entity(&self, entity_id: impl Into<String>, body: EntityInput) -> UpdateEntityRequest {
        UpdateEntityRequest::new(self.config.clone(), entity_id, body)
    }

    /// 删除免审词条
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/delete
    pub fn delete_entity(&self, entity_id: impl Into<String>) -> DeleteEntityRequest {
        DeleteEntityRequest::new(self.config.clone(), entity_id)
    }

    /// 获取词条详情
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/get
    pub fn get_entity(&self, entity_id: impl Into<String>) -> GetEntityRequest {
        GetEntityRequest::new(self.config.clone(), entity_id)
    }

    /// 获取词条列表
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/list
    pub fn list_entities(&self) -> ListEntityRequest {
        ListEntityRequest::new(self.config.clone())
    }

    /// 精准搜索词条
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/match
    pub fn match_entities(&self, word: impl Into<String>) -> MatchEntityRequest {
        MatchEntityRequest::new(self.config.clone(), word)
    }

    /// 模糊搜索词条
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/search
    pub fn search_entities(&self) -> SearchEntityRequest {
        SearchEntityRequest::new(self.config.clone())
    }

    /// 词条高亮
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/highlight
    pub fn highlight_entities(&self, text: impl Into<String>) -> HighlightEntityRequest {
        HighlightEntityRequest::new(self.config.clone(), text)
    }

    // ===== 分类管理 =====
    /// 获取词典分类
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/classification/list
    pub fn list_classifications(&self) -> ListClassificationRequest {
        ListClassificationRequest::new(self.config.clone())
    }

    // ===== 词库管理 =====
    /// 获取词库列表
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/repo/list
    pub fn list_repos(&self) -> ListRepoRequest {
        ListRepoRequest::new(self.config.clone())
    }

    // ===== 文件管理 =====
    /// 上传图片
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/file/upload
    /// doc: https://open.feishu.cn/document/lingo-v1/file/upload
    pub fn upload_file(&self, name: impl Into<String>, file: Vec<u8>) -> UploadFileRequest {
        UploadFileRequest::new(self.config.clone(), name, file)
    }

    /// 下载图片（返回二进制 bytes）
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/file/download
    /// doc: https://open.feishu.cn/document/lingo-v1/file/download
    pub fn download_file(&self, file_token: impl Into<String>) -> DownloadFileRequest {
        DownloadFileRequest::new(self.config.clone(), file_token)
    }
}
