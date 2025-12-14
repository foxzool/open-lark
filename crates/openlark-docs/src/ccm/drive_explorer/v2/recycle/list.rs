use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取回收站列表请求
#[derive(Debug, Serialize, Default)]
pub struct GetRecycleListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 文件类型过滤
    pub file_type: Option<String>,
}

/// 获取回收站列表响应
#[derive(Debug, Deserialize, Default)]
pub struct GetRecycleListResponse {
    /// 文件列表
    pub files: Vec<FileMeta>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 操作结果
    pub result: String,
}

/// 文件元数据
#[derive(Debug, Deserialize, Default)]
pub struct FileMeta {
    /// 文件token
    pub file_token: String,
    /// 文件名称
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub update_time: String,
    /// 删除时间
    pub delete_time: String,
    /// 创建者信息
    pub creator: CreatorInfo,
}

/// 创建者信息
#[derive(Debug, Deserialize, Default)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
}

/// 获取回收站列表
/// docPath: https://open.feishu.cn/open-apis/drive/explorer/v2/recycle/list
pub async fn get_recycle_list(
    request: GetRecycleListRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetRecycleListResponse>> {
    let url = format!(
        "{}/open-apis/drive/explorer/v2/recycle/list",
        config.base_url
    );

    let mut query_params = vec![];

    if let Some(page_size) = request.page_size {
        query_params.push(("page_size".to_string(), page_size.to_string()));
    }

    if let Some(page_token) = &request.page_token {
        query_params.push(("page_token".to_string(), page_token.clone()));
    }

    if let Some(file_type) = &request.file_type {
        query_params.push(("file_type".to_string(), file_type.clone()));
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
    async fn test_get_recycle_list() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetRecycleListRequest {
            page_size: Some(20),
            page_token: None,
            file_type: Some("folder".to_string()),
        };

        let result = get_recycle_list(request, &config, None).await;
        assert!(result.is_ok());
    }
}