//! 查询面试官信息列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/interviewer/list

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
    user_ids: Option<Vec<String>>,
    verify_status: Option<i32>,
    earliest_update_time: Option<String>,
    latest_update_time: Option<String>,
    user_id_type: Option<String>,
}

impl ListRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_ids: None,
            verify_status: None,
            earliest_update_time: None,
            latest_update_time: None,
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

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn verify_status(mut self, verify_status: i32) -> Self {
        self.verify_status = Some(verify_status);
        self
    }

    pub fn earliest_update_time(mut self, earliest_update_time: impl Into<String>) -> Self {
        self.earliest_update_time = Some(earliest_update_time.into());
        self
    }

    pub fn latest_update_time(mut self, latest_update_time: impl Into<String>) -> Self {
        self.latest_update_time = Some(latest_update_time.into());
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
            if !(1..=200).contains(&page_size) {
                return Err(error::validation_error(
                    "page_size",
                    "page_size 必须在 1-200 之间",
                ));
            }
        }

        if let Some(ref user_ids) = self.user_ids {
            if user_ids.len() > 50 {
                return Err(error::validation_error(
                    "user_ids",
                    "user_ids 不能超过 50 个",
                ));
            }
        }

        let mut request = ApiRequest::<ListResponse>::get("/open-apis/hire/v1/interviewers");
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(user_ids) = self.user_ids {
            request = request.query(
                "user_ids",
                serde_json::to_string(&user_ids).map_err(|e| {
                    error::validation_error("user_ids", format!("无法序列化数组查询参数: {}", e))
                })?,
            );
        }
        if let Some(verify_status) = self.verify_status {
            request = request.query("verify_status", verify_status.to_string());
        }
        if let Some(earliest_update_time) = self.earliest_update_time {
            request = request.query("earliest_update_time", earliest_update_time);
        }
        if let Some(latest_update_time) = self.latest_update_time {
            request = request.query("latest_update_time", latest_update_time);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error("查询面试官信息列表响应数据为空", "服务器没有返回有效的数据")
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
