/// CCM Doc 模块
///
/// 旧版文档相关API实现，包含6个API：
/// - create: 创建旧版文档
/// - meta: 获取旧版文档元信息
/// - sheet_meta: 获取旧版文档中的电子表格元数据
/// - raw_content: 获取旧版文档纯文本内容
/// - content: 获取旧版文档富文本内容
/// - batch_update: 编辑旧版文档内容
use openlark_core::config::Config;

/// CCM Doc 服务
#[derive(Debug, Clone)]
pub struct CcmDocService {
    config: Config,
}

impl CcmDocService {
    /// 创建新的CCM Doc服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取旧版（old）API
    pub fn old(&self) -> crate::ccm::ccm_doc::old::CcmDocOldService {
        crate::ccm::ccm_doc::old::CcmDocOldService::new(self.config.clone())
    }
}

// 导出版本模块
pub mod old;
