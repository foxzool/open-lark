//! 获取云文档权限设置V2
//!
//! 根据 filetoken 获取文档的公共设置。
//! API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/get-document-sharing-settings-v2
//! 对应CSV记录: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/get-document-sharing-settings-v2

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDrivePermissionApiOld;

/// 获取云文档权限设置V2请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPublicPermissionParams {
    /// 文件token
    #[serde(rename = "token")]
    pub token: String,
}

/// 获取云文档权限设置V2响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPublicPermissionResponse {
    /// 公共权限设置
    pub data: Option<PublicPermission>,
}

/// 公共权限设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicPermission {
    /// 文件token
    #[serde(rename = "token")]
    pub token: String,
    /// 是否公开
    pub public: bool,
    /// 公开链接
    #[serde(rename = "public_url")]
    pub public_url: Option<String>,
    /// 外部用户权限，可选值：view、edit
    #[serde(rename = "external_permission")]
    pub external_permission: Option<String>,
    /// 是否需要密码
    #[serde(rename = "need_password")]
    pub need_password: bool,
    /// 密码过期时间
    #[serde(rename = "password_expire_time")]
    pub password_expire_time: Option<i64>,
}

impl ApiResponseTrait for GetPublicPermissionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档权限设置V2请求
pub struct GetPublicPermissionRequest {
    config: Config,
}

impl GetPublicPermissionRequest {
    /// 创建获取云文档权限设置V2请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/get-document-sharing-settings-v2
    /// 对应CSV记录: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/get-document-sharing-settings-v2
    pub async fn execute(
        self,
        params: GetPublicPermissionParams,
    ) -> SDKResult<GetPublicPermissionResponse> {
        // 验证必填字段
        validate_required!(params.token, "文件token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDrivePermissionApiOld::Public;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetPublicPermissionResponse> =
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