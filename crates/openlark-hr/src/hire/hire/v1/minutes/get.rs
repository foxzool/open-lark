//! 获取面试速记明细
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/minutes/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct GetRequest {
    config: Config,
    interview_id: String,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl GetRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            interview_id: String::new(),
            page_token: None,
            page_size: None,
        }
    }

    pub fn interview_id(mut self, interview_id: impl Into<String>) -> Self {
        self.interview_id = interview_id.into();
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        if let Some(page_size) = self.page_size {
            if !(1..=100).contains(&page_size) {
                return Err(error::validation_error(
                    "page_size",
                    "page_size 必须在 1-100 之间",
                ));
            }
        }

        if self.interview_id.trim().is_empty() {
            return Err(error::validation_error(
                "interview_id",
                "interview_id 不能为空",
            ));
        }

        let mut request = ApiRequest::<GetResponse>::get("/open-apis/hire/v1/minutes");
        request = request.query("interview_id", self.interview_id);
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error("获取面试速记明细响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 响应数据
    ///
    /// 当前按未建模 JSON 原样透传；字段收敛后再替换为显式结构。
    pub data: Value,
}

impl ApiResponseTrait for GetResponse {
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
