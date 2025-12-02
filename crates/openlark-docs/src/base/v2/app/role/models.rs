use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 角色创建请求结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoleRequest {
    /// 角色名称
    pub role_name: String,
    /// 表格角色配置列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_roles: Option<Vec<TableRole>>,
}

/// 角色更新请求结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRoleRequest {
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// 表格角色配置列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_roles: Option<Vec<TableRole>>,
}

/// 表格角色配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableRole {
    /// 表格ID
    pub table_id: String,
    /// 表格权限
    pub table_perm: i32,
    /// 记录规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_rule: Option<RecordRule>,
}

/// 记录访问规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordRule {
    /// 条件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

/// 条件定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Condition {
    /// 字段名称
    pub field_name: String,
    /// 操作符
    pub operator: String,
    /// 值列表
    pub value: Vec<serde_json::Value>,
}

/// 角色响应结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleResponse {
    /// 角色ID
    pub role_id: String,
    /// 角色名称
    pub role_name: String,
    /// 表格角色列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_roles: Option<Vec<TableRole>>,
}

/// 列出角色响应结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRolesResponse {
    /// 角色列表
    pub items: Vec<RoleItem>,
}

/// 角色项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleItem {
    /// 角色名称
    pub role_name: String,
    /// 表格角色列表
    pub table_roles: Vec<TableRoleInfo>,
}

/// 表格角色信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableRoleInfo {
    /// 表格权限
    pub table_perm: i32,
    /// 表格名称
    pub table_name: String,
    /// 表格ID
    pub table_id: String,
    /// 记录规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_rule: Option<RecordRule>,
}

/// 分页参数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaginationParams {
    /// 页面大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl CreateRoleRequest {
    /// 验证创建角色请求
    pub fn validate(&self) -> Result<(), String> {
        if self.role_name.trim().is_empty() {
            return Err("角色名称不能为空".to_string());
        }

        if self.role_name.len() > 100 {
            return Err("角色名称长度不能超过100个字符".to_string());
        }

        if let Some(table_roles) = &self.table_roles {
            if table_roles.len() > 100 {
                return Err("表格角色数量不能超过100个".to_string());
            }

            for table_role in table_roles {
                if let Err(e) = table_role.validate() {
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}

impl UpdateRoleRequest {
    /// 验证更新角色请求
    pub fn validate(&self) -> Result<(), String> {
        if let Some(role_name) = &self.role_name {
            if role_name.trim().is_empty() {
                return Err("角色名称不能为空".to_string());
            }

            if role_name.len() > 100 {
                return Err("角色名称长度不能超过100个字符".to_string());
            }
        }

        if let Some(table_roles) = &self.table_roles {
            if table_roles.len() > 100 {
                return Err("表格角色数量不能超过100个".to_string());
            }

            for table_role in table_roles {
                if let Err(e) = table_role.validate() {
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}

impl TableRole {
    /// 验证表格角色配置
    pub fn validate(&self) -> Result<(), String> {
        if self.table_id.trim().is_empty() {
            return Err("表格ID不能为空".to_string());
        }

        if self.table_perm < 0 || self.table_perm > 7 {
            return Err("表格权限值必须在0-7之间".to_string());
        }

        if let Some(rec_rule) = &self.rec_rule {
            if let Err(e) = rec_rule.validate() {
                return Err(e);
            }
        }

        Ok(())
    }
}

impl RecordRule {
    /// 验证记录规则
    pub fn validate(&self) -> Result<(), String> {
        if let Some(conditions) = &self.conditions {
            if conditions.len() > 50 {
                return Err("条件数量不能超过50个".to_string());
            }

            for condition in conditions {
                if let Err(e) = condition.validate() {
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}

impl Condition {
    /// 验证条件
    pub fn validate(&self) -> Result<(), String> {
        if self.field_name.trim().is_empty() {
            return Err("字段名称不能为空".to_string());
        }

        if self.operator.trim().is_empty() {
            return Err("操作符不能为空".to_string());
        }

        if self.value.is_empty() {
            return Err("值列表不能为空".to_string());
        }

        if self.value.len() > 50 {
            return Err("值列表长度不能超过50个".to_string());
        }

        Ok(())
    }
}

impl PaginationParams {
    /// 验证分页参数
    pub fn validate(&self) -> Result<(), String> {
        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 1000 {
                return Err("页面大小必须在1-1000之间".to_string());
            }
        }

        if let Some(page_token) = &self.page_token {
            if page_token.trim().is_empty() {
                return Err("页面标记不能为空".to_string());
            }
        }

        Ok(())
    }
}