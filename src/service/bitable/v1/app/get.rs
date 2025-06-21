use reqwest::Method;
use serde::Deserialize;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use super::AppService;

impl AppService {
    /// 获取多维表格元数据
    pub async fn get(
        &self,
        request: GetAppRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetAppResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = format!("/open-apis/bitable/v1/apps/{}", request.app_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 获取多维表格元数据请求
#[derive(Debug, Default)]
pub struct GetAppRequest {
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    app_token: String,
}

impl GetAppRequest {
    pub fn builder() -> GetAppRequestBuilder {
        GetAppRequestBuilder::default()
    }

    /// 创建获取多维表格元数据请求
    pub fn new(app_token: impl ToString) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
        }
    }
}

#[derive(Default)]
pub struct GetAppRequestBuilder {
    request: GetAppRequest,
}

impl GetAppRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    pub fn build(self) -> GetAppRequest {
        self.request
    }
}

#[derive(Deserialize, Debug)]
pub struct GetAppResponse {
    /// 多维表格的 app 信息
    pub app: GetAppResponseData,
}

#[derive(Deserialize, Debug)]
pub struct GetAppResponseData {
    /// 多维表格的 app_token
    pub app_token: String,
    /// 多维表格的名字
    pub name: String,
    /// 多维表格的版本号（对多维表格进行修改时更新，如新增、删除数据表，修改数据表名等，初始为1，
    /// 每次更新+1）
    pub revision: i32,
    /// 多维表格是否开启了高级权限。取值包括：
    ///
    /// - true：表示开启了高级权限
    /// - false：表示关闭了高级权限
    pub is_advanced: bool,
    /// 文档时区
    pub time_zone: String,
}

impl ApiResponseTrait for GetAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_app_request() {
        let request = GetAppRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
    }

    #[test]
    fn test_get_app_request_new() {
        let request = GetAppRequest::new("bascnmBA*****yGehy8");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
    }
}