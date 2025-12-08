//! 删除知识空间成员
//!
//! 从知识空间删除成员。
//! 文档参考：https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::wiki::v2::models::WikiSpaceMember;
use crate::common::api_endpoints::WikiApiV2;

/// 删除知识空间成员请求
pub struct DeleteWikiSpaceMemberRequest {
    space_id: String,
    member_id: String,
    member_type: String,
    config: Config,
}

/// 删除知识空间成员响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWikiSpaceMemberResponse {
    /// 删除的成员信息
    pub data: Option<WikiSpaceMember>,
}

impl ApiResponseTrait for DeleteWikiSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteWikiSpaceMemberRequest {
    /// 创建删除知识空间成员请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            member_id: String::new(),
            member_type: String::new(),
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 设置成员ID
    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.member_id = member_id.into();
        self
    }

    /// 设置成员类型
    pub fn member_type(mut self, member_type: impl Into<String>) -> Self {
        self.member_type = member_type.into();
        self
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/delete
    pub async fn execute(self) -> SDKResult<DeleteWikiSpaceMemberResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(self.member_id, "成员ID不能为空");
        validate_required!(self.member_type, "成员类型不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceMemberDelete(self.space_id.clone(), self.member_id.clone(), self.member_type.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<DeleteWikiSpaceMemberResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}