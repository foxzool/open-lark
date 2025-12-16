/// 知识空间管理服务
///
/// 提供飞书知识库空间的管理功能，包括：
/// - 创建知识空间
/// - 获取空间列表和详情
/// - 空间成员管理
/// - 空间设置管理

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::WikiApiV2, api_utils::*};
// pub use member::*;
// pub use setting::*;
// pub use node::*;

pub mod member;
pub mod setting;
pub mod node;
/// 知识空间信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpaceInfo {
    /// 空间唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,
    /// 空间名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 空间描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 空间状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<SpaceMember>,
    /// 空间可见性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl Default for SpaceInfo {
    fn default() -> Self {
        Self {
            space_id: None,
            name: None,
            description: None,
            status: None,
            create_time: None,
            update_time: None,
            creator: None,
            visibility: None,
        }
    }
}

/// 空间成员信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpaceMember {
    /// 成员唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    /// 成员名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl Default for SpaceMember {
    fn default() -> Self {
        Self {
            member_id: None,
            member_type: None,
            name: None,
            email: None,
            avatar: None,
        }
    }
}

/// 获取知识空间列表请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpacesParams {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 获取知识空间列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSpacesResponse {
    /// 空间列表
    pub items: Vec<SpaceInfo>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for GetSpacesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSpaceResponse {
    /// 空间信息
    pub data: SpaceInfo,
}

impl ApiResponseTrait for GetSpaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建知识空间请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpaceParams {
    /// 空间名称
    pub name: String,
    /// 空间描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 空间可见性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

/// 创建知识空间响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSpaceResponse {
    /// 空间信息
    pub data: SpaceInfo,
}

impl ApiResponseTrait for CreateSpaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间成员列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSpaceMembersResponse {
    /// 成员列表
    pub items: Vec<SpaceMember>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for GetSpaceMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加知识空间成员请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSpaceMemberParams {
    /// 成员列表
    pub members: Vec<SpaceMember>,
}

/// 添加知识空间成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddSpaceMemberResponse {
    /// 添加结果
    pub data: Option<AddMemberResult>,
}

/// 添加成员结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMemberResult {
    /// 成功添加的成员数
    pub success_count: i32,
    /// 失败的成员数
    pub failed_count: i32,
}

impl ApiResponseTrait for AddSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除知识空间成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteSpaceMemberResponse {
    /// 删除结果
    pub data: Option<DeleteMemberResult>,
}

/// 删除成员结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMemberResult {
    /// 是否成功删除
    pub success: bool,
}

impl ApiResponseTrait for DeleteSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新知识空间设置请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpaceSettingParams {
    /// 空间名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 空间描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 空间可见性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

/// 更新知识空间设置响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSpaceSettingResponse {
    /// 空间信息
    pub data: SpaceInfo,
}

impl ApiResponseTrait for UpdateSpaceSettingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 知识空间管理服务
#[derive(Clone, Debug)]
pub struct SpaceService {
    config: Config,
}

impl SpaceService {
    /// 创建空间管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取知识空间列表
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/get-space-list
    pub async fn list(&self, params: Option<GetSpacesParams>) -> SDKResult<GetSpacesResponse> {
        let api_endpoint = WikiApiV2::SpaceList;

        let mut api_request: ApiRequest<GetSpacesResponse> = ApiRequest::get(&api_endpoint.to_url());

        // 如果有参数，将其作为请求体而不是查询参数
        if let Some(p) = params {
            api_request = api_request.body(serde_json::json!(&p));
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取知识空间列表")
    }

    /// 获取知识空间信息
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/get-space
    pub async fn get(&self, space_id: &str) -> SDKResult<GetSpaceResponse> {
        validate_required_field("空间ID", Some(space_id), "空间ID不能为空")?;

        let api_endpoint = WikiApiV2::SpaceGet(space_id.to_string());
        let api_request: ApiRequest<GetSpaceResponse> = ApiRequest::get(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取知识空间信息")
    }

    /// 创建知识空间
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/create-space
    pub async fn create(&self, params: CreateSpaceParams) -> SDKResult<CreateSpaceResponse> {
        validate_required_field("空间名称", Some(&params.name), "空间名称不能为空")?;

        let api_endpoint = WikiApiV2::SpaceCreate;
        let api_request: ApiRequest<CreateSpaceResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建知识空间")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "创建知识空间")
    }

    /// 获取知识空间成员列表
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/get-space-member-list
    pub async fn get_members(&self, space_id: &str, page_size: Option<i32>, page_token: Option<String>) -> SDKResult<GetSpaceMembersResponse> {
        validate_required_field("空间ID", Some(space_id), "空间ID不能为空")?;

        let api_endpoint = WikiApiV2::SpaceMemberList(space_id.to_string());
        let mut api_request: ApiRequest<GetSpaceMembersResponse> = ApiRequest::get(&api_endpoint.to_url());

        // 构建查询参数
        if let Some(size) = page_size {
            api_request = api_request.query("page_size", &size.to_string());
        }
        if let Some(token) = page_token {
            api_request = api_request.query("page_token", &token);
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取知识空间成员列表")
    }

    /// 添加知识空间成员
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/add-space-member
    pub async fn add_member(&self, space_id: &str, params: AddSpaceMemberParams) -> SDKResult<AddSpaceMemberResponse> {
        validate_required_field("空间ID", Some(space_id), "空间ID不能为空")?;

        if params.members.is_empty() {
            return Err(openlark_core::error::CoreError::validation(
                "members",
                "成员列表不能为空",
            ));
        }

        let api_endpoint = WikiApiV2::SpaceMemberCreate(space_id.to_string());
        let api_request: ApiRequest<AddSpaceMemberResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "添加知识空间成员")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "添加知识空间成员")
    }

    /// 删除知识空间成员
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/delete-space-member
    pub async fn delete_member(&self, space_id: &str, member_id: &str) -> SDKResult<DeleteSpaceMemberResponse> {
        validate_required_field("空间ID", Some(space_id), "空间ID不能为空")?;
        validate_required_field("成员ID", Some(member_id), "成员ID不能为空")?;

        let api_endpoint = WikiApiV2::SpaceMemberDelete(space_id.to_string(), member_id.to_string(), "user".to_string());
        let api_request: ApiRequest<DeleteSpaceMemberResponse> = ApiRequest::delete(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "删除知识空间成员")
    }

    /// 更新知识空间设置
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/update-space-setting
    pub async fn update_setting(&self, space_id: &str, params: UpdateSpaceSettingParams) -> SDKResult<UpdateSpaceSettingResponse> {
        validate_required_field("空间ID", Some(space_id), "空间ID不能为空")?;

        let api_endpoint = WikiApiV2::SpaceSettingUpdate(space_id.to_string());
        let api_request: ApiRequest<UpdateSpaceSettingResponse> =
            ApiRequest::put(&api_endpoint.to_url()).body(serialize_params(&params, "更新知识空间设置")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "更新知识空间设置")
    }
}

// ==================== 构建器模式 ====================

/// 获取知识空间列表构建器
pub struct GetSpacesBuilder {
    request: GetSpacesParams,
}

impl Default for GetSpacesBuilder {
    fn default() -> Self {
        Self {
            request: GetSpacesParams {
                page_size: None,
                page_token: None,
            },
        }
    }
}

impl GetSpacesBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 执行获取知识空间列表操作
    pub async fn execute(self, service: &SpaceService) -> SDKResult<GetSpacesResponse> {
        service.list(Some(self.request)).await
    }
}

/// 创建知识空间构建器
pub struct CreateSpaceBuilder {
    request: CreateSpaceParams,
}

impl CreateSpaceBuilder {
    /// 创建新的构建器
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            request: CreateSpaceParams {
                name: name.into(),
                description: None,
                visibility: None,
            },
        }
    }

    /// 设置空间描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 设置空间可见性
    pub fn visibility(mut self, visibility: impl Into<String>) -> Self {
        self.request.visibility = Some(visibility.into());
        self
    }

    /// 执行创建知识空间操作
    pub async fn execute(self, service: &SpaceService) -> SDKResult<CreateSpaceResponse> {
        service.create(self.request).await
    }
}

/// 获取知识空间成员列表构建器
pub struct GetSpaceMembersBuilder {
    space_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl GetSpaceMembersBuilder {
    /// 创建新的构建器
    pub fn new(space_id: impl Into<String>) -> Self {
        Self {
            space_id: space_id.into(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行获取知识空间成员列表操作
    pub async fn execute(self, service: &SpaceService) -> SDKResult<GetSpaceMembersResponse> {
        service.get_members(&self.space_id, self.page_size, self.page_token).await
    }
}

/// 添加知识空间成员构建器
pub struct AddSpaceMemberBuilder {
    space_id: String,
    request: AddSpaceMemberParams,
}

impl AddSpaceMemberBuilder {
    /// 创建新的构建器
    pub fn new(space_id: impl Into<String>) -> Self {
        Self {
            space_id: space_id.into(),
            request: AddSpaceMemberParams {
                members: Vec::new(),
            },
        }
    }

    /// 添加成员
    pub fn add_member(mut self, member: SpaceMember) -> Self {
        self.request.members.push(member);
        self
    }

    /// 设置成员列表
    pub fn members(mut self, members: Vec<SpaceMember>) -> Self {
        self.request.members = members;
        self
    }

    /// 执行添加知识空间成员操作
    pub async fn execute(self, service: &SpaceService) -> SDKResult<AddSpaceMemberResponse> {
        service.add_member(&self.space_id, self.request).await
    }
}

/// 更新知识空间设置构建器
pub struct UpdateSpaceSettingBuilder {
    space_id: String,
    request: UpdateSpaceSettingParams,
}

impl UpdateSpaceSettingBuilder {
    /// 创建新的构建器
    pub fn new(space_id: impl Into<String>) -> Self {
        Self {
            space_id: space_id.into(),
            request: UpdateSpaceSettingParams {
                name: None,
                description: None,
                visibility: None,
            },
        }
    }

    /// 设置空间名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
        self
    }

    /// 设置空间描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 设置空间可见性
    pub fn visibility(mut self, visibility: impl Into<String>) -> Self {
        self.request.visibility = Some(visibility.into());
        self
    }

    /// 执行更新知识空间设置操作
    pub async fn execute(self, service: &SpaceService) -> SDKResult<UpdateSpaceSettingResponse> {
        service.update_setting(&self.space_id, self.request).await
    }
}

impl SpaceService {
    /// 创建获取知识空间列表构建器
    pub fn get_spaces_builder(&self) -> GetSpacesBuilder {
        GetSpacesBuilder::new()
    }

    /// 创建创建知识空间构建器
    pub fn create_space_builder(&self, name: impl Into<String>) -> CreateSpaceBuilder {
        CreateSpaceBuilder::new(name)
    }

    /// 创建获取知识空间成员列表构建器
    pub fn get_space_members_builder(&self, space_id: impl Into<String>) -> GetSpaceMembersBuilder {
        GetSpaceMembersBuilder::new(space_id)
    }

    /// 创建添加知识空间成员构建器
    pub fn add_space_member_builder(&self, space_id: impl Into<String>) -> AddSpaceMemberBuilder {
        AddSpaceMemberBuilder::new(space_id)
    }

    /// 创建更新知识空间设置构建器
    pub fn update_space_setting_builder(&self, space_id: impl Into<String>) -> UpdateSpaceSettingBuilder {
        UpdateSpaceSettingBuilder::new(space_id)
    }
}

// 导入节点管理模块
// pub mod nodes; // Generated: Module file not found

// 重新导出nodes模块的公共API
// pub use nodes::*; // Generated: Module use not found

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = SpaceService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_space_info_default_creation() {
        let space_info = SpaceInfo::default();
        assert_eq!(space_info.space_id, None);
        assert_eq!(space_info.name, None);
        assert_eq!(space_info.description, None);
        assert_eq!(space_info.status, None);
        assert_eq!(space_info.visibility, None);
    }

    #[test]
    fn test_space_member_default_creation() {
        let member = SpaceMember::default();
        assert_eq!(member.member_id, None);
        assert_eq!(member.member_type, None);
        assert_eq!(member.name, None);
        assert_eq!(member.email, None);
        assert_eq!(member.avatar, None);
    }

    #[test]
    fn test_get_spaces_builder() {
        let builder = GetSpacesBuilder::new()
            .page_size(20)
            .page_token("token123");

        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(builder.request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_create_space_builder() {
        let builder = CreateSpaceBuilder::new("测试空间")
            .description("这是一个测试空间")
            .visibility("public");

        assert_eq!(builder.request.name, "测试空间");
        assert_eq!(builder.request.description, Some("这是一个测试空间".to_string()));
        assert_eq!(builder.request.visibility, Some("public".to_string()));
    }

    #[test]
    fn test_get_space_members_builder() {
        let builder = GetSpaceMembersBuilder::new("space123")
            .page_size(10)
            .page_token("token456");

        assert_eq!(builder.space_id, "space123");
        assert_eq!(builder.page_size, Some(10));
        assert_eq!(builder.page_token, Some("token456".to_string()));
    }

    #[test]
    fn test_add_space_member_builder() {
        let member = SpaceMember {
            member_id: Some("member123".to_string()),
            member_type: Some("user".to_string()),
            name: Some("张三".to_string()),
            email: Some("zhangsan@example.com".to_string()),
            avatar: None,
        };

        let builder = AddSpaceMemberBuilder::new("space789")
            .add_member(member.clone());

        assert_eq!(builder.space_id, "space789");
        assert_eq!(builder.request.members.len(), 1);
        assert_eq!(builder.request.members[0].member_id, member.member_id);
    }

    #[test]
    fn test_update_space_setting_builder() {
        let builder = UpdateSpaceSettingBuilder::new("space456")
            .name("新空间名称")
            .description("新的空间描述")
            .visibility("private");

        assert_eq!(builder.space_id, "space456");
        assert_eq!(builder.request.name, Some("新空间名称".to_string()));
        assert_eq!(builder.request.description, Some("新的空间描述".to_string()));
        assert_eq!(builder.request.visibility, Some("private".to_string()));
    }

    #[test]
    fn test_response_trait_implementations() {
        assert_eq!(GetSpacesResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetSpaceResponse::data_format(), ResponseFormat::Data);
        assert_eq!(CreateSpaceResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetSpaceMembersResponse::data_format(), ResponseFormat::Data);
        assert_eq!(AddSpaceMemberResponse::data_format(), ResponseFormat::Data);
        assert_eq!(DeleteSpaceMemberResponse::data_format(), ResponseFormat::Data);
        assert_eq!(UpdateSpaceSettingResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_serialization() {
        // 测试SpaceInfo序列化
        let space_info = SpaceInfo {
            space_id: Some("space123".to_string()),
            name: Some("测试空间".to_string()),
            description: Some("测试描述".to_string()),
            status: Some("active".to_string()),
            visibility: Some("public".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&space_info).unwrap();
        let deserialized: SpaceInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(space_info.space_id, deserialized.space_id);
        assert_eq!(space_info.name, deserialized.name);
        assert_eq!(space_info.description, deserialized.description);
        assert_eq!(space_info.status, deserialized.status);
        assert_eq!(space_info.visibility, deserialized.visibility);
    }

    #[test]
    fn test_builder_default_creation() {
        let builder = GetSpacesBuilder::default();
        assert_eq!(builder.request.page_size, None);
        assert_eq!(builder.request.page_token, None);
    }

    #[test]
    fn test_multiple_members_in_builder() {
        let member1 = SpaceMember {
            member_id: Some("member1".to_string()),
            member_type: Some("user".to_string()),
            name: Some("用户1".to_string()),
            ..Default::default()
        };

        let member2 = SpaceMember {
            member_id: Some("member2".to_string()),
            member_type: Some("user".to_string()),
            name: Some("用户2".to_string()),
            ..Default::default()
        };

        let builder = AddSpaceMemberBuilder::new("space123")
            .members(vec![member1.clone(), member2.clone()]);

        assert_eq!(builder.request.members.len(), 2);
        assert_eq!(builder.request.members[0].member_id, member1.member_id);
        assert_eq!(builder.request.members[1].member_id, member2.member_id);
    }
}