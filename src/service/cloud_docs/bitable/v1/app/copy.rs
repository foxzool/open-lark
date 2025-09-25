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
    /// 复制多维表格
    pub async fn copy(
        &self,
        request: CopyAppRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CopyAppResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = BITABLE_V1_APP_COPY.replace("{app_token}", &request.app_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = serde_json::to_vec(&CopyAppRequestBody {
            name: request.name,
            folder_token: request.folder_token,
            time_zone: request.time_zone,
        })?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 复制多维表格请求
#[derive(Debug, Default)]
pub struct CopyAppRequest {
    api_request: ApiRequest,
    /// 待复制的多维表格的 app_token
    app_token: String,
    /// 复制的多维表格 App 名字
    name: Option<String>,
    /// 复制的多维表格所在文件夹的 token
    folder_token: Option<String>,
    /// 时区
    time_zone: Option<String>,
}

impl CopyAppRequest {
    pub fn builder() -> CopyAppRequestBuilder {
        CopyAppRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CopyAppRequestBuilder {
    request: CopyAppRequest,
}

impl CopyAppRequestBuilder {
    /// 待复制的多维表格的 app_token
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 复制的多维表格 App 名字
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = Some(name.to_string());
        self
    }

    /// 复制的多维表格所在文件夹的 token
    pub fn folder_token(mut self, folder_token: impl ToString) -> Self {
        self.request.folder_token = Some(folder_token.to_string());
        self
    }

    /// 时区
    pub fn time_zone(mut self, time_zone: impl ToString) -> Self {
        self.request.time_zone = Some(time_zone.to_string());
        self
    }

    pub fn build(self) -> CopyAppRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    CopyAppRequestBuilder,
    AppService,
    CopyAppRequest,
    BaseResponse<CopyAppResponse>,
    copy
);

#[derive(Serialize)]
struct CopyAppRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CopyAppResponse {
    /// 复制的多维表格的 app 信息
    pub app: CopyAppResponseData,
}

#[derive(Deserialize, Debug)]
pub struct CopyAppResponseData {
    /// 多维表格的 app_token
    pub app_token: String,
    /// 多维表格的名字
    pub name: String,
    /// 多维表格的版本号
    pub revision: i32,
    /// 多维表格的链接
    pub url: String,
}

impl ApiResponseTrait for CopyAppResponse {
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
    fn test_copy_app_request() {
        let request = CopyAppRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .name("复制的多维表格")
            .folder_token("fldcnmBA*****yGehy8")
            .time_zone("Asia/Shanghai")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.name, Some("复制的多维表格".to_string()));
        assert_eq!(
            request.folder_token,
            Some("fldcnmBA*****yGehy8".to_string())
        );
        assert_eq!(request.time_zone, Some("Asia/Shanghai".to_string()));
    }

    #[test]
    fn test_copy_app_request_body_serialization() {
        let body = CopyAppRequestBody {
            name: Some("复制的多维表格".to_string()),
            folder_token: Some("fldcnmBA*****yGehy8".to_string()),
            time_zone: Some("Asia/Shanghai".to_string()),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "复制的多维表格",
            "folder_token": "fldcnmBA*****yGehy8",
            "time_zone": "Asia/Shanghai"
        });

        assert_eq!(serialized, expected);
    }
}
