//! docx API 数据模型
//!
//! 定义文档(DOCX)操作API的数据结构。

use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// 文档信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentInfo {
    /// 文档token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_token: Option<String>,
    /// 文档版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<i64>,
    /// 文档标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 块信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockInfo {
    /// 块ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<i64>,
    /// 块类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<i32>,
    /// 块文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// 创建文档请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateDocumentRequest {
    /// 文档标题
    pub title: String,
    /// 文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 父类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<String>,
    /// 封面key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_key: Option<String>,
}

impl CreateDocumentRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("文档标题不能为空".to_string());
        }

        if self.title.len() > 100 {
            return Err("文档标题长度不能超过100个字符".to_string());
        }

        Ok(())
    }
}

/// 创建文档响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateDocumentResponse {
    /// 文档token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_token: Option<String>,
    /// 文档信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentInfo>,
}

impl ApiResponseTrait for CreateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDocumentRequest {
    /// 文档token
    pub document_token: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 是否需要权限信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_permission: Option<bool>,
    /// 是否需要统计信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_statistics: Option<bool>,
}

impl GetDocumentRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.document_token.trim().is_empty() {
            return Err("文档token不能为空".to_string());
        }

        Ok(())
    }
}

/// 获取文档响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetDocumentResponse {
    /// 文档信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentInfo>,
}

impl ApiResponseTrait for GetDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档原始内容请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDocumentRawContentRequest {
    /// 文档token
    pub document_token: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetDocumentRawContentRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.document_token.trim().is_empty() {
            return Err("文档token不能为空".to_string());
        }

        Ok(())
    }
}

/// 获取文档原始内容响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetDocumentRawContentResponse {
    /// 文档原始内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
}

impl ApiResponseTrait for GetDocumentRawContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出文档块请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListDocumentBlocksRequest {
    /// 文档token
    pub document_token: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 块ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl ListDocumentBlocksRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.document_token.trim().is_empty() {
            return Err("文档token不能为空".to_string());
        }

        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 1000 {
                return Err("分页大小必须在1-1000之间".to_string());
            }
        }

        Ok(())
    }
}

/// 列出文档块响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListDocumentBlocksResponse {
    /// 文档信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentInfo>,
    /// 块列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BlockInfo>>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListDocumentBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建块子级请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBlockChildrenRequest {
    /// 文档token
    pub document_token: String,
    /// 块ID
    pub block_id: String,
    /// 子块列表
    pub children: Vec<serde_json::Value>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl CreateBlockChildrenRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.document_token.trim().is_empty() {
            return Err("文档token不能为空".to_string());
        }

        if self.block_id.trim().is_empty() {
            return Err("块ID不能为空".to_string());
        }

        if self.children.is_empty() {
            return Err("子块列表不能为空".to_string());
        }

        if self.children.len() > 100 {
            return Err("子块数量不能超过100个".to_string());
        }

        Ok(())
    }
}

/// 创建块子级响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateBlockChildrenResponse {
    /// 块ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for CreateBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 简化的群公告信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnnouncementInfo {
    /// 群ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// 群公告token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcement_token: Option<String>,
    /// 群公告标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 群公告内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// 获取群公告请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatAnnouncementRequest {
    /// 群ID
    pub chat_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl ChatAnnouncementRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.chat_id.trim().is_empty() {
            return Err("群ID不能为空".to_string());
        }

        Ok(())
    }
}

/// 获取群公告响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ChatAnnouncementResponse {
    /// 群公告信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcement: Option<AnnouncementInfo>,
}

impl ApiResponseTrait for ChatAnnouncementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// 为了简化，为其他所有需要的API类型创建基本的结构体定义

/// 创建块后代请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBlockDescendantRequest {
    /// 文档token
    pub document_token: String,
    /// 块ID
    pub block_id: String,
    /// 块列表
    pub blocks: Vec<serde_json::Value>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl CreateBlockDescendantRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.document_token.trim().is_empty() {
            return Err("文档token不能为空".to_string());
        }
        Ok(())
    }
}

/// 创建块后代响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateBlockDescendantResponse {
    /// 块ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for CreateBlockDescendantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新块请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateBlockRequest {
    /// 文档token
    pub document_token: String,
    /// 块ID
    pub block_id: String,
    /// 块类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<String>,
    /// 子块列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<serde_json::Value>>,
    /// 文本元素
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_elements: Option<Vec<serde_json::Value>>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl UpdateBlockRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.document_token.trim().is_empty() {
            return Err("文档token不能为空".to_string());
        }
        if self.block_id.trim().is_empty() {
            return Err("块ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 更新块响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateBlockResponse {
    /// 成功标志
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for UpdateBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取块请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetBlockRequest {
    /// 文档token
    pub document_token: String,
    /// 块ID
    pub block_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetBlockRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.document_token.trim().is_empty() {
            return Err("文档token不能为空".to_string());
        }
        if self.block_id.trim().is_empty() {
            return Err("块ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 获取块响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetBlockResponse {
    /// 块信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<BlockInfo>,
}

impl ApiResponseTrait for GetBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新块请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateBlocksRequest {
    /// 文档token
    pub document_token: String,
    /// 更新请求列表
    pub requests: Vec<serde_json::Value>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl BatchUpdateBlocksRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.document_token.trim().is_empty() {
            return Err("文档token不能为空".to_string());
        }
        if self.requests.is_empty() {
            return Err("更新请求列表不能为空".to_string());
        }
        Ok(())
    }
}

/// 批量更新块响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchUpdateBlocksResponse {
    /// 响应列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<serde_json::Value>>,
}

impl ApiResponseTrait for BatchUpdateBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取块子级请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetBlockChildrenRequest {
    /// 文档token
    pub document_token: String,
    /// 块ID
    pub block_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetBlockChildrenRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.document_token.trim().is_empty() {
            return Err("文档token不能为空".to_string());
        }
        if self.block_id.trim().is_empty() {
            return Err("块ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 获取块子级响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetBlockChildrenResponse {
    /// 文档信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentInfo>,
    /// 子块列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BlockInfo>>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for GetBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除块子级请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteBlockChildrenRequest {
    /// 文档token
    pub document_token: String,
    /// 父块ID
    pub block_id: String,
    /// 要删除的块ID列表
    pub block_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl DeleteBlockChildrenRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.document_token.trim().is_empty() {
            return Err("文档token不能为空".to_string());
        }
        if self.block_id.trim().is_empty() {
            return Err("块ID不能为空".to_string());
        }
        if self.block_ids.is_empty() {
            return Err("要删除的块ID列表不能为空".to_string());
        }
        Ok(())
    }
}

/// 删除块子级响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteBlockChildrenResponse {
    /// 已删除的块ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_block_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for DeleteBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 内容转换请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConvertContentRequest {
    /// 文档token
    pub document_token: String,
    /// 目标类型
    pub target_type: String,
    /// 块ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl ConvertContentRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.document_token.trim().is_empty() {
            return Err("文档token不能为空".to_string());
        }
        if self.target_type.trim().is_empty() {
            return Err("目标类型不能为空".to_string());
        }
        Ok(())
    }
}

/// 内容转换响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConvertContentResponse {
    /// 转换后的内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl ApiResponseTrait for ConvertContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出群公告块请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListChatAnnouncementBlocksRequest {
    /// 群ID
    pub chat_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl ListChatAnnouncementBlocksRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.chat_id.trim().is_empty() {
            return Err("群ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 列出群公告块响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListChatAnnouncementBlocksResponse {
    /// 群公告信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcement: Option<AnnouncementInfo>,
    /// 块列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BlockInfo>>,
}

impl ApiResponseTrait for ListChatAnnouncementBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建群公告块子级请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateChatAnnouncementBlockChildrenRequest {
    /// 群ID
    pub chat_id: String,
    /// 块ID
    pub block_id: String,
    /// 子块列表
    pub children: Vec<serde_json::Value>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl CreateChatAnnouncementBlockChildrenRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.chat_id.trim().is_empty() {
            return Err("群ID不能为空".to_string());
        }
        if self.block_id.trim().is_empty() {
            return Err("块ID不能为空".to_string());
        }
        if self.children.is_empty() {
            return Err("子块列表不能为空".to_string());
        }
        Ok(())
    }
}

/// 创建群公告块子级响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateChatAnnouncementBlockChildrenResponse {
    /// 块ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for CreateChatAnnouncementBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新群公告块请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateChatAnnouncementBlocksRequest {
    /// 群ID
    pub chat_id: String,
    /// 更新请求列表
    pub requests: Vec<serde_json::Value>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl BatchUpdateChatAnnouncementBlocksRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.chat_id.trim().is_empty() {
            return Err("群ID不能为空".to_string());
        }
        if self.requests.is_empty() {
            return Err("更新请求列表不能为空".to_string());
        }
        Ok(())
    }
}

/// 批量更新群公告块响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchUpdateChatAnnouncementBlocksResponse {
    /// 响应列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<serde_json::Value>>,
}

impl ApiResponseTrait for BatchUpdateChatAnnouncementBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群公告块请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetChatAnnouncementBlockRequest {
    /// 群ID
    pub chat_id: String,
    /// 块ID
    pub block_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetChatAnnouncementBlockRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.chat_id.trim().is_empty() {
            return Err("群ID不能为空".to_string());
        }
        if self.block_id.trim().is_empty() {
            return Err("块ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 获取群公告块响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetChatAnnouncementBlockResponse {
    /// 块信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<BlockInfo>,
}

impl ApiResponseTrait for GetChatAnnouncementBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群公告块子级请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetChatAnnouncementBlockChildrenRequest {
    /// 群ID
    pub chat_id: String,
    /// 块ID
    pub block_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetChatAnnouncementBlockChildrenRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.chat_id.trim().is_empty() {
            return Err("群ID不能为空".to_string());
        }
        if self.block_id.trim().is_empty() {
            return Err("块ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 获取群公告块子级响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetChatAnnouncementBlockChildrenResponse {
    /// 群公告信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcement: Option<AnnouncementInfo>,
    /// 子块列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BlockInfo>>,
}

impl ApiResponseTrait for GetChatAnnouncementBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除群公告块子级请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteChatAnnouncementBlockChildrenRequest {
    /// 群ID
    pub chat_id: String,
    /// 父块ID
    pub block_id: String,
    /// 要删除的块ID列表
    pub block_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl DeleteChatAnnouncementBlockChildrenRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.chat_id.trim().is_empty() {
            return Err("群ID不能为空".to_string());
        }
        if self.block_id.trim().is_empty() {
            return Err("块ID不能为空".to_string());
        }
        if self.block_ids.is_empty() {
            return Err("要删除的块ID列表不能为空".to_string());
        }
        Ok(())
    }
}

/// 删除群公告块子级响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteChatAnnouncementBlockChildrenResponse {
    /// 已删除的块ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_block_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for DeleteChatAnnouncementBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
