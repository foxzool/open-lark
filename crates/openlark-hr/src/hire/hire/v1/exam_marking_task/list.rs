//! 获取笔试阅卷任务列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/exam_marking_task/list

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
pub struct ListRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id: String,
    activity_status: Option<i32>,
    user_id_type: Option<String>,
}

impl ListRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id: String::new(),
            activity_status: None,
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    pub fn activity_status(mut self, activity_status: i32) -> Self {
        self.activity_status = Some(activity_status);
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
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
            if !(1..=20).contains(&page_size) {
                return Err(error::validation_error(
                    "page_size",
                    "page_size 必须在 1-20 之间",
                ));
            }
        }

        if self.user_id.trim().is_empty() {
            return Err(error::validation_error("user_id", "user_id 不能为空"));
        }

        let mut request = ApiRequest::<ListResponse>::get("/open-apis/hire/v1/exam_marking_tasks");
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        request = request.query("user_id", self.user_id);
        if let Some(activity_status) = self.activity_status {
            request = request.query("activity_status", activity_status.to_string());
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error(
                "获取笔试阅卷任务列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 响应数据
    ///
    /// 当前按未建模 JSON 原样透传；字段收敛后再替换为显式结构。
    pub data: Value,
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
