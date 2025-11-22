//! Card API 数据模型
//!
//! 提供卡片实体管理相关的数据结构，支持卡片的创建、更新、配置等操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 创建卡片请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCardRequest {
    /// 卡片内容
    pub card_content: Value,
    /// 卡片类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// 模板ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 临时卡片标记（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp: Option<bool>,
    /// 临时卡片过期时间（可选，秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_expire_time: Option<i32>,
}

impl CreateCardRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        // 验证卡片内容不为空
        if self.card_content.is_null() {
            return Err("卡片内容不能为空".to_string());
        }

        // 验证卡片内容是否为有效对象
        if !self.card_content.is_object() {
            return Err("卡片内容必须是有效的JSON对象".to_string());
        }

        // 验证临时卡片过期时间
        if let Some(temp_expire_time) = self.temp_expire_time {
            if temp_expire_time <= 0 || temp_expire_time > 86400 {
                return Err("临时卡片过期时间必须在1-86400秒之间".to_string());
            }
        }

        Ok(())
    }
}

/// 创建卡片响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateCardResponse {
    /// 卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl ApiResponseTrait for CreateCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新卡片请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateCardRequest {
    /// 卡片ID
    pub card_id: String,
    /// 卡片内容
    pub card_content: Value,
    /// 卡片类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// 更新掩码（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mask: Option<Vec<String>>,
}

impl UpdateCardRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.card_id.trim().is_empty() {
            return Err("卡片ID不能为空".to_string());
        }

        // 验证卡片内容不为空
        if self.card_content.is_null() {
            return Err("卡片内容不能为空".to_string());
        }

        // 验证卡片内容是否为有效对象
        if !self.card_content.is_object() {
            return Err("卡片内容必须是有效的JSON对象".to_string());
        }

        Ok(())
    }
}

/// 更新卡片响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateCardResponse {
    /// 卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl ApiResponseTrait for UpdateCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新卡片请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateCardRequest {
    /// 卡片ID
    pub card_id: String,
    /// 批量更新操作
    pub operations: Vec<CardOperation>,
}

impl BatchUpdateCardRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.card_id.trim().is_empty() {
            return Err("卡片ID不能为空".to_string());
        }

        if self.operations.is_empty() {
            return Err("操作列表不能为空".to_string());
        }

        if self.operations.len() > 50 {
            return Err("操作列表不能超过50个".to_string());
        }

        for (index, operation) in self.operations.iter().enumerate() {
            if let Err(e) = operation.validate() {
                return Err(format!("操作{}验证失败: {}", index + 1, e));
            }
        }

        Ok(())
    }
}

/// 批量更新卡片响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchUpdateCardResponse {
    /// 卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl ApiResponseTrait for BatchUpdateCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新卡片设置请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateCardSettingsRequest {
    /// 卡片ID
    pub card_id: String,
    /// 卡片设置
    pub settings: CardSettings,
}

impl UpdateCardSettingsRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.card_id.trim().is_empty() {
            return Err("卡片ID不能为空".to_string());
        }

        self.settings.validate()?;
        Ok(())
    }
}

/// 更新卡片设置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateCardSettingsResponse {
    /// 卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl ApiResponseTrait for UpdateCardSettingsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 转换卡片ID请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConvertCardIdRequest {
    /// 源ID类型
    pub source_id_type: String,
    /// 目标ID类型
    pub target_id_type: String,
    /// 卡片ID列表
    pub card_ids: Vec<String>,
}

impl ConvertCardIdRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.source_id_type.trim().is_empty() {
            return Err("源ID类型不能为空".to_string());
        }

        if self.target_id_type.trim().is_empty() {
            return Err("目标ID类型不能为空".to_string());
        }

        if self.card_ids.is_empty() {
            return Err("卡片ID列表不能为空".to_string());
        }

        if self.card_ids.len() > 100 {
            return Err("卡片ID列表不能超过100个".to_string());
        }

        for (index, card_id) in self.card_ids.iter().enumerate() {
            if card_id.trim().is_empty() {
                return Err(format!("卡片ID{}不能为空", index + 1));
            }
        }

        Ok(())
    }
}

/// 转换卡片ID响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConvertCardIdResponse {
    /// 转换结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversions: Option<Vec<CardIdConversion>>,
}

impl ApiResponseTrait for ConvertCardIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 卡片操作
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardOperation {
    /// 操作类型
    pub operation_type: String,
    /// 组件ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    /// 组件数据（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<Value>,
    /// 卡片配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
}

impl CardOperation {
    /// 验证操作参数
    pub fn validate(&self) -> Result<(), String> {
        if self.operation_type.trim().is_empty() {
            return Err("操作类型不能为空".to_string());
        }

        // 验证操作类型是否支持
        let valid_operations = vec![
            "update_element",
            "delete_element",
            "add_element",
            "update_config",
        ];
        if !valid_operations.contains(&self.operation_type.as_str()) {
            return Err(format!("不支持的操作类型: {}", self.operation_type));
        }

        Ok(())
    }
}

/// 卡片设置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CardSettings {
    /// 是否启用响应
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_response: Option<bool>,
    /// 响应超时时间（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_timeout: Option<i32>,
    /// 自定义设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_settings: Option<Value>,
}

impl CardSettings {
    /// 验证设置参数
    pub fn validate(&self) -> Result<(), String> {
        if let Some(response_timeout) = self.response_timeout {
            if response_timeout <= 0 || response_timeout > 300 {
                return Err("响应超时时间必须在1-300秒之间".to_string());
            }
        }
        Ok(())
    }
}

/// 卡片ID转换结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CardIdConversion {
    /// 源卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_card_id: Option<String>,
    /// 目标卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_card_id: Option<String>,
    /// 转换状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// 卡片信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CardInfo {
    /// 卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 卡片类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<CreatorInfo>,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreatorInfo {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// 卡片类型常量
pub mod card_types {
    /// 交互卡片
    pub const INTERACTIVE: &str = "interactive";
    /// 全屏通知
    pub const FULL_SCREEN_NOTIFICATION: &str = "full_screen_notification";
    /// 暂态消息
    pub const TRANSIENT_MESSAGE: &str = "transient_message";
}

/// ID类型常量
pub mod id_types {
    /// 卡片ID
    pub const CARD_ID: &str = "card_id";
    /// 卡片模板ID
    pub const TEMPLATE_ID: &str = "template_id";
    /// 外部ID
    pub const EXTERNAL_ID: &str = "external_id";
}

/// 操作类型常量
pub mod operation_types {
    /// 更新组件
    pub const UPDATE_ELEMENT: &str = "update_element";
    /// 删除组件
    pub const DELETE_ELEMENT: &str = "delete_element";
    /// 添加组件
    pub const ADD_ELEMENT: &str = "add_element";
    /// 更新配置
    pub const UPDATE_CONFIG: &str = "update_config";
}
