use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

pub struct PermissionsService {
    config: Config,
}

impl PermissionsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取云文档权限设置
    pub async fn get(
        &self,
        request: GetPermissionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetPermissionResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = format!("/open-apis/drive/v2/permissions/{}/public", request.token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 更新云文档权限设置
    pub async fn patch(
        &self,
        request: PatchPermissionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetPermissionResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PATCH;
        api_req.api_path = format!("/open-apis/drive/v2/permissions/{}/public", request.token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 获取云文档权限设置
#[derive(Debug, Default)]
pub struct GetPermissionRequest {
    api_request: ApiRequest,
    /// 文件的 token
    token: String,
    /// 文件类型，需要与文件的 token 相匹配
    ///
    /// 示例值："doc"
    ///
    /// 可选值有：
    ///
    /// - doc：旧版文档
    /// - sheet：电子表格
    /// - file：云空间文件
    /// - wiki：知识库节点
    /// - bitable：多维表格
    /// - docx：新版文档
    /// - mindnote：思维笔记
    /// - minutes：妙记
    /// - slides：幻灯片
    r#type: String,
}

impl GetPermissionRequest {
    pub fn builder() -> GetPermissionRequestBuilder {
        GetPermissionRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct GetPermissionRequestBuilder {
    request: GetPermissionRequest,
}

impl GetPermissionRequestBuilder {
    /// 文件的 token
    pub fn token(mut self, token: impl ToString) -> Self {
        self.request.token = token.to_string();
        self
    }

    /// 文件类型，需要与文件的 token 相匹配
    ///
    /// 示例值："doc"
    ///
    /// 可选值有：
    ///
    /// - doc：旧版文档
    /// - sheet：电子表格
    /// - file：云空间文件
    /// - wiki：知识库节点
    /// - bitable：多维表格
    /// - docx：新版文档
    /// - mindnote：思维笔记
    /// - minutes：妙记
    /// - slides：幻灯片
    pub fn r#type(mut self, r#type: impl ToString) -> Self {
        self.request.r#type = r#type.to_string();
        self.request
            .api_request
            .query_params
            .insert("type".to_string(), r#type.to_string());
        self
    }

    pub fn build(self) -> GetPermissionRequest {
        self.request
    }
}

/// 返回的文档公共设置
#[derive(Debug, Deserialize)]
pub struct GetPermissionResponse {
    /// 返回的文档公共设置
    pub permission_public: PermissionPublic,
}

impl ApiResponseTrait for GetPermissionResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Deserialize)]
/// 返回的文档公共设置
pub struct PermissionPublic {
    /// 允许内容被分享到组织外
    ///
    /// 可选值有：
    ///
    /// - open：打开
    /// - closed：关闭
    /// - allow_share_partner_tenant：允许分享给关联组织
    pub external_access_entity: Option<String>,
    /// 谁可以创建副本、打印、下载
    ///
    /// 可选值有：
    ///
    /// - anyone_can_view：拥有可阅读权限的用户
    /// - anyone_can_edit：拥有可编辑权限的用户
    /// - only_full_access：拥有可管理权限（包括我）的用户
    pub security_entity:  Option<String>,
    /// 谁可以评论
    ///
    /// 可选值有：
    ///
    /// - anyone_can_view：拥有可阅读权限的用户
    /// - anyone_can_edit：拥有可编辑权限的用户
    pub comment_entity: Option<String>,
    /// 谁可以添加和管理协作者-组织维度
    ///
    /// 可选值有：
    ///
    /// - anyone：所有可阅读或编辑此文档的用户
    /// - same_tenant：组织内所有可阅读或编辑此文档的用户
    pub share_entity: Option<String>,
    /// 谁可以添加和管理协作者-协作者维度
    ///
    /// 可选值有：
    ///
    /// - collaborator_can_view：拥有可阅读权限的协作者
    /// - collaborator_can_edit：拥有可编辑权限的协作者
    /// - collaborator_full_access：拥有可管理权限（包括我）的协作者
    pub manage_collaborator_entity: Option<String>,
    /// 链接分享设置
    ///
    /// 可选值有：
    ///
    /// - tenant_readable：组织内获得链接的人可阅读
    /// - tenant_editable：组织内获得链接的人可编辑
    /// - partner_tenant_readable：关联组织的人可阅读
    /// - partner_tenant_editable：关联组织的人可编辑
    /// - anyone_readable：互联网上获得链接的任何人可阅读（仅external_access=“open”时有效）
    /// - anyone_editable：互联网上获得链接的任何人可编辑（仅external_access=“open”时有效）
    /// - closed：关闭链接分享
    pub link_share_entity: Option<String>,
    /// 谁可以复制内容
    ///
    /// 可选值有：
    ///
    /// - anyone_can_view：拥有可阅读权限的用户
    /// - anyone_can_edit：拥有可编辑权限的用户
    /// - only_full_access：拥有可管理权限（包括我）的协作者
    pub copy_entity: Option<String>,
    /// 节点是否已加锁，加锁之后不再继承父级页面的权限
    pub lock_switch: Option<bool>,
}

#[derive(Debug, Default, Serialize)]
pub struct PatchPermissionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文件的 token
    #[serde(skip)]
    token: String,
    /// 允许内容被分享到组织外
    ///
    /// 示例值："open"
    ///
    /// 可选值有：
    ///
    /// - open：打开
    /// - closed：关闭
    /// - allow_share_partner_tenant：允许分享给关联组织（只有租户后台设置仅允许关联组织分享，
    /// 才能设置为该值）
    #[serde(skip_serializing_if = "Option::is_none")]
    external_access_entity: Option<String>,
    /// 谁可以创建副本、打印、下载
    ///
    /// 示例值："anyone_can_view"
    ///
    /// 可选值有：
    ///
    /// - anyone_can_view：拥有可阅读权限的用户
    /// - anyone_can_edit：拥有可编辑权限的用户
    /// - only_full_access：拥有可管理权限（包括我）的用户
    #[serde(skip_serializing_if = "Option::is_none")]
    security_entity: Option<String>,
    /// 谁可以评论
    ///
    /// 示例值："anyone_can_view"
    ///
    /// 可选值有：
    ///
    /// - anyone_can_view：拥有可阅读权限的用户
    /// - anyone_can_edit：拥有可编辑权限的用户
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_entity: Option<String>,
    /// 谁可以添加和管理协作者-组织维度
    ///
    /// 示例值："anyone"
    ///
    /// 可选值有：
    ///
    /// - anyone：所有可阅读或编辑此文档的用户
    /// - same_tenant：组织内所有可阅读或编辑此文档的用户
    #[serde(skip_serializing_if = "Option::is_none")]
    share_entity: Option<String>,
    /// 谁可以添加和管理协作者-协作者维度
    ///
    /// 示例值："collaborator_can_view"
    ///
    /// 可选值有：
    ///
    /// - collaborator_can_view：拥有可阅读权限的协作者
    /// - collaborator_can_edit：拥有可编辑权限的协作者
    /// - collaborator_full_access：拥有可管理权限（包括我）的协作者
    #[serde(skip_serializing_if = "Option::is_none")]
    manage_collaborator_entity: Option<String>,
    /// 链接分享设置
    ///
    /// 示例值："tenant_readable"
    ///
    /// 可选值有：
    ///
    /// tenant_readable：组织内获得链接的人可阅读
    /// tenant_editable：组织内获得链接的人可编辑
    /// partner_tenant_readable：关联组织的人可阅读（只有租户后台设置仅允许关联组织分享，
    /// 才能设置为该值） partner_tenant_editable：
    /// 关联组织的人可编辑（只有租户后台设置仅允许关联组织分享，才能设置为该值）
    /// anyone_readable：互联网上获得链接的任何人可阅读（仅external_access_entity=“open”时有效）
    /// anyone_editable：互联网上获得链接的任何人可编辑（仅external_access_entity=“open”时有效）
    /// closed：关闭链接分享
    #[serde(skip_serializing_if = "Option::is_none")]
    link_share_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_entity: Option<String>,
}

impl PatchPermissionRequest {
    pub fn builder() -> PatchPermissionRequestBuilder {
        PatchPermissionRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct PatchPermissionRequestBuilder {
    request: PatchPermissionRequest,
}

impl PatchPermissionRequestBuilder {
    /// 文件的 token
    pub fn token(mut self, token: impl ToString) -> Self {
        self.request.token = token.to_string();
        self
    }

    /// 文件类型，需要与文件的 token 相匹配
    ///
    /// 示例值："doc"
    ///
    /// 可选值有：
    ///
    /// - doc：旧版文档
    /// - sheet：电子表格
    /// - file：云空间文件
    /// - wiki：知识库节点
    /// - bitable：多维表格
    /// - docx：新版文档
    /// - mindnote：思维笔记
    /// - minutes：妙记
    /// - slides：幻灯片
    pub fn r#type(mut self, r#type: impl ToString) -> Self {
        self.request
            .api_request
            .query_params
            .insert("type".to_string(), r#type.to_string());
        self
    }

    /// 允许内容被分享到组织外
    ///
    /// 示例值："open"
    ///
    /// 可选值有：
    ///
    /// - open：打开
    /// - closed：关闭
    /// - allow_share_partner_tenant：允许分享给关联组织（只有租户后台设置仅允许关联组织分享，
    /// 才能设置为该值）
    pub fn external_access_entity(mut self, external_access_entity: impl ToString) -> Self {
        self.request.external_access_entity = Some(external_access_entity.to_string());
        self
    }

    /// 谁可以创建副本、打印、下载
    ///
    /// 示例值："anyone_can_view"
    ///
    /// 可选值有：
    ///
    /// - anyone_can_view：拥有可阅读权限的用户
    /// - anyone_can_edit：拥有可编辑权限的用户
    /// - only_full_access：拥有可管理权限（包括我）的用户
    pub fn security_entity(mut self, security_entity: impl ToString) -> Self {
        self.request.security_entity = Some(security_entity.to_string());
        self
    }

    /// 谁可以评论
    ///
    /// 示例值："anyone_can_view"
    ///
    /// 可选值有：
    ///
    /// - anyone_can_view：拥有可阅读权限的用户
    /// - anyone_can_edit：拥有可编辑权限的用户
    pub fn comment_entity(mut self, comment_entity: impl ToString) -> Self {
        self.request.comment_entity = Some(comment_entity.to_string());
        self
    }

    /// 谁可以添加和管理协作者-组织维度
    ///
    /// 示例值："anyone"
    ///
    /// 可选值有：
    ///
    /// - anyone：所有可阅读或编辑此文档的用户
    /// - same_tenant：组织内所有可阅读或编辑此文档的用户
    pub fn share_entity(mut self, share_entity: impl ToString) -> Self {
        self.request.share_entity = Some(share_entity.to_string());
        self
    }

    /// 谁可以添加和管理协作者-协作者维度
    ///
    /// 示例值："collaborator_can_view"
    ///
    /// 可选值有：
    ///
    /// - collaborator_can_view：拥有可阅读权限的协作者
    /// - collaborator_can_edit：拥有可编辑权限的协作者
    /// - collaborator_full_access：拥有可管理权限（包括我）的协作者
    pub fn manage_collaborator_entity(mut self, manage_collaborator_entity: impl ToString) -> Self {
        self.request.manage_collaborator_entity = Some(manage_collaborator_entity.to_string());
        self
    }

    /// 链接分享设置
    ///
    /// 示例值："tenant_readable"
    ///
    /// 可选值有：
    ///
    /// tenant_readable：组织内获得链接的人可阅读
    /// tenant_editable：组织内获得链接的人可编辑
    /// partner_tenant_readable：关联组织的人可阅读（只有租户后台设置仅允许关联组织分享，
    /// 才能设置为该值） partner_tenant_editable：
    /// 关联组织的人可编辑（只有租户后台设置仅允许关联组织分享，才能设置为该值）
    /// anyone_readable：互联网上获得链接的任何人可阅读（仅external_access_entity=“open”时有效）
    /// anyone_editable：互联网上获得链接的任何人可编辑（仅external_access_entity=“open”时有效）
    /// closed：关闭链接分享
    pub fn link_share_entity(mut self, link_share_entity: impl ToString) -> Self {
        self.request.link_share_entity = Some(link_share_entity.to_string());
        self
    }

    /// 谁可以复制内容
    ///
    /// 可选值有：
    ///
    /// - anyone_can_view：拥有可阅读权限的用户
    /// - anyone_can_edit：拥有可编辑权限的用户
    /// - only_full_access：拥有可管理权限（包括我）的协作者
    pub fn copy_entity(mut self, copy_entity: impl ToString) -> Self {
        self.request.copy_entity = Some(copy_entity.to_string());
        self
    }

    pub fn build(mut self) -> PatchPermissionRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}
