/// 获取密码保护信息
///
/// 获取文件的密码保护设置信息。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/password
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取密码保护信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPasswordRequest {
    /// 文件token
    pub file_token: String,
}

impl GetPasswordRequest {
    pub fn new(file_token: impl Into<String>) -> Self {
        Self { file_token: file_token.into() }
    }
}

/// 获取密码保护信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPasswordResponse {
    /// 密码保护信息
    pub data: Option<PasswordProtectionInfo>,
}

impl ApiResponseTrait for GetPasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 密码保护信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordProtectionInfo {
    /// 文件token
    pub file_token: String,
    /// 是否有密码保护
    pub has_password: bool,
    /// 密码提示
    pub password_hint: Option<String>,
    /// 部门密码列表
    pub department_passwords: Option<Vec<DepartmentPasswordInfo>>,
    /// 创建时间
    pub created_at: String,
}

/// 部门密码信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentPasswordInfo {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub department_name: String,
    /// 密码提示
    pub password_hint: Option<String>,
    /// 创建时间
    pub created_at: String,
}

/// 获取密码保护信息
///
/// 获取文件的密码保护设置信息。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/password
pub async fn get_password(
    request: GetPasswordRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetPasswordResponse>> {
    // 创建API请求
    let url = DriveApi::GetPublicPassword(request.file_token.clone()).to_url();
    let mut api_request: ApiRequest<GetPasswordResponse> =
        ApiRequest::get(&url);

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
    fn test_get_password_request_builder() {
        let request = GetPasswordRequest::new("file_token");
        assert_eq!(request.file_token, "file_token");
    }

    #[test]
    fn test_password_protection_info_structure() {
        let dept_passwords = vec![
            DepartmentPasswordInfo {
                department_id: "dept_123".to_string(),
                department_name: "技术部".to_string(),
                password_hint: Some("部门密码提示".to_string()),
                created_at: "2023-01-01T00:00:00Z".to_string(),
            }
        ];

        let password_info = PasswordProtectionInfo {
            file_token: "file_456".to_string(),
            has_password: true,
            password_hint: Some("文件密码提示".to_string()),
            department_passwords: Some(dept_passwords),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(password_info.file_token, "file_456");
        assert_eq!(password_info.has_password, true);
        assert_eq!(password_info.password_hint, Some("文件密码提示".to_string()));
        assert_eq!(password_info.department_passwords.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetPasswordResponse::data_format(), ResponseFormat::Data);
    }
}