/// 获取文件查看记录
///
/// 获取文件的查看记录列表，包括查看者、查看时间等信息。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/view_records
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 获取文件查看记录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl GetFileViewRecordsRequest {
    /// 创建获取文件查看记录请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            page_size: None,
            page_token: None,
            view_type: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置查看类型
    pub fn view_type(mut self, view_type: impl Into<String>) -> Self {
        self.view_type = Some(view_type.into());
        self
    }
}

/// 获取文件查看记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileViewRecordsResponse {
    /// 查看记录数据
    pub data: Option<ViewRecordData>,
}

impl ApiResponseTrait for GetFileViewRecordsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查看记录数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewRecordData {
    /// 查看记录列表
    pub view_records: Vec<ViewRecord>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 查看记录
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarInfo {
    /// 头像URL
    pub url: String,
    /// 头像大小
    pub size: Option<u32>,
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备类型
    pub device_type: String,
    /// 操作系统
    pub os: String,
    /// 浏览器
    pub browser: Option<String>,
}

/// 获取文件查看记录
///
/// 获取文件的查看记录列表，包括查看者、查看时间等信息。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/view_records
pub async fn get_file_view_records(
    request: GetFileViewRecordsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetFileViewRecordsResponse>> {
    // 创建API请求
    let url = DriveApi::ListFileViewRecords(request.file_token.clone()).to_url();
    let mut api_request: ApiRequest<GetFileViewRecordsResponse> = ApiRequest::get(&url);

    // 添加查询参数
    if let Some(page_size) = request.page_size {
        api_request = api_request.query_param("page_size", &page_size.to_string());
    }
    if let Some(page_token) = &request.page_token {
        api_request = api_request.query_param("page_token", page_token);
    }
    if let Some(view_type) = &request.view_type {
        api_request = api_request.query_param("view_type", view_type);
    }

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
    fn test_get_file_view_records_request_builder() {
        let request = GetFileViewRecordsRequest::new("file_token")
            .page_size(20)
            .page_token("token123")
            .view_type("preview");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("token123".to_string()));
        assert_eq!(request.view_type, Some("preview".to_string()));
    }

    #[test]
    fn test_view_record_structure() {
        let user = UserInfo {
            user_id: "user_123".to_string(),
            name: "张三".to_string(),
            email: Some("zhangsan@example.com".to_string()),
            avatar: None,
        };

        let device_info = DeviceInfo {
            device_type: "desktop".to_string(),
            os: "Windows 10".to_string(),
            browser: Some("Chrome".to_string()),
        };

        let record = ViewRecord {
            record_id: "record_456".to_string(),
            user,
            view_time: "2023-01-01T10:00:00Z".to_string(),
            view_type: "preview".to_string(),
            duration: Some(300),
            ip_address: Some("192.168.1.1".to_string()),
            device_info: Some(device_info),
        };

        assert_eq!(record.record_id, "record_456");
        assert_eq!(record.view_type, "preview");
        assert_eq!(record.duration, Some(300));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            GetFileViewRecordsResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
