/// CCM Drive Permission 数据模型定义
use serde::{Deserialize, Serialize};

use super::responses::{MemberPermittedData, MemberTransferData, PublicData};

/// 判断协作者权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPermittedResponse {
    /// 响应数据
    pub data: Option<MemberPermittedData>,
}

/// 转移拥有者响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberTransferResponse {
    /// 响应数据
    pub data: Option<MemberTransferData>,
}

/// 获取公开权限设置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicResponse {
    /// 响应数据
    pub data: Option<PublicData>,
}