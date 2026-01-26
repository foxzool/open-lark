/// CCM Drive Permission 数据模型定义
///
/// 此模块包含权限相关的数据结构定义。
use serde::{Deserialize, Serialize};

/// 判断协作者权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPermittedResponse {
    /// 是否有权限
    pub permitted: bool,
}

/// 转移拥有者响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberTransferResponse {
    /// 是否成功
    pub success: bool,
}

/// 获取公开权限设置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicPermissionResponse {
    /// 公开权限设置
    pub public_permission: Option<PublicPermission>,
}

/// 公开权限设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicPermission {
    /// 是否公开
    pub public: bool,
    /// 权限类型
    pub permission_type: String,
}

// use super::responses::{MemberPermittedData, MemberTransferData, PublicData};

// /// 判断协作者权限响应
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MemberPermittedResponse {
//     /// 响应数据
//     pub data: Option<MemberPermittedData>,
// }
//
// /// 转移拥有者响应
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MemberTransferResponse {
//     /// 响应数据
//     pub data: Option<MemberTransferData>,
// }
//
// /// 获取公开权限设置响应
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PublicResponse {
//     /// 响应数据
//     pub data: Option<PublicData>,
// }
