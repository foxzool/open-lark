use serde::{Deserialize, Serialize};

/// 企业信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Tenant {
    /// 企业名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 企业显示名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 企业头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<TenantAvatar>,
    /// 企业 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

/// 企业头像信息
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TenantAvatar {
    /// 头像 72x72 像素
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_72: Option<String>,
    /// 头像 240x240 像素
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_240: Option<String>,
    /// 头像 640x640 像素
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_640: Option<String>,
    /// 原始尺寸头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_origin: Option<String>,
}

/// 企业席位信息
#[derive(Debug, Serialize, Deserialize)]
pub struct TenantProductAssignInfo {
    /// 企业内席位总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_seat_count: Option<i32>,
    /// 已分配席位数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_seat_count: Option<i32>,
    /// 历史最大分配席位数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_assigned_seat_count: Option<i32>,
    /// 购买时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_time: Option<String>,
    /// 到期时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 产品名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// 服务状态。可能值有：
    /// - trial: 试用
    /// - paid: 付费
    /// - expired: 已过期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_status: Option<String>,
}
