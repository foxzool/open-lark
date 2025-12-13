/// CCM Doc API Old V1 模块
///
/// 包含所有旧版文档相关的API实现
use openlark_core::config::Config;

/// 旧版文档服务
#[derive(Debug, Clone)]
pub struct CcmDocOldV1 {
    config: Config,
}

impl CcmDocOldV1 {
    /// 创建新的旧版文档服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 创建旧版文档
    pub fn create(&self) -> create::CreateDocumentRequest {
        create::CreateDocumentRequest::new(self.config.clone())
    }

    /// 获取旧版文档元信息
    pub fn meta(&self) -> meta::GetDocumentMetaRequest {
        meta::GetDocumentMetaRequest::new(self.config.clone())
    }

    /// 获取旧版文档中的电子表格元数据
    pub fn sheet_meta(&self) -> sheet_meta::GetSheetMetaRequest {
        sheet_meta::GetSheetMetaRequest::new(self.config.clone())
    }

    /// 获取旧版文档纯文本内容
    pub fn raw_content(&self) -> raw_content::GetDocumentRawContentRequest {
        raw_content::GetDocumentRawContentRequest::new(self.config.clone())
    }

    /// 获取旧版文档富文本内容
    pub fn content(&self) -> content::GetDocumentContentRequest {
        content::GetDocumentContentRequest::new(self.config.clone())
    }

    /// 编辑旧版文档内容
    pub fn batch_update(&self) -> batch_update::BatchUpdateDocumentRequest {
        batch_update::BatchUpdateDocumentRequest::new(self.config.clone())
    }
}

// 导出所有API模块
pub mod batch_update;
pub mod content;
pub mod create;
pub mod meta;
pub mod raw_content;
pub mod sheet_meta;
