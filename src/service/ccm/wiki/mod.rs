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
use crate::prelude::*;
use crate::service::ccm::wiki::v2::WikiV2Service;
use crate::service::ccm::wiki::v1::WikiV1Service;
/// 知识库服务
#[derive(Debug, Clone)],
pub struct WikiService {
    /// v2版本API服务
    pub v2: WikiV2Service,
    /// v1版本API服务
    pub v1: WikiV1Service,
}
impl WikiService {
    /// 创建新的知识库服务实例
pub fn new() -> Self {
        Self {
            v2: WikiV2Service::new(client.clone()),
            v1: WikiV1Service::new(client.clone()),
        }
}
}
/// v2版本API
pub mod v2;
/// v1版本API
pub mod v1;