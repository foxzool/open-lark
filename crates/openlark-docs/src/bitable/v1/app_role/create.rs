#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
use super::AppRoleService;
impl AppRoleService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 新增自定义角色请求,
#[derive(Clone)]
pub struct CreateAppRoleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 角色名称
    role_name: String,
    /// 数据表权限,
#[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限,
#[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>}
/// 数据表权限,
#[derive(Clone)]
pub struct TableRole {
    /// 数据表 id
    pub table_id: String,
    /// 权限
    pub role: String,
    /// 记录权限,
#[serde(skip_serializing_if = "Option::is_none")]
    rec_rule: Option<String>}
/// 数据表默认权限,
#[derive(Clone)]
pub struct BlockRole {
    /// 多维表格数据表的唯一标识符
    pub block_id: String,
    /// 权限
    pub role: String,
impl CreateAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct CreateAppRoleRequestBuilder {
    request: CreateAppRoleRequest}
impl CreateAppRoleRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    CreateAppRoleRequestBuilder,
    AppRoleService,
    CreateAppRoleRequest,
    Response<CreateAppRoleResponse>,
    create,
);
/// 自定义角色信息
#[derive(Clone)]
pub struct AppRole {
    /// 自定义角色的id
    pub role_id: String,
    /// 角色名称
    pub role_name: String,
    /// 数据表权限
    pub table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    pub block_roles: Option<Vec<BlockRole>>}
/// 新增自定义角色响应,
#[derive(Clone)]
pub struct CreateAppRoleResponse {
    /// 新增的自定义角色信息
    pub role: AppRole,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_create_app_role_request_builder() {
let table_roles = vec![TableRole {,
            table_id: "tblxxxxxx".to_string(),
            role: "editor".to_string(),
            rec_rule: Some("all".to_string())}];
let request = CreateAppRoleRequest::builder(),
            .app_token()
.role_name()
            .table_roles()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_name, "测试自定义角色");
assert!(request.table_roles.is_some());
    }
