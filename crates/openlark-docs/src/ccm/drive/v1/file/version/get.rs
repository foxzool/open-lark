use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取文件版本信息请求
#[derive(Debug, Serialize, Default)]
pub struct GetFileVersionRequest {
    /// 文件token
    pub file_token: String,
    /// 版本ID
    pub version_id: String,
}

/// 获取文件版本信息响应
#[derive(Debug, Deserialize, Default)]
pub struct GetFileVersionResponse {
    /// 版本信息
    pub version: VersionInfo,
    /// 操作结果
    pub result: String,
}

/// 版本信息
#[derive(Debug, Deserialize, Default)]
pub struct VersionInfo {
    /// 版本ID
    pub version_id: String,
    /// 版本号
    pub version_number: i32,
    /// 版本名称
    pub name: String,
    /// 创建时间
    pub created_at: String,
    /// 创建者信息
    pub creator: CreatorInfo,
    /// 文件大小
    pub size: u64,
    /// 版本类型
    pub r#type: String,
    /// 是否为主要版本
    pub is_major: bool,
    /// 版本备注
    pub remarks: Option<String>,
    /// 修改历史
    pub changes: Option<Vec<ChangeInfo>>,
    /// 下载链接
    pub download_url: Option<String>,
    /// 预览链接
    pub preview_url: Option<String>,
}

/// 创建者信息
#[derive(Debug, Deserialize, Default)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 头像
    pub avatar: Option<AvatarInfo>,
}

/// 头像信息
#[derive(Debug, Deserialize, Default)]
pub struct AvatarInfo {
    /// 头像URL
    pub url: String,
    /// 头像大小
    pub size: Option<u32>,
}

/// 修改信息
#[derive(Debug, Deserialize, Default)]
pub struct ChangeInfo {
    /// 修改类型
    pub r#type: String,
    /// 修改描述
    pub description: String,
}

/// 获取文件版本信息
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/versions/:version_id
pub async fn get_file_version(
    request: GetFileVersionRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetFileVersionResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/files/{}/versions/{}",
        config.base_url, request.file_token, request.version_id
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
    async fn test_get_file_version() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetFileVersionRequest {
            file_token: "test_file_token".to_string(),
            version_id: "test_version_id".to_string(),
        };

        let result = get_file_version(request, &config, None).await;
        assert!(result.is_ok());
    }
}