/// 转移拥有者
///
/// 根据文档信息和用户信息转移文档的所有者。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/transfer-ownership

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDrivePermissionApiOld;

/// 转移拥有者请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOwnershipParams {
    /// 文件token
    #[serde(rename = "token")]
    pub token: String,
    /// 新拥有者用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户ID类型，可选值：user_id、open_id、union_id
    #[serde(rename = "user_id_type")]
    pub user_id_type: String,
}

/// 转移拥有者响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOwnershipResponse {
    /// 转移结果
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for TransferOwnershipResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 转移拥有者请求
pub struct TransferOwnershipRequest {
    config: Config,
}

impl TransferOwnershipRequest {
    /// 创建转移拥有者请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/transfer-ownership
    pub async fn execute(
        self,
        params: TransferOwnershipParams,
    ) -> SDKResult<TransferOwnershipResponse> {
        // 验证必填字段
        validate_required!(params.token, "文件token不能为空");
        validate_required!(params.user_id, "新拥有者用户ID不能为空");
        validate_required!(params.user_id_type, "用户ID类型不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDrivePermissionApiOld::MemberTransfer;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<TransferOwnershipResponse> =
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