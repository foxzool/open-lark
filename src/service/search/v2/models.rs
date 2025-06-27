use serde::{Deserialize, Serialize};

/// 搜索消息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchMessageRequest {
    /// 搜索关键字
    pub query: String,
    /// 分页大小，默认20，最大200
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 搜索应用请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAppRequest {
    /// 搜索关键字
    pub query: String,
    /// 分页大小，默认20，最大200
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 搜索结果项
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResultItem {
    /// 结果ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 搜索响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    /// 搜索结果列表
    pub items: Vec<SearchResultItem>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 数据源信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DataSource {
    /// 数据源ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 数据源名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据源描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据源状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 创建数据源请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDataSourceRequest {
    /// 数据源名称
    pub name: String,
    /// 数据源描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据源配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

/// 更新数据源请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDataSourceRequest {
    /// 数据源名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据源描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据源配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

/// 数据源列表请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ListDataSourceRequest {
    /// 分页大小，默认20，最大100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 数据源列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListDataSourceResponse {
    /// 数据源列表
    pub items: Vec<DataSource>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 数据项
#[derive(Debug, Serialize, Deserialize)]
pub struct DataItem {
    /// 数据项ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 数据项属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 创建数据项请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDataItemRequest {
    /// 数据项ID
    pub id: String,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 数据项属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// 批量创建数据项请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCreateDataItemRequest {
    /// 数据项列表
    pub items: Vec<CreateDataItemRequest>,
}

/// 数据范式
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    /// 范式ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 范式名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 范式描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 范式定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<serde_json::Value>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 创建数据范式请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSchemaRequest {
    /// 范式名称
    pub name: String,
    /// 范式描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 范式定义
    pub definition: serde_json::Value,
}

/// 更新数据范式请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSchemaRequest {
    /// 范式名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 范式描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 范式定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<serde_json::Value>,
}
