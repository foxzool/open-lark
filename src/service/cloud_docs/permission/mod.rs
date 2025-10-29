//! 权限管理模块,
//!,
//! 提供云文档权限相关功能：,
//! - 协作者权限管理（增删改查、批量操作、权限检查、所有者转移等）,
//! - 公开权限设置（v1/v2版本）,
pub mod member;
pub mod public_v1;
pub mod public_v2;
use crate::core::{config::Config, req_option::RequestOption, SDKResult};

use self::{member::*, public_v1::*, public_v2::*};
/// 权限服务
pub struct PermissionService {
    config: Config,
}
impl PermissionService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// 使用共享配置（实验性）
    pub fn new_from_shared() -> Self {
Self {,
            config: (*shared).clone(),
        }
}
/// 批量增加协作者权限
    ///,
/// 批量为云文档添加协作者，支持同时添加多个用户或群组，并设置不同的权限级别。
    /// 适用于需要快速授权多个协作者的场景。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn batch_create_member(,
        &self,
        request: BatchCreatePermissionMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<BatchCreatePermissionMemberResponse>> {
        batch_create_permission_member(request, &self.config, option).await,
}
/// 转移所有者
    ///,
/// 将云文档的所有权转移给其他用户。转移后，原所有者将变为普通协作者，
    /// 新所有者将获得完整的文档管理权限。请谨慎使用此操作。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn transfer_owner(,
        &self,
        request: TransferOwnerRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<TransferOwnerResponse>> {
        transfer_owner(request, &self.config, option).await,
}
/// 判断当前用户是否有某权限
    ///,
/// 检查当前用户对云文档是否有特定权限，如查看、编辑、评论、分享等权限。
    /// 常用于在界面中控制功能按钮的显示和隐藏。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn auth_permission(,
        &self,
        request: AuthPermissionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<AuthPermissionResponse>> {
        auth_permission(request, &self.config, option).await,
}
/// 获取协作者列表
    ///,
/// 获取云文档的所有协作者信息，包括协作者的身份、权限级别、加入时间等。
    /// 支持分页查询和权限类型过滤。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn list_members(,
        &self,
        request: ListPermissionMembersRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<ListPermissionMembersResponse>> {
        list_permission_members(request, &self.config, option).await,
}
/// 增加协作者权限
    ///,
/// 为云文档添加单个协作者，支持指定用户或群组，并设置具体的权限级别。
    /// 可以设置查看、编辑、评论、分享等权限。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn create_member(,
        &self,
        request: CreatePermissionMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<CreatePermissionMemberResponse>> {
        create_permission_member(request, &self.config, option).await,
}
/// 更新协作者权限
    ///,
/// 更新现有协作者的权限级别，可以提升或降低协作者的权限。
    /// 支持修改查看、编辑、评论、分享等各项权限设置。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn update_member(,
        &self,
        request: UpdatePermissionMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<UpdatePermissionMemberResponse>> {
        update_permission_member(request, &self.config, option).await,
}
/// 移除协作者权限
    ///,
/// 从云文档中移除协作者，被移除的用户将失去对该文档的所有访问权限。
    /// 操作不可恢复，请谨慎使用。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn delete_member(,
        &self,
        request: DeletePermissionMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<DeletePermissionMemberResponse>> {
        delete_permission_member(request, &self.config, option).await,
}
/// 获取云文档权限设置
    ///,
/// 获取云文档的公开权限设置，包括文档是否对组织内公开、是否需要密码保护等。
    /// 用于了解文档当前的公开访问状态。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn get_permission_public(,
        &self,
        request: GetPermissionPublicRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<GetPermissionPublicResponse>> {
        get_permission_public(request, &self.config, option).await,
}
/// 更新云文档权限设置
    ///,
/// 更新云文档的公开权限设置，可以设置文档对组织内公开、开启密码保护等。
    /// 支持灵活的权限配置选项。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn patch_permission_public(,
        &self,
        request: PatchPermissionPublicRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<PatchPermissionPublicResponse>> {
        patch_permission_public(request, &self.config, option).await,
}
/// 开启密码保护
    ///,
/// 为云文档开启密码保护，访问者需要输入正确密码才能查看文档内容。
    /// 适用于需要额外安全保护的敏感文档。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn create_password(,
        &self,
        request: CreatePasswordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<CreatePasswordResponse>> {
        create_password(request, &self.config, option).await,
}
/// 刷新密码
    ///,
/// 更新云文档的访问密码，生成新的密码并立即生效。
    /// 原有密码将失效，需要通知用户使用新密码访问。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn update_password(,
        &self,
        request: UpdatePasswordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<UpdatePasswordResponse>> {
        update_password(request, &self.config, option).await,
}
/// 关闭密码保护
    ///,
/// 关闭云文档的密码保护功能，文档将恢复到原有的权限设置状态。
    /// 关闭后，用户不再需要输入密码即可访问文档。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn delete_password(,
        &self,
        request: DeletePasswordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<DeletePasswordResponse>> {
        delete_password(request, &self.config, option).await,
}
/// 获取云文档权限设置 (v2)
    ///,
/// 获取云文档的公开权限设置（v2版本），提供更丰富的权限配置选项和更详细的权限信息。
    /// 推荐在新项目中使用此接口。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn get_permission_public_v2(,
        &self,
        request: GetPermissionPublicV2Request,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<GetPermissionPublicV2Response>> {
        get_permission_public_v2(request, &self.config, option).await,
}
/// 更新云文档权限设置 (v2)
    ///,
/// 更新云文档的公开权限设置（v2版本），支持更细粒度的权限控制和更灵活的配置选项。
    /// 提供比 v1 接口更强的功能性和可定制性。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn patch_permission_public_v2(,
        &self,
        request: PatchPermissionPublicV2Request,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<PatchPermissionPublicV2Response>> {
        patch_permission_public_v2(request, &self.config, option).await,
}
}
