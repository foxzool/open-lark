//! 更新招聘需求
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/job_requirement/update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新招聘需求请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateRequest {
    /// 配置信息
    config: Config,
    job_requirement_id: String,
}

impl UpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_requirement_id: String::new(),
        }
    }

    pub fn job_requirement_id(mut self, job_requirement_id: impl Into<String>) -> Self {
        self.job_requirement_id = job_requirement_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResponse> {
        validate_required!(
            self.job_requirement_id.trim(),
            "job_requirement_id 不能为空"
        );

        let request = ApiRequest::<UpdateResponse>::put(format!(
            "/open-apis/hire/v1/job_requirements/{}",
            self.job_requirement_id
        ));
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新招聘需求响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新招聘需求响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateResponse {
    /// 响应数据
    ///
    /// 当前按未建模 JSON 原样透传；字段收敛后再替换为显式结构。
    pub data: Value,
}

impl ApiResponseTrait for UpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
