/// 更新密码保护
///
/// 更新文件的密码保护设置，包括个人密码和部门密码。
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

/// 更新密码保护请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasswordRequest {
    /// 文件token
    pub file_token: String,
    /// 旧密码（验证用）
    pub old_password: Option<String>,
    /// 新密码
    pub new_password: String,
    /// 新密码提示
    pub new_password_hint: Option<String>,
    /// 针对部门的密码
    pub department_passwords: Option<Vec<DepartmentPassword>>,
}

impl UpdatePasswordRequest {
    /// 创建更新密码保护请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    /// * `new_password` - 新密码
    pub fn new(
        file_token: impl Into<String>,
        new_password: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            old_password: None,
            new_password: new_password.into(),
            new_password_hint: None,
            department_passwords: None,
        }
    }

    /// 设置旧密码
    pub fn old_password(mut self, old_password: impl Into<String>) -> Self {
        self.old_password = Some(old_password.into());
        self
    }

    /// 设置新密码提示
    pub fn new_password_hint(mut self, hint: impl Into<String>) -> Self {
        self.new_password_hint = Some(hint.into());
        self
    }

    /// 设置部门密码
    pub fn department_passwords(mut self, passwords: Vec<DepartmentPassword>) -> Self {
        self.department_passwords = Some(passwords);
        self
    }
}

/// 部门密码设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentPassword {
    /// 部门ID
    pub department_id: String,
    /// 部门密码
    pub password: String,
    /// 密码提示
    pub password_hint: Option<String>,
}

impl DepartmentPassword {
    /// 创建部门密码设置
    ///
    /// # 参数
    /// * `department_id` - 部门ID
    /// * `password` - 部门密码
    pub fn new(
        department_id: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            department_id: department_id.into(),
            password: password.into(),
            password_hint: None,
        }
    }

    /// 设置密码提示
    pub fn password_hint(mut self, hint: impl Into<String>) -> Self {
        self.password_hint = Some(hint.into());
        self
    }
}

/// 更新密码保护响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasswordResponse {
    /// 密码保护信息
    pub data: Option<PasswordProtectionInfo>,
}

impl ApiResponseTrait for UpdatePasswordResponse {
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

/// 更新密码保护
///
/// 更新文件的密码保护设置，包括个人密码和部门密码。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/password
pub async fn update_password(
    request: UpdatePasswordRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<UpdatePasswordResponse>> {
    // 构建请求体
    let mut body = json!({
        "newPassword": request.new_password
    });

    if let Some(old_password) = &request.old_password {
        body["oldPassword"] = json!(old_password);
    }
    if let Some(new_password_hint) = &request.new_password_hint {
        body["newPasswordHint"] = json!(new_password_hint);
    }
    if let Some(department_passwords) = &request.department_passwords {
        body["departmentPasswords"] = json!(department_passwords);
    }

    // 创建API请求
    let url = DriveApi::UpdatePublicPassword(request.file_token.clone()).to_url();
    let mut api_request: ApiRequest<UpdatePasswordResponse> =
        ApiRequest::put(&url)
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
    fn test_update_password_request_builder() {
        let dept_password = DepartmentPassword::new("dept_001", "dept_pass")
            .password_hint("部门密码提示");

        let request = UpdatePasswordRequest::new("file_token", "new_password")
            .old_password("old_password")
            .new_password_hint("新的密码提示")
            .department_passwords(vec![dept_password]);

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.new_password, "new_password");
        assert_eq!(request.old_password, Some("old_password".to_string()));
    }

    #[test]
    fn test_department_password_builder() {
        let dept_password = DepartmentPassword::new("dept_001", "password123")
            .password_hint("部门密码提示");

        assert_eq!(dept_password.department_id, "dept_001");
        assert_eq!(dept_password.password, "password123");
        assert_eq!(dept_password.password_hint, Some("部门密码提示".to_string()));
    }

    #[test]
    fn test_password_protection_info_structure() {
        let dept_passwords = vec![
            DepartmentPasswordInfo {
                department_id: "dept_123".to_string(),
                department_name: "技术部".to_string(),
                password_hint: Some("部门密码".to_string()),
                created_at: "2023-01-01T00:00:00Z".to_string(),
            }
        ];

        let protection_info = PasswordProtectionInfo {
            file_token: "file_456".to_string(),
            has_password: true,
            password_hint: Some("密码提示".to_string()),
            department_passwords: Some(dept_passwords),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(protection_info.file_token, "file_456");
        assert_eq!(protection_info.has_password, true);
        assert_eq!(protection_info.department_passwords.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UpdatePasswordResponse::data_format(), ResponseFormat::Data);
    }
}