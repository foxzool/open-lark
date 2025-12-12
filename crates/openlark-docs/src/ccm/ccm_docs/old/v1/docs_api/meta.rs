//! 获取元数据
//!
//! 根据 token 获取各类文件的元数据。
//! API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata
//! 对应CSV记录: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocsApiOld;

/// 获取元数据请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetadataParams {
    /// 文件token
    pub token: String,
    /// 文件类型，可选值：doc、sheet、bitable、mindnote、file
    #[serde(rename = "doc_type")]
    pub doc_type: Option<String>,
}

/// 获取元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetadataResponse {
    /// 文件元数据
    pub data: Option<Metadata>,
}

/// 文件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// 文件token
    pub token: String,
    /// 文件标题
    pub title: String,
    /// 文件类型
    #[serde(rename = "doc_type")]
    pub doc_type: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
    /// 创建者信息
    pub creator: Option<UserInfo>,
    /// 更新者信息
    pub updater: Option<UserInfo>,
    /// 父节点token
    #[serde(rename = "parent_token")]
    pub parent_token: Option<String>,
    /// 文件大小（字节）
    #[serde(rename = "file_size")]
    pub file_size: Option<i64>,
    /// 预览URL
    #[serde(rename = "preview_url")]
    pub preview_url: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名称
    pub name: String,
}

impl ApiResponseTrait for GetMetadataResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取元数据请求
pub struct GetMetadataRequest {
    config: Config,
}

impl GetMetadataRequest {
    /// 创建获取元数据请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata
    /// 对应CSV记录: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata
    pub async fn execute(
        self,
        params: GetMetadataParams,
    ) -> SDKResult<GetMetadataResponse> {
        // 验证必填字段
        validate_required!(params.token, "文件token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDocsApiOld::Meta;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetMetadataResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serde_json::to_value(params).map_err(|e| {
                    openlark_core::error::validation_error(
                        "参数序列化失败",
                        &format!("无法序列化请求参数: {}", e)
                    )
                })?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}