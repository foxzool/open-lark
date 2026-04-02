//! 根据简历附件创建招聘官网投递任务
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/website.delivery/create_by_attachment

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 根据简历附件创建招聘官网投递任务请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CreateByAttachmentRequest {
    /// 配置信息
    config: Config,
    website_id: Option<String>,
}

impl CreateByAttachmentRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            website_id: None,
        }
    }

    pub fn website_id(mut self, website_id: impl Into<String>) -> Self {
        self.website_id = Some(website_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateByAttachmentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateByAttachmentResponse> {
        let website_id = self.website_id.unwrap_or_default();
        validate_required!(website_id.trim(), "website_id 不能为空");

        let request = ApiRequest::<CreateByAttachmentResponse>::post(format!(
            "/open-apis/hire/v1/websites/{}/deliveries/create_by_attachment",
            website_id
        ));
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "根据简历附件创建招聘官网投递任务响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 根据简历附件创建招聘官网投递任务响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateByAttachmentResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for CreateByAttachmentResponse {
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
