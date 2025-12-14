use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取我的空间（根文件夹）元数据响应
#[derive(Debug, Deserialize, Default)]
pub struct GetRootFolderMetaResponse {
    /// 根文件夹信息
    pub root_folder: FolderMeta,
    /// 操作结果
    pub result: String,
}

/// 文件夹元数据
#[derive(Debug, Deserialize, Default)]
pub struct FolderMeta {
    /// 文件夹token
    pub folder_token: String,
    /// 文件夹名称
    pub name: String,
    /// 文件夹类型
    pub r#type: String,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub update_time: String,
    /// 创建者信息
    pub creator: CreatorInfo,
    /// 父文件夹token
    pub parent_token: Option<String>,
}

/// 创建者信息
#[derive(Debug, Deserialize, Default)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
}

/// 获取我的空间（根文件夹）元数据
/// docPath: https://open.feishu.cn/open-apis/drive/explorer/v2/root_folder/meta
pub async fn get_root_folder_meta(
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetRootFolderMetaResponse>> {
    let url = format!(
        "{}/open-apis/drive/explorer/v2/root_folder/meta",
        config.base_url
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
    async fn test_get_root_folder_meta() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let result = get_root_folder_meta(&config, None).await;
        assert!(result.is_ok());
    }
}