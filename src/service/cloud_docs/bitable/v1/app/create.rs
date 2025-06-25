use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::AppService;

impl AppService {
    /// 创建多维表格
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/create>
    pub async fn create(
        &self,
        request: CreateAppRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateAppResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/bitable/v1/apps".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = serde_json::to_vec(&CreateAppRequestBody {
            name: request.name,
            folder_token: request.folder_token,
            time_zone: request.time_zone,
        })?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 创建多维表格请求
#[derive(Debug, Default)]
pub struct CreateAppRequest {
    api_request: ApiRequest,
    /// 多维表格 App 名字
    name: String,
    /// 多维表格所在文件夹的 token，若不传则默认添加到用户云空间的根目录下
    folder_token: Option<String>,
    /// 时区
    time_zone: Option<String>,
}

impl CreateAppRequest {
    pub fn builder() -> CreateAppRequestBuilder {
        CreateAppRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateAppRequestBuilder {
    request: CreateAppRequest,
}

impl CreateAppRequestBuilder {
    /// 多维表格 App 名字
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = name.to_string();
        self
    }

    /// 多维表格所在文件夹的 token
    pub fn folder_token(mut self, folder_token: impl ToString) -> Self {
        self.request.folder_token = Some(folder_token.to_string());
        self
    }

    /// 时区
    pub fn time_zone(mut self, time_zone: impl ToString) -> Self {
        self.request.time_zone = Some(time_zone.to_string());
        self
    }

    pub fn build(self) -> CreateAppRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    CreateAppRequestBuilder,
    AppService,
    CreateAppRequest,
    BaseResponse<CreateAppResponse>,
    create
);

#[derive(Serialize)]
struct CreateAppRequestBody {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateAppResponse {
    /// 多维表格的 app 信息
    pub app: CreateAppResponseData,
}

#[derive(Deserialize, Debug)]
pub struct CreateAppResponseData {
    /// 多维表格的 app_token
    pub app_token: String,
    /// 多维表格的名字
    pub name: String,
    /// 多维表格的版本号
    pub revision: i32,
    /// 多维表格的链接
    pub url: String,
}

impl ApiResponseTrait for CreateAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_app_request() {
        let request = CreateAppRequest::builder()
            .name("测试多维表格")
            .folder_token("fldcnmBA*****yGehy8")
            .time_zone("Asia/Shanghai")
            .build();

        assert_eq!(request.name, "测试多维表格");
        assert_eq!(
            request.folder_token,
            Some("fldcnmBA*****yGehy8".to_string())
        );
        assert_eq!(request.time_zone, Some("Asia/Shanghai".to_string()));
    }

    #[test]
    fn test_create_app_request_body_serialization() {
        let body = CreateAppRequestBody {
            name: "测试多维表格".to_string(),
            folder_token: Some("fldcnmBA*****yGehy8".to_string()),
            time_zone: Some("Asia/Shanghai".to_string()),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "测试多维表格",
            "folder_token": "fldcnmBA*****yGehy8",
            "time_zone": "Asia/Shanghai"
        });

        assert_eq!(serialized, expected);
    }
}
