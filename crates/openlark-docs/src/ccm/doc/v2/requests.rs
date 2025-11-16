//! 旧版文档 v2 API 请求结构体
//!
//! 定义所有旧版文档API的请求参数结构。

use super::models::{BatchUpdateOperation, BatchUpdateOperationType, DocType};
use serde::{Deserialize, Serialize};

/// 创建旧版文档请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateDocV2Request {
    /// 文档标题
    pub title: String,
    /// 文档类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<DocType>,
    /// 文件夹Token（可选，指定创建位置）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 模板ID（可选，使用模板创建）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

impl CreateDocV2Request {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("文档标题不能为空".to_string());
        }
        if self.title.len() > 255 {
            return Err("文档标题长度不能超过255个字符".to_string());
        }
        Ok(())
    }
}

/// 获取旧版文档元信息请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDocMetaV2Request {
    /// 文档Token
    pub doc_token: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetDocMetaV2Request {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.doc_token.trim().is_empty() {
            return Err("文档Token不能为空".to_string());
        }
        Ok(())
    }
}

/// 获取旧版文档纯文本内容请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDocRawContentV2Request {
    /// 文档Token
    pub doc_token: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetDocRawContentV2Request {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.doc_token.trim().is_empty() {
            return Err("文档Token不能为空".to_string());
        }
        Ok(())
    }
}

/// 获取旧版文档富文本内容请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDocContentV2Request {
    /// 文档Token
    pub doc_token: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetDocContentV2Request {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.doc_token.trim().is_empty() {
            return Err("文档Token不能为空".to_string());
        }
        Ok(())
    }
}

/// 获取电子表格元数据请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDocSheetMetaV2Request {
    /// 文档Token
    pub doc_token: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetDocSheetMetaV2Request {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.doc_token.trim().is_empty() {
            return Err("文档Token不能为空".to_string());
        }
        Ok(())
    }
}

/// 编辑旧版文档内容请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateDocBatchV2Request {
    /// 文档Token
    pub doc_token: String,
    /// 批量更新操作列表
    pub operations: Vec<BatchUpdateOperation>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl UpdateDocBatchV2Request {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.doc_token.trim().is_empty() {
            return Err("文档Token不能为空".to_string());
        }
        if self.operations.is_empty() {
            return Err("操作列表不能为空".to_string());
        }
        if self.operations.len() > 100 {
            return Err("操作数量不能超过100个".to_string());
        }

        for (index, operation) in self.operations.iter().enumerate() {
            if let Some(op_type) = &operation.operation_type {
                match op_type {
                    BatchUpdateOperationType::Insert | BatchUpdateOperationType::Replace => {
                        if operation
                            .content
                            .as_ref()
                            .map_or(true, |c| c.trim().is_empty())
                        {
                            return Err(format!(
                                "第{}个操作: 插入或替换操作不能有空的content",
                                index + 1
                            ));
                        }
                    }
                    BatchUpdateOperationType::UpdateTitle => {
                        if operation
                            .new_title
                            .as_ref()
                            .map_or(true, |t| t.trim().is_empty())
                        {
                            return Err(format!(
                                "第{}个操作: 更新标题操作不能有空的new_title",
                                index + 1
                            ));
                        }
                    }
                    BatchUpdateOperationType::Delete => {
                        // 删除操作需要有效的起始和结束位置
                        if operation.start_index.is_none() || operation.end_index.is_none() {
                            return Err(format!(
                                "第{}个操作: 删除操作必须指定start_index和end_index",
                                index + 1
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
