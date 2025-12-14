use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取文件版本列表请求
#[derive(Debug, Serialize, Default)]
pub struct ListFileVersionsRequest {
    /// 文件token
    pub file_token: String,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 获取文件版本列表响应
#[derive(Debug, Deserialize, Default)]
pub struct ListFileVersionsResponse {
    /// 版本列表
    pub versions: Vec<VersionInfo>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
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

/// 获取文件版本列表
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/versions
pub async fn list_file_versions(
    request: ListFileVersionsRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<ListFileVersionsResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/files/{}/versions",
        config.base_url, request.file_token
    );

    let mut query_params = vec![];

    if let Some(page_size) = request.page_size {
        query_params.push(("page_size".to_string(), page_size.to_string()));
    }

    if let Some(page_token) = &request.page_token {
        query_params.push(("page_token".to_string(), page_token.clone()));
    }

    let req = OpenLarkRequest {
        url,
        method: http::Method::GET,
        headers: vec![],
        query_params,
        body: None,
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_list_file_versions() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = ListFileVersionsRequest {
            file_token: "test_file_token".to_string(),
            page_size: Some(20),
            page_token: None,
        };

        let result = list_file_versions(request, &config, None).await;
        assert!(result.is_ok());
    }
}