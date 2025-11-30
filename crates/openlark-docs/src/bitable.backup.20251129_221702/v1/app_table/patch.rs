//! 更新数据表模块 (Patch 方式)
//!
//! 提供数据表的增量更新功能，使用 JSON Patch 格式进行部分字段更新。

use openlark_core::{
    api::ApiRequest,
    core::{BaseResponse, ResponseFormat, api::ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    service::bitable::v1::{TableData, TableField},
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 更新数据表请求 (Patch)
#[derive(Clone)]
pub struct PatchTableRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
    /// 表名
    pub name: Option<String>,
    /// 表字段
    pub fields: Option<Vec<TableField>>,
}

impl PatchTableRequest {
    /// 创建新的更新请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::PATCH, UPDATE_TABLE.to_string()),
            app_token: String::new(),
            table_id: String::new(),
            name: None,
            fields: None,
        }
    }

    /// 创建构建器
    pub fn builder() -> PatchTableRequestBuilder {
        PatchTableRequestBuilder::default()
    }
}

/// 更新数据表请求构建器
#[derive(Default)]
pub struct PatchTableRequestBuilder {
    request: PatchTableRequest,
}

impl PatchTableRequestBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    /// 设置表格 ID
    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request.table_id = table_id.into();
        self
    }

    /// 设置表名
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
        self
    }

    /// 设置字段
    pub fn fields(mut self, fields: Vec<TableField>) -> Self {
        self.request.fields = Some(fields);
        self
    }

    /// 构建请求
    pub fn build(self) -> PatchTableRequest {
        self.request
    }
}

/// 更新数据表请求体
#[derive(Serialize)]
struct PatchTableRequestBody {
    /// 表名
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 表字段
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<TableField>>,
}

/// 更新数据表响应
#[derive(Clone)]
pub struct PatchTableResponse {
    /// 更新的数据表信息
    pub table: PatchTableResponseData,
}

/// 更新数据表响应数据
#[derive(Clone)]
pub struct PatchTableResponseData {
    /// 数据表的 table_id
    pub table_id: String,
    /// 数据表的名字
    pub name: String,
    /// 数据表的版本号
    pub revision: i32,
    /// 数据表字段列表
    pub fields: Vec<TableField>,
    /// 数据表记录数量
    pub record_count: i32,
}

impl ApiResponseTrait for PatchTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_table_request_builder() {
        let fields = vec![
            TableField {
                field_name: "name".to_string(),
                field_type: "text".to_string(),
                ..Default::default()
            }
        ];

        let request = PatchTableRequest::builder()
            .app_token("bascnxxxxxxxxxxxxxxx")
            .table_id("tblxxxxxxxxxxxxxxx")
            .name("更新的表格")
            .fields(fields)
            .build();

        assert_eq!(request.app_token, "bascnxxxxxxxxxxxxxxx");
        assert_eq!(request.table_id, "tblxxxxxxxxxxxxxxx");
        assert_eq!(request.name, Some("更新的表格".to_string()));
        assert!(request.fields.is_some());
    }

    #[test]
    fn test_patch_table_request_body_serialization() {
        let fields = vec![
            TableField {
                field_name: "name".to_string(),
                field_type: "text".to_string(),
                ..Default::default()
            }
        ];

        let body = PatchTableRequestBody {
            name: Some("更新的表格".to_string()),
            fields: Some(fields),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = serde_json::json!({
            "name": "更新的表格",
            "fields": [
                {
                    "field_name": "name",
                    "field_type": "text"
                }
            ]
        });

        assert_eq!(serialized, expected);
    }
}