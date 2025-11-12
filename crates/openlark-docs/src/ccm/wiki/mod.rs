#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// wiki - 知识库服务
//,
// 提供知识库相关的所有功能，包括：
// - 知识空间的创建、管理
// - Wiki节点的创建、移动、复制、删除
// - 节点内容管理
// - 知识空间成员管理
// - Wiki搜索功能
// - 节点权限管理
//,
// 覆盖16个API接口（wiki/v2: 15个，wiki/v1: 1个）
use openlark_core::prelude::*;
use openlark_core::service::ccm::wiki::v2::WikiV2Service;
use openlark_core::service::ccm::wiki::v1::WikiV1Service;
/// 知识库服务
#[derive(Debug, Clone)]
pub struct WikiService {
}

impl WikiService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v2版本API
pub mod v2;
/// v1版本API
pub mod v1;
}