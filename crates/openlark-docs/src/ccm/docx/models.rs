//! ccm_docs API 数据模型
//!
//! 定义云文档搜索和元数据API的数据结构。

use std::collections::HashMap;
use crate::prelude::*;
use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 搜索云文档的请求参数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchDocsRequest {
    /// 搜索关键字，支持模糊搜索
    pub search_key: String,
    /// 搜索的文档类型列表，不填则搜索所有类型
    /// 支持的文档类型：doc, sheet, slide, mindnote, docx, bitable, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_types: Option<Vec<String>>,
    /// 搜索范围，不填则搜索用户有权限的所有文档
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_scope: Option<SearchScope>,
    /// 排序规则，不填则默认按相关度排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortRule>,
    /// 分页参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量，默认20，最大100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 搜索范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SearchScope {
    /// 搜索我创建的文档
    CreatedByMe,
    /// 搜索与我共享的文档
    SharedWithMe,
    /// 搜索我收藏的文档
    Starred,
    /// 搜索最近访问的文档
    RecentlyViewed,
    /// 搜索指定文件夹下的文档
    Folder(String),
}

/// 排序规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SortRule {
    /// 排序字段
    /// 支持的字段：created_time, modified_time, name, size
    pub sort_field: String,
    /// 排序方向：asc升序，desc降序
    pub sort_direction: SortDirection,
}

/// 排序方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    /// 升序
    Asc,
    /// 降序
    Desc,
}

/// 搜索云文档的响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchDocsResponse {
    /// 搜索结果列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DocumentItem>>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页令牌，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchDocsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文档信息项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentItem {
    /// 文档token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 文档标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文档类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
    /// 文档创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 文档修改时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 文档创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserInfo>,
    /// 文档拥有者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserInfo>,
    /// 文档所属文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_token: Option<String>,
    /// 文档所属文件夹名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    /// 文档大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 文档URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 是否收藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_starred: Option<bool>,
    /// 文档权限信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<DocumentPermissions>,
    /// 文档扩展属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// 文档权限信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentPermissions {
    /// 是否可查看
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view: Option<bool>,
    /// 是否可编辑
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    /// 是否可评论
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_comment: Option<bool>,
    /// 是否可分享
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_share: Option<bool>,
    /// 是否可下载
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_download: Option<bool>,
}

/// 获取文档元数据的请求参数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDocMetaRequest {
    /// 文档token列表，一次最多查询100个
    pub tokens: Vec<String>,
    /// 需要返回的扩展字段，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_fields: Option<Vec<String>>,
}

/// 获取文档元数据的响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetDocMetaResponse {
    /// 文档元数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DocumentMetaItem>>,
}

impl ApiResponseTrait for GetDocMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文档元数据项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentMetaItem {
    /// 文档token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 文档基础信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_info: Option<DocumentBasicInfo>,
    /// 文档统计信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DocumentStatistics>,
    /// 文档扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// 文档基础信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentBasicInfo {
    /// 文档标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文档类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
    /// 文档创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 文档修改时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 文档创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserInfo>,
    /// 文档拥有者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserInfo>,
    /// 文档所属文件夹
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<FolderInfo>,
    /// 文档URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 文档权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<DocumentPermissions>,
}

/// 文件夹信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FolderInfo {
    /// 文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 文件夹名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 文档统计信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentStatistics {
    /// 文档大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 字数（仅适用于文档）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_count: Option<i64>,
    /// 页数（仅适用于文档、表格、幻灯片）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
    /// 表格行数（仅适用于表格）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    /// 表格列数（仅适用于表格）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i32>,
    /// 幻灯片页数（仅适用于幻灯片）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slide_count: Option<i32>,
    /// 思维导图节点数（仅适用于思维导图）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
}

// 实现参数验证trait
impl SearchDocsRequest {
    /// 验证搜索参数
    pub fn validate(&self) -> Result<(), String> {
        if self.search_key.trim().is_empty() {
            return Err("搜索关键字不能为空".to_string());
        }

        if self.search_key.len() > 1000 {
            return Err("搜索关键字长度不能超过1000个字符".to_string());
        }

        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 100 {
                return Err("每页数量必须在1-100之间".to_string());
            }
        }

        if let Some(ref doc_types) = self.doc_types {
            if doc_types.len() > 10 {
                return Err("文档类型列表长度不能超过10".to_string());
            }

            for doc_type in doc_types {
                if !Self::is_valid_doc_type(doc_type) {
                    return Err(format!("不支持的文档类型: {}", doc_type));
                }
            }
        }

        Ok(())
    }

    /// 检查文档类型是否有效
    fn is_valid_doc_type(doc_type: &str) -> bool {
        matches!(
            doc_type,
            "doc" | "sheet" | "slide" | "mindnote" | "docx" | "bitable" | "file"
        )
    }
}

impl GetDocMetaRequest {
    /// 验证元数据请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.tokens.is_empty() {
            return Err("文档token列表不能为空".to_string());
        }

        if self.tokens.len() > 100 {
            return Err("文档token列表长度不能超过100".to_string());
        }

        for token in &self.tokens {
            if token.trim().is_empty() {
                return Err("文档token不能为空".to_string());
            }
        }

        Ok(())
    }
}
