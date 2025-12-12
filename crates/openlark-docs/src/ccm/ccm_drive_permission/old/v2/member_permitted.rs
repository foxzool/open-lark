//! 判断协作者是否有某权限
//!
//! 根据 filetoken 判断当前登录用户是否具有某权限。
//! API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/querying-if-a-collaborator-has-a-specific-permission
//! 对应CSV记录: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/querying-if-a-collaborator-has-a-specific-permission

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDrivePermissionApiOld;

/// 判断协作者是否有某权限请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckMemberPermittedParams {
    /// 文件token
    #[serde(rename = "token")]
    pub token: String,
    /// 权限类型，可选值：view, edit, comment, share, download, print
    #[serde(rename = "permission")]
    pub permission: String,
}

/// 判断协作者是否有某权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckMemberPermittedResponse {
    /// 权限检查结果
    pub data: Option<PermissionResult>,
}

/// 权限检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionResult {
    /// 是否有权限
    pub permitted: bool,
    /// 权限类型
    #[serde(rename = "permission")]
    pub permission: String,
}

impl ApiResponseTrait for CheckMemberPermittedResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 判断协作者是否有某权限请求
pub struct CheckMemberPermittedRequest {
    config: Config,
}

impl CheckMemberPermittedRequest {
    /// 创建判断协作者是否有某权限请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/querying-if-a-collaborator-has-a-specific-permission
    /// 对应CSV记录: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/querying-if-a-collaborator-has-a-specific-permission
    pub async fn execute(
        self,
        params: CheckMemberPermittedParams,
    ) -> SDKResult<CheckMemberPermittedResponse> {
        // 验证必填字段
        validate_required!(params.token, "文件token不能为空");
        validate_required!(params.permission, "权限类型不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDrivePermissionApiOld::MemberPermitted;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<CheckMemberPermittedResponse> =
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