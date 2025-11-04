//! 地址簿服务数据模型
//!
//! 定义地址簿相关的数据结构，包括联系人、分组等。

use serde::{Deserialize, Serialize};

/// 联系人结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    /// 联系人ID
    pub id: String,
    /// 联系人姓名
    pub name: String,
    /// 电话号码
    pub phone: Option<String>,
    /// 邮箱地址
    pub email: Option<String>,
    /// 所属分组ID列表
    pub group_ids: Vec<String>,
    /// 联系人状态
    pub status: ContactStatus,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 创建联系人请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContactRequest {
    /// 联系人姓名
    pub name: String,
    /// 电话号码
    pub phone: Option<String>,
    /// 邮箱地址
    pub email: Option<String>,
    /// 所属分组ID列表
    pub group_ids: Vec<String>,
}

/// 更新联系人请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContactRequest {
    /// 联系人姓名
    pub name: String,
    /// 电话号码
    pub phone: Option<String>,
    /// 邮箱地址
    pub email: Option<String>,
    /// 所属分组ID列表
    pub group_ids: Vec<String>,
    /// 联系人状态
    pub status: Option<ContactStatus>,
}

/// 查询联系人列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListContactsRequest {
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面令牌
    pub page_token: Option<String>,
    /// 分组ID过滤
    pub group_id: Option<String>,
    /// 状态过滤
    pub status: Option<ContactStatus>,
}

/// 搜索联系人请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchContactsRequest {
    /// 搜索关键词
    pub query: String,
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面令牌
    pub page_token: Option<String>,
    /// 搜索范围
    pub search_fields: Option<Vec<ContactSearchField>>,
}

/// 查询联系人列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListContactsResponse {
    /// 联系人列表
    pub contacts: Vec<Contact>,
    /// 总数
    pub total: i32,
    /// 是否有更多数据
    pub has_more: bool,
    /// 下一页令牌
    pub next_page_token: Option<String>,
}

/// 分组结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// 分组ID
    pub id: String,
    /// 分组名称
    pub name: String,
    /// 分组描述
    pub description: Option<String>,
    /// 父分组ID
    pub parent_id: Option<String>,
    /// 分组状态
    pub status: GroupStatus,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 创建分组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    /// 分组名称
    pub name: String,
    /// 分组描述
    pub description: Option<String>,
    /// 父分组ID
    pub parent_id: Option<String>,
}

/// 更新分组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGroupRequest {
    /// 分组名称
    pub name: String,
    /// 分组描述
    pub description: Option<String>,
    /// 父分组ID
    pub parent_id: Option<String>,
    /// 分组状态
    pub status: Option<GroupStatus>,
}

/// 查询分组列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupsRequest {
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面令牌
    pub page_token: Option<String>,
    /// 父分组ID过滤
    pub parent_id: Option<String>,
}

/// 查询分组列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupsResponse {
    /// 分组列表
    pub groups: Vec<Group>,
    /// 总数
    pub total: i32,
    /// 是否有更多数据
    pub has_more: bool,
    /// 下一页令牌
    pub next_page_token: Option<String>,
}

/// 基础响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<T>,
}

/// 联系人状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContactStatus {
    /// 激活状态
    Active,
    /// 非激活状态
    Inactive,
    /// 已删除状态
    Deleted,
}

/// 分组状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupStatus {
    /// 激活状态
    Active,
    /// 非激活状态
    Inactive,
    /// 已删除状态
    Deleted,
}

/// 联系人搜索字段枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContactSearchField {
    /// 姓名字段
    Name,
    /// 电话字段
    Phone,
    /// 邮箱字段
    Email,
    /// 所有字段
    All,
}

// Default实现
impl Default for Contact {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            phone: None,
            email: None,
            group_ids: Vec::new(),
            status: ContactStatus::Active,
            created_time: None,
            updated_time: None,
        }
    }
}

impl Default for Group {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: None,
            parent_id: None,
            status: GroupStatus::Active,
            created_time: None,
            updated_time: None,
        }
    }
}

impl Default for ContactStatus {
    fn default() -> Self {
        ContactStatus::Active
    }
}

impl Default for GroupStatus {
    fn default() -> Self {
        GroupStatus::Active
    }
}

impl Default for ContactSearchField {
    fn default() -> Self {
        ContactSearchField::All
    }
}

impl Default for ListContactsRequest {
    fn default() -> Self {
        Self {
            page_size: Some(20),
            page_token: None,
            group_id: None,
            status: None,
        }
    }
}

impl Default for SearchContactsRequest {
    fn default() -> Self {
        Self {
            query: String::new(),
            page_size: Some(20),
            page_token: None,
            search_fields: Some(vec![ContactSearchField::All]),
        }
    }
}

impl Default for ListGroupsRequest {
    fn default() -> Self {
        Self {
            page_size: Some(20),
            page_token: None,
            parent_id: None,
        }
    }
}