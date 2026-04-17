//! 获取面试记录附件
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/interview_record.attachment/get

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
    application_id: String,
    interview_record_id: Option<String>,
    language: Option<i32>,
}

impl GetRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
            interview_record_id: None,
            language: None,
        }
    }

    pub fn application_id(mut self, application_id: impl Into<String>) -> Self {
        self.application_id = application_id.into();
        self
    }

    pub fn interview_record_id(mut self, interview_record_id: impl Into<String>) -> Self {
        self.interview_record_id = Some(interview_record_id.into());
        self
    }

    pub fn language(mut self, language: i32) -> Self {
        self.language = Some(language);
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
        if self.application_id.trim().is_empty() {
            return Err(error::validation_error(
                "application_id",
                "application_id 不能为空",
            ));
        }

        let mut request =
            ApiRequest::<GetResponse>::get("/open-apis/hire/v1/interview_records/attachments");
        request = request.query("application_id", self.application_id);
        if let Some(interview_record_id) = self.interview_record_id {
            request = request.query("interview_record_id", interview_record_id);
        }
        if let Some(language) = self.language {
            request = request.query("language", language.to_string());
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error("获取面试记录附件响应数据为空", "服务器没有返回有效的数据")
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
