/// Lingo语言服务 v1
///
/// 提供智能语言处理功能。

use openlark_core::config::Config;
use openlark_core::SDKResult;

// 导出子模块
pub mod draft;
pub mod entity;
pub mod classification;
pub mod repo;
pub mod file;
pub mod search;
pub mod models;

// 重新导出所有服务类型
pub use draft::*;
pub use entity::*;
pub use classification::*;
pub use repo::*;
pub use file::*;
pub use search::*;
pub use models::*;

/// Lingo 语言服务主结构
pub struct LingoServiceV1 {
    /// 配置信息
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

    // 草稿管理
    /// 创建草稿
    /// docPath: https://open.feishu.cn/document/lingo-v1/draft/create
    pub async fn create_draft(&self, params: CreateLingoDraftParams) -> SDKResult<LingoDraft> {
        create_lingo_draft(self.config, params).await
    }

    /// 更新草稿
    /// docPath: https://open.feishu.cn/document/lingo-v1/draft/update
    pub async fn update_draft(&self, draft_id: &str, params: UpdateLingoDraftParams) -> SDKResult<LingoDraft> {
        update_lingo_draft(draft_id, self.config, params).await
    }

    // 词条管理
    /// 创建免审词条
    /// docPath: https://open.feishu.cn/document/lingo-v1/entity/create
    pub async fn create_entity(&self, params: CreateLingoEntityParams) -> SDKResult<LingoEntity> {
        create_lingo_entity(self.config, params).await
    }

    // /// 更新免审词条
    // /// docPath: https://open.feishu.cn/document/lingo-v1/entity/update
    // pub async fn update_entity(&self, entity_id: &str, params: UpdateLingoEntityParams) -> SDKResult<LingoEntity> {
    //     update_lingo_entity(self.config, entity_id, params).await
    // }
    //
    // /// 删除免审词条
    // /// docPath: https://open.feishu.cn/document/lingo-v1/entity/delete
    // pub async fn delete_entity(&self, entity_id: &str) -> SDKResult<()> {
    //     delete_lingo_entity(self.config, entity_id).await
    // }
    //
    // /// 获取词条详情
    // /// docPath: https://open.feishu.cn/document/lingo-v1/entity/get
    // pub async fn get_entity(&self, entity_id: &str) -> SDKResult<LingoEntity> {
    //     get_lingo_entity(self.config, entity_id).await
    // }
    //
    // /// 获取词条列表
    // /// docPath: https://open.feishu.cn/document/lingo-v1/entity/list
    // pub async fn list_entities(&self, params: ListLingoEntityParams) -> SDKResult<Vec<LingoEntity>> {
    //     list_lingo_entities(self.config, params).await
    // }
    //
    // /// 精准搜索词条
    // /// docPath: https://open.feishu.cn/document/lingo-v1/entity/match
    // pub async fn match_entities(&self, params: MatchLingoEntityParams) -> SDKResult<Vec<LingoEntity>> {
    //     match_lingo_entities(self.config, params).await
    // }
    //
    // /// 模糊搜索词条
    // /// docPath: https://open.feishu.cn/document/lingo-v1/entity/search
    // pub async fn search_entities(&self, params: SearchLingoEntityParams) -> SDKResult<Vec<LingoEntity>> {
    //     search_lingo_entities(self.config, params).await
    // }
    //
    // /// 词条高亮
    // /// docPath: https://open.feishu.cn/document/lingo-v1/entity/highlight
    // pub async fn highlight_entities(&self, params: HighlightLingoEntityParams) -> SDKResult<Vec<EntityHighlight>> {
    //     highlight_lingo_entities(self.config, params).await
    // }

    // 分类管理
    /// 获取词典分类
    /// docPath: https://open.feishu.cn/document/lingo-v1/classification/list
    // /// 更新免审词条
    // /// docPath: https://open.feishu.cn/document/lingo-v1/entity/update
    // pub async fn update_entity(&self, entity_id: &str, params: UpdateLingoEntityParams) -> SDKResult<LingoEntity> {
    //     update_lingo_entity(self.config, entity_id, params).await
    // }
    
    // ... (other method comments)

    // 分类管理
    /// 获取词典分类
    /// docPath: https://open.feishu.cn/document/lingo-v1/classification/list
    pub async fn list_classifications(&self) -> SDKResult<Vec<ClassificationItem>> {
        list_classification(self.config).await
    }

    // 词库管理
    /// 获取词库列表
    /// docPath: https://open.feishu.cn/document/lingo-v1/repo/list
    pub async fn list_repos(&self) -> SDKResult<Vec<RepoItem>> {
        list_repo(self.config).await
    }

    // // 文件管理
    // /// 上传图片
    // /// docPath: https://open.feishu.cn/document/lingo-v1/file/upload
    // pub async fn upload_file(&self, params: UploadLingoFileParams) -> SDKResult<FileUploadResult> {
    //     upload_lingo_file(self.config, params).await
    // }
    //
    // /// 下载图片
    // /// docPath: https://open.feishu.cn/document/lingo-v1/file/download
    // pub async fn download_file(&self, file_token: &str) -> SDKResult<Vec<u8>> {
    //     download_lingo_file(self.config, file_token).await
    // }
}