/// Lingo语言服务 v1 模块
///
/// 提供智能语言处理功能，包括草稿管理和词条管理

pub mod draft;
pub mod entity;
pub mod classification;
pub mod repo;
pub mod file;

// 重新导出所有构建器
pub use draft::{CreateDraftBuilder, UpdateDraftBuilder};
pub use entity::{
    CreateEntityBuilder, UpdateEntityBuilder, DeleteEntityBuilder,
    GetEntityBuilder, ListEntityBuilder,
    SearchEntityBuilder, HighlightEntityBuilder
};
pub use classification::ListClassificationBuilder;
pub use repo::ListRepoBuilder;
pub use file::{UploadFileBuilder, DownloadFileBuilder};

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

    // 草稿管理
    /// 创建草稿
    /// API文档: https://open.feishu.cn/document/lingo-v1/draft/create
    pub fn create_draft(&self) -> CreateDraftBuilder<'a> {
        CreateDraftBuilder::new(self.config)
    }

    /// 更新草稿
    /// API文档: https://open.feishu.cn/document/lingo-v1/draft/update
    pub fn update_draft(&self, draft_id: String) -> UpdateDraftBuilder<'a> {
        UpdateDraftBuilder::new(self.config, draft_id)
    }

    // 词条管理
    /// 创建免审词条
    /// API文档: https://open.feishu.cn/document/lingo-v1/entity/create
    pub fn create_entity(&self) -> CreateEntityBuilder<'a> {
        CreateEntityBuilder::new(self.config)
    }

    /// 更新免审词条
    /// API文档: https://open.feishu.cn/document/lingo-v1/entity/update
    pub fn update_entity(&self, entity_id: String) -> UpdateEntityBuilder<'a> {
        UpdateEntityBuilder::new(self.config, entity_id)
    }

    /// 删除免审词条
    /// API文档: https://open.feishu.cn/document/lingo-v1/entity/delete
    pub fn delete_entity(&self, entity_id: String) -> DeleteEntityBuilder<'a> {
        DeleteEntityBuilder::new(self.config, entity_id)
    }

    /// 获取词条详情
    /// API文档: https://open.feishu.cn/document/lingo-v1/entity/get
    pub fn get_entity(&self, entity_id: String) -> GetEntityBuilder<'a> {
        GetEntityBuilder::new(self.config, entity_id)
    }

    /// 获取词条列表
    /// API文档: https://open.feishu.cn/document/lingo-v1/entity/list
    pub fn list_entities(&self) -> ListEntityBuilder<'a> {
        ListEntityBuilder::new(self.config)
    }

    // /// 精准搜索词条
    // /// API文档: https://open.feishu.cn/document/lingo-v1/entity/match
    // pub fn match_entities(&self, query: String) -> MatchEntityBuilder<'a> {
    //     MatchEntityBuilder::new(self.config, query)
    // }

    /// 模糊搜索词条
    /// API文档: https://open.feishu.cn/document/lingo-v1/entity/search
    pub fn search_entities(&self, query: String) -> SearchEntityBuilder<'a> {
        SearchEntityBuilder::new(self.config, query)
    }

    /// 词条高亮
    /// API文档: https://open.feishu.cn/document/lingo-v1/entity/highlight
    pub fn highlight_entities(&self, text: String) -> HighlightEntityBuilder<'a> {
        HighlightEntityBuilder::new(self.config, text)
    }

    // 分类管理
    /// 获取词典分类
    /// API文档: https://open.feishu.cn/document/lingo-v1/classification/list
    pub fn list_classifications(&self) -> ListClassificationBuilder<'a> {
        ListClassificationBuilder::new(self.config)
    }

    // 词库管理
    /// 获取词库列表
    /// API文档: https://open.feishu.cn/document/lingo-v1/repo/list
    pub fn list_repos(&self) -> ListRepoBuilder<'a> {
        ListRepoBuilder::new(self.config)
    }

    // 文件管理
    /// 上传图片
    /// API文档: https://open.feishu.cn/document/lingo-v1/file/upload
    pub fn upload_file(&self, file_path: String) -> UploadFileBuilder<'a> {
        UploadFileBuilder::new(self.config, file_path)
    }

    /// 下载图片
    /// API文档: https://open.feishu.cn/document/lingo-v1/file/download
    pub fn download_file(&self, file_token: String) -> DownloadFileBuilder<'a> {
        DownloadFileBuilder::new(self.config, file_token)
    }
}