use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::AppService;

impl AppService {
    /// 更新多维表格元数据
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/update>
    pub async fn update(
        &self,
        request: UpdateAppRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateAppResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PUT;
        api_req.api_path = BITABLE_V1_APP_UPDATE.replace("{app_token}", &request.app_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = serde_json::to_vec(&UpdateAppRequestBody {
            name: request.name,
            is_advanced: request.is_advanced,
        })?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 更新多维表格元数据请求
#[derive(Debug, Default)]
pub struct UpdateAppRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 多维表格的名字
    name: Option<String>,
    /// 多维表格是否开启高级权限
    is_advanced: Option<bool>,
}

impl UpdateAppRequest {
    pub fn builder() -> UpdateAppRequestBuilder {
        UpdateAppRequestBuilder::default()
    }

    /// 创建更新多维表格元数据请求
    pub fn new(app_token: impl ToString) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
            name: None,
            is_advanced: None,
        }
    }
}

#[derive(Default)]
pub struct UpdateAppRequestBuilder {
    request: UpdateAppRequest,
}

impl UpdateAppRequestBuilder {
    /// 多维表格的 app_token
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 多维表格的名字
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = Some(name.to_string());
        self
    }

    /// 多维表格是否开启高级权限
    pub fn is_advanced(mut self, is_advanced: bool) -> Self {
        self.request.is_advanced = Some(is_advanced);
        self
    }

    pub fn build(self) -> UpdateAppRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    UpdateAppRequestBuilder,
    AppService,
    UpdateAppRequest,
    BaseResponse<UpdateAppResponse>,
    update
);

#[derive(Serialize)]
struct UpdateAppRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_advanced: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateAppResponse {
    /// 多维表格的 app 信息
    pub app: UpdateAppResponseData,
}

#[derive(Deserialize, Debug)]
pub struct UpdateAppResponseData {
    /// 多维表格的 app_token
    pub app_token: String,
    /// 多维表格的名字
    pub name: String,
    /// 多维表格的版本号
    pub revision: i32,
    /// 多维表格是否开启了高级权限
    pub is_advanced: bool,
}

impl ApiResponseTrait for UpdateAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_update_app_request() {
        let request = UpdateAppRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .name("新的表格名称")
            .is_advanced(true)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.name, Some("新的表格名称".to_string()));
        assert_eq!(request.is_advanced, Some(true));
    }

    #[test]
    fn test_update_app_request_new() {
        let request = UpdateAppRequest::new("bascnmBA*****yGehy8");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.name, None);
        assert_eq!(request.is_advanced, None);
    }

    #[test]
    fn test_update_app_request_body_serialization() {
        let body = UpdateAppRequestBody {
            name: Some("新的表格名称".to_string()),
            is_advanced: Some(true),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "新的表格名称",
            "is_advanced": true
        });

        assert_eq!(serialized, expected);
    }
}
