use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 更新云文档权限设置
///
/// 更新文件或文件夹的公开权限设置
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-public/patch
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 更新公开权限设置请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePublicPermissionRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub token: String,
    /// 权限设置
    pub setting: PermissionSetting,
}

/// 权限设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSetting {
    /// 是否公开
    pub public: bool,
    /// 权限类型
    pub r#type: String,
}

impl UpdatePublicPermissionRequest {
    /// 创建更新公开权限设置请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `setting` - 权限设置
    pub fn new(config: Config, token: impl Into<String>, setting: PermissionSetting) -> Self {
        Self {
            config,
            token: token.into(),
            setting,
        }
    }

    pub async fn execute(self) -> SDKResult<Response<UpdatePublicPermissionResponse>> {
        let api_endpoint = DriveApi::UpdatePublicPermission(self.token.clone());

        let api_request = ApiRequest::<UpdatePublicPermissionResponse>::patch(&api_endpoint.to_url())
            .body(serde_json::json!({
                "setting": self.setting
            }));

        Transport::request(api_request, &self.config, None).await
    }
}

/// 更新权限设置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePublicPermissionResponse {
    /// 更新后的权限设置
    pub data: Option<PermissionSetting>,
}

impl ApiResponseTrait for UpdatePublicPermissionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_public_permission_request_builder() {
        let config = Config::default();
        let setting = PermissionSetting {
            public: true,
            r#type: "anyone_with_link".to_string(),
        };

        let request = UpdatePublicPermissionRequest::new(config, "file_token", setting);

        assert_eq!(request.token, "file_token");
        assert!(request.setting.public);
        assert_eq!(request.setting.r#type, "anyone_with_link");
    }

    #[test]
    fn test_permission_setting_structure() {
        let setting = PermissionSetting {
            public: true,
            r#type: "anyone_with_link".to_string(),
            share_url: None,
            password_protected: false,
            password_info: None,
        };

        assert!(setting.public);
        assert_eq!(setting.r#type, "anyone_with_link");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            UpdatePublicPermissionResponse::data_format(),
            ResponseFormat::Data
        );
    }
}