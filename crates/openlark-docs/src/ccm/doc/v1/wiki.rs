//! Cloud Docs Wiki v1服务模块（兼容层）
//!
//! 这个模块提供了从v1到v2 Wiki服务的兼容层，允许使用v1 API接口
//! 访问v2的Wiki功能。

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

use openlark_core::config::Config;

/// 知识库服务 v2（通过v1接口访问）
#[derive(Clone, Debug)]
pub struct WikiService {
    pub config: Config,
}

impl WikiService {
    /// 创建Wiki服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 重新导出Wiki服务类型别名
pub type WikiServiceV2 = WikiService;
