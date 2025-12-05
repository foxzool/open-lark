//! 旧版文档 v2 API 响应结构体
//!
//! 定义所有旧版文档API的响应数据结构。

use super::models::{DocumentContent, DocumentInfo, SheetMeta};
use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 创建旧版文档响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateDocV2Response {
    /// 文档信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentInfo>,
}

impl ApiResponseTrait for CreateDocV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档元信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetDocMetaV2Response {
    /// 文档信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentInfo>,
}

impl ApiResponseTrait for GetDocMetaV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档纯文本内容响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetDocRawContentV2Response {
    /// 文档内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<DocumentContent>,
}

impl ApiResponseTrait for GetDocRawContentV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档富文本内容响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetDocContentV2Response {
    /// 文档内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<DocumentContent>,
}

impl ApiResponseTrait for GetDocContentV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取电子表格元数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetDocSheetMetaV2Response {
    /// 电子表格元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_meta: Option<SheetMeta>,
}

impl ApiResponseTrait for GetDocSheetMetaV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 编辑旧版文档内容响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateDocBatchV2Response {
    /// 批量更新结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_update_result: Option<BatchUpdateResult>,
}

impl ApiResponseTrait for UpdateDocBatchV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateResult {
    /// 文档Token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_token: Option<String>,
    /// 更新的操作列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<BatchUpdateOperationResult>>,
    /// 新版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_revision: Option<i32>,
}

/// 操作结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateOperationResult {
    /// 操作索引
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 操作类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    /// 操作是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 错误信息（如果失败）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
