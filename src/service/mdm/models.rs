use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 国家/地区信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryRegion {
    /// 主数据编码
    pub master_data_code: String,
    /// 国家/地区名称
    pub name: String,
    /// 国家/地区英文名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    /// 国家/地区代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// ISO 国家代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_country_code: Option<String>,
    /// 区域类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_type: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// 排序序号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
    /// 描述信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 用户数据维度关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthDataRelation {
    /// 关系ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_id: Option<String>,
    /// 用户ID
    pub user_id: String,
    /// 数据维度ID
    pub data_dimension_id: String,
    /// 数据维度名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_dimension_name: Option<String>,
    /// 数据维度类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_dimension_type: Option<String>,
    /// 绑定类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_type: Option<String>,
    /// 权限级别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 生效开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start_time: Option<i64>,
    /// 生效结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_end_time: Option<i64>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 数据维度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDimension {
    /// 数据维度ID
    pub data_dimension_id: String,
    /// 数据维度名称
    pub name: String,
    /// 数据维度描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据维度类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_type: Option<String>,
    /// 数据维度分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}
