//! 获取人才标签信息列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/talent_tag/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::I18nText;

#[derive(Debug, Clone)]
pub struct ListRequest {
    config: Config,
    keyword: Option<String>,
    id_list: Option<Vec<String>>,
    tag_type: Option<i32>,
    include_inactive: Option<bool>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            keyword: None,
            id_list: None,
            tag_type: None,
            include_inactive: None,
            page_size: None,
            page_token: None,
        }
    }

    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keyword = Some(keyword.into());
        self
    }

    pub fn id_list(mut self, id_list: Vec<String>) -> Self {
        self.id_list = Some(id_list);
        self
    }

    pub fn tag_type(mut self, tag_type: i32) -> Self {
        self.tag_type = Some(tag_type);
        self
    }

    pub fn include_inactive(mut self, include_inactive: bool) -> Self {
        self.include_inactive = Some(include_inactive);
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        if let Some(page_size) = self.page_size {
            if !(1..=100).contains(&page_size) {
                return Err(error::validation_error(
                    "page_size",
                    "page_size 必须在 1-100 之间",
                ));
            }
        }

        let mut request = ApiRequest::<ListResponse>::get("/open-apis/hire/v1/talent_tags");
        if let Some(keyword) = self.keyword {
            request = request.query("keyword", keyword);
        }
        if let Some(id_list) = self.id_list {
            request = request.query(
                "id_list",
                serde_json::to_string(&id_list).map_err(|e| {
                    error::validation_error("id_list", format!("无法序列化数组查询参数: {}", e))
                })?,
            );
        }
        if let Some(tag_type) = self.tag_type {
            request = request.query("type", tag_type.to_string());
        }
        if let Some(include_inactive) = self.include_inactive {
            request = request.query("include_inactive", include_inactive.to_string());
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error(
                "获取人才标签信息列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentTagItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub tag_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    pub items: Vec<TalentTagItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #[test]
    fn test_serialization_roundtrip() {
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
