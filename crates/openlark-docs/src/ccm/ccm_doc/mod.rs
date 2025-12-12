//! CCM Doc 模块
//!
//! 旧版文档相关API实现，包含6个API：
//! - create: 创建旧版文档
//! - meta: 获取旧版文档元信息
//! - sheet_meta: 获取旧版文档中的电子表格元数据
//! - raw_content: 获取旧版文档纯文本内容
//! - content: 获取旧版文档富文本内容
//! - batch_update: 编辑旧版文档内容

use openlark_core::config::Config;

/// CCM Doc 服务
#[derive(Debug, Clone)]
pub struct CcmDocService {
    config: Config,
}

impl CcmDocService {
    /// 创建新的CCM Doc服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取V1版本API
    pub fn v1(&self) -> CcmDocV1 {
        CcmDocV1::new(self.config.clone())
    }

    /// 获取旧版版本API（兼容性保留）
    pub fn old(&self) -> crate::ccm::ccm_doc::old::v1::CcmDocOldV1 {
        crate::ccm::ccm_doc::old::v1::CcmDocOldV1::new(self.config.clone())
    }
}

/// CCM Doc V1 API访问器
#[derive(Debug, Clone)]
pub struct CcmDocV1 {
    config: Config,
}

impl CcmDocV1 {
    /// 创建新的V1 API访问器实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// 导出版本模块
pub mod old;
pub mod v1;