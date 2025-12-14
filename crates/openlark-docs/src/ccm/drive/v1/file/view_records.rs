use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取文件查看记录请求
#[derive(Debug, Serialize, Default)]
pub struct GetFileViewRecordsRequest {
    /// 文件token
    pub file_token: String,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 查看类型
    pub view_type: Option<String>,
}

/// 获取文件查看记录响应
#[derive(Debug, Deserialize, Default)]
pub struct GetFileViewRecordsResponse {
    /// 查看记录列表
    pub view_records: Vec<ViewRecord>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 操作结果
    pub result: String,
}

/// 查看记录
#[derive(Debug, Deserialize, Default)]
pub struct ViewRecord {
    /// 记录ID
    pub record_id: String,
    /// 用户信息
    pub user: UserInfo,
    /// 查看时间
    pub view_time: String,
    /// 查看类型
    pub view_type: String,
    /// 查看时长（秒）
    pub duration: Option<u32>,
    /// IP地址
    pub ip_address: Option<String>,
    /// 设备信息
    pub device_info: Option<DeviceInfo>,
}

/// 用户信息
#[derive(Debug, Deserialize, Default)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 邮箱
    pub email: Option<String>,
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

/// 设备信息
#[derive(Debug, Deserialize, Default)]
pub struct DeviceInfo {
    /// 设备类型
    pub device_type: String,
    /// 操作系统
    pub os: String,
    /// 浏览器
    pub browser: Option<String>,
}

/// 获取文件查看记录
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/view_records
pub async fn get_file_view_records(
    request: GetFileViewRecordsRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetFileViewRecordsResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/files/{}/view_records",
        config.base_url, request.file_token
    );

    let mut query_params = vec![];

    if let Some(page_size) = request.page_size {
        query_params.push(("page_size".to_string(), page_size.to_string()));
    }

    if let Some(page_token) = &request.page_token {
        query_params.push(("page_token".to_string(), page_token.clone()));
    }

    if let Some(view_type) = &request.view_type {
        query_params.push(("view_type".to_string(), view_type.clone()));
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
    async fn test_get_file_view_records() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetFileViewRecordsRequest {
            file_token: "test_file_token".to_string(),
            page_size: Some(20),
            page_token: None,
            view_type: Some("preview".to_string()),
        };

        let result = get_file_view_records(request, &config, None).await;
        assert!(result.is_ok());
    }
}