/// 删除密码保护
///
/// 删除文件的密码保护设置。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/password
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 删除密码保护请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePasswordRequest {
    /// 文件token
    pub file_token: String,
    /// 密码（验证用）
    pub password: Option<String>,
    /// 是否删除所有密码（包括部门密码）
    pub delete_all: Option<bool>,
}

impl DeletePasswordRequest {
    /// 创建删除密码保护请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            password: None,
            delete_all: None,
        }
    }

    /// 设置密码（验证用）
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// 设置是否删除所有密码（包括部门密码）
    pub fn delete_all(mut self, delete_all: bool) -> Self {
        self.delete_all = Some(delete_all);
        self
    }
}

/// 删除密码保护响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePasswordResponse {
    /// 是否成功
    pub success: bool,
    /// 操作结果
    pub result: String,
}

impl ApiResponseTrait for DeletePasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除密码保护
///
/// 删除文件的密码保护设置。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/password
pub async fn delete_password(
    request: DeletePasswordRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<DeletePasswordResponse>> {
    // 构建请求体
    let mut body = json!({});

    if let Some(password) = &request.password {
        body["password"] = json!(password);
    }
    if let Some(delete_all) = request.delete_all {
        body["deleteAll"] = json!(delete_all);
    }

    // 创建API请求
    let url = DriveApi::DeletePublicPassword(request.file_token.clone()).to_url();
    let mut api_request: ApiRequest<DeletePasswordResponse> =
        ApiRequest::delete(&url)
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_password_request_builder() {
        let request = DeletePasswordRequest::new("file_token")
            .password("test_password")
            .delete_all(false);

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.password, Some("test_password".to_string()));
        assert_eq!(request.delete_all, Some(false));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(DeletePasswordResponse::data_format(), ResponseFormat::Data);
    }
}