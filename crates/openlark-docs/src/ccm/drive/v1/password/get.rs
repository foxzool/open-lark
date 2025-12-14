use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取密码保护信息请求
#[derive(Debug, Serialize, Default)]
pub struct GetPasswordRequest {
    /// 文件token
    pub file_token: String,
}

/// 获取密码保护信息响应
#[derive(Debug, Deserialize, Default)]
pub struct GetPasswordResponse {
    /// 密码保护信息
    pub password_protection: PasswordProtectionInfo,
    /// 操作结果
    pub result: String,
}

/// 密码保护信息
#[derive(Debug, Deserialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
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
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/password
pub async fn get_password(
    request: GetPasswordRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetPasswordResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/files/{}/password",
        config.base_url, request.file_token
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::GET,
        headers: vec![],
        query_params: vec![],
        body: None,
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_password() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetPasswordRequest {
            file_token: "test_file_token".to_string(),
        };

        let result = get_password(request, &config, None).await;
        assert!(result.is_ok());
    }
}