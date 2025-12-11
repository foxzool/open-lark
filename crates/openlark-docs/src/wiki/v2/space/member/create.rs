//! 添加知识空间成员
//!
//! 为知识空间添加成员。
//! 文档参考：https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::WikiApiV2;
use crate::wiki::v2::models::WikiSpaceMember;

/// 添加知识空间成员请求
pub struct CreateWikiSpaceMemberRequest {
    space_id: String,
    config: Config,
}

/// 添加知识空间成员请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceMemberParams {
    /// 成员类型 ("user" | "bot" | "group")
    pub member_type: String,
    /// 用户ID (与open_id二选一)
    pub user_id: Option<String>,
    /// Open ID (与user_id二选一)
    pub open_id: Option<String>,
    /// Union ID (跨平台ID)
    pub union_id: Option<String>,
    /// 权限级别
    pub perm: Option<String>,
}

/// 添加知识空间成员响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceMemberResponse {
    /// 成员信息
    pub data: Option<WikiSpaceMember>,
}

impl ApiResponseTrait for CreateWikiSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateWikiSpaceMemberRequest {
    /// 创建添加知识空间成员请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/create
    pub async fn execute(
        self,
        params: CreateWikiSpaceMemberParams,
    ) -> SDKResult<CreateWikiSpaceMemberResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(params.member_type, "成员类型不能为空");

        // 验证用户标识符 (user_id, open_id, union_id 至少有一个)
        if params.user_id.is_none() && params.open_id.is_none() && params.union_id.is_none() {
            return Err(openlark_core::error::validation_error(
                "用户标识符不能为空",
                "user_id、open_id、union_id 至少需要提供一个",
            ));
        }

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceMemberCreate(self.space_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<CreateWikiSpaceMemberResponse> =
            ApiRequest::post(&api_endpoint.to_url());

        // 设置请求体
        api_request.body = Some(openlark_core::api::RequestData::Json(serde_json::to_value(
            &params,
        )?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
