//! 表单字段相关模型

use serde::{Deserialize, Serialize};

/// 更新表单字段问题请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchFormFieldRequest {
    /// 问题标题
    pub title: Option<String>,
    /// 问题描述
    pub description: Option<String>,
    /// 是否必填
    pub required: Option<bool>,
    /// 是否可见
    pub visible: Option<bool>,
    /// 问题设置
    pub setting: Option<serde_json::Value>,
}

impl PatchFormFieldRequest {
    /// 创建空的更新请求
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            required: None,
            visible: None,
            setting: None,
        }
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        // 至少要有一个字段需要更新
        if self.title.is_none()
            && self.description.is_none()
            && self.required.is_none()
            && self.visible.is_none()
            && self.setting.is_none()
        {
            return Err("至少需要提供一个要更新的字段".to_string());
        }

        // 如果提供了标题，不能为空
        if let Some(ref title) = self.title {
            if title.trim().is_empty() {
                return Err("问题标题不能为空".to_string());
            }
        }

        Ok(())
    }
}

impl Default for PatchFormFieldRequest {
    fn default() -> Self {
        Self::new()
    }
}
