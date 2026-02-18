//! 部门相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 部门名称国际化配置（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentI18nName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 部门信息（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Department {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<DepartmentI18nName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_ids: Option<Vec<String>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 部门详情响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentResponse {
    pub department: Department,
}

impl ApiResponseTrait for DepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 部门列表响应 data（分页）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentListResponse {
    #[serde(default)]
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub items: Vec<Department>,
}

impl ApiResponseTrait for DepartmentListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取部门信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetDepartmentsResponse {
    #[serde(default)]
    pub items: Vec<Department>,
}

impl ApiResponseTrait for BatchGetDepartmentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(
        original: &T,
    ) {
        let json = serde_json::to_string(original).expect("序列化失败");
        let deserialized: T = serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(original, &deserialized, "roundtrip 后数据不一致");
    }

    #[test]
    fn test_department_i18n_name_serialization() {
        let i18n = DepartmentI18nName {
            zh_cn: Some("技术部".to_string()),
            en_us: Some("Tech Department".to_string()),
            ja_jp: None,
            extra: HashMap::new(),
        };
        test_roundtrip(&i18n);
    }

    #[test]
    fn test_department_serialization() {
        let dept = Department {
            name: Some("技术部".to_string()),
            i18n_name: None,
            parent_department_id: Some("parent123".to_string()),
            department_id: Some("dept123".to_string()),
            open_department_id: Some("opendept123".to_string()),
            leader_user_id: Some("leader123".to_string()),
            chat_id: Some("chat123".to_string()),
            order: Some("1".to_string()),
            unit_ids: Some(vec!["unit1".to_string(), "unit2".to_string()]),
            extra: HashMap::new(),
        };
        test_roundtrip(&dept);
    }

    #[test]
    fn test_department_response_serialization() {
        let response = DepartmentResponse {
            department: Department {
                name: Some("技术部".to_string()),
                i18n_name: None,
                parent_department_id: None,
                department_id: Some("dept123".to_string()),
                open_department_id: None,
                leader_user_id: None,
                chat_id: None,
                order: None,
                unit_ids: None,
                extra: HashMap::new(),
            },
        };
        test_roundtrip(&response);
    }

    #[test]
    fn test_department_list_response_serialization() {
        let response = DepartmentListResponse {
            has_more: true,
            page_token: Some("next_page".to_string()),
            items: vec![Department {
                name: Some("技术部".to_string()),
                i18n_name: None,
                parent_department_id: None,
                department_id: Some("dept123".to_string()),
                open_department_id: None,
                leader_user_id: None,
                chat_id: None,
                order: None,
                unit_ids: None,
                extra: HashMap::new(),
            }],
        };
        test_roundtrip(&response);
    }

    #[test]
    fn test_batch_get_departments_response_serialization() {
        let response = BatchGetDepartmentsResponse {
            items: vec![Department {
                name: Some("技术部".to_string()),
                i18n_name: None,
                parent_department_id: None,
                department_id: Some("dept123".to_string()),
                open_department_id: None,
                leader_user_id: None,
                chat_id: None,
                order: None,
                unit_ids: None,
                extra: HashMap::new(),
            }],
        };
        test_roundtrip(&response);
    }
}
