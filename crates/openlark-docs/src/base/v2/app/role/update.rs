/// Base 更新自定义角色API
///
/// API文档: https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/update-2
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{models::RoleResponse as Role, RoleService};

/// 更新自定义角色请求体（内部使用）
#[derive(Serialize)]
pub struct UpdateRoleRequestBody {
    /// 角色名称
    pub role_name: Option<String>,
    /// 表格角色配置列表
    pub table_roles: Option<Vec<super::models::TableRole>>,
}

impl UpdateRoleRequestBody {
    /// 验证更新角色请求体
    pub fn validate(&self) -> Result<(), String> {
        if let Some(role_name) = &self.role_name {
            if role_name.trim().is_empty() {
                return Err("角色名称不能为空".to_string());
            }

            if role_name.len() > 100 {
                return Err("角色名称长度不能超过100个字符".to_string());
            }
        }

        if let Some(table_roles) = &self.table_roles {
            if table_roles.len() > 100 {
                return Err("表格角色数量不能超过100个".to_string());
            }

            for table_role in table_roles {
                if let Err(e) = table_role.validate() {
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}

/// 更新自定义角色请求
pub struct UpdateRoleRequest {
    app_token: String,
    role_id: String,
    /// 角色名称
    role_name: Option<String>,
    /// 表格角色配置列表
    table_roles: Option<Vec<super::models::TableRole>>,
    /// 配置信息
    config: Config,
}

/// 更新自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRoleResponse {
    /// 角色信息
    pub data: Role,
}

impl ApiResponseTrait for UpdateRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateRoleRequest {
    /// 创建更新自定义角色请求
    pub fn new(config: Config) -> Self {
        Self {
            app_token: String::new(),
            role_id: String::new(),
            role_name: None,
            table_roles: None,
            config,
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置角色 ID
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 设置角色名称
    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.role_name = Some(role_name.into());
        self
    }

    /// 设置表格角色配置列表
    pub fn table_roles(mut self, table_roles: Vec<super::models::TableRole>) -> Self {
        self.table_roles = Some(table_roles);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateRoleResponse> {
        // 验证必填字段
        validate_required!(self.app_token, "应用令牌不能为空");
        validate_required!(self.role_id, "角色ID不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::BaseApiV2;
        let api_endpoint = BaseApiV2::RoleUpdate(self.app_token.clone(), self.role_id.clone());

        // 构建请求体 - 符合官方文档格式
        let request_body = UpdateRoleRequestBody {
            role_name: self.role_name,
            table_roles: self.table_roles,
        };

        // 验证请求参数
        if let Err(e) = request_body.validate() {
            return Err(openlark_core::error::validation_error(
                "更新角色请求验证失败",
                e,
            ));
        }

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<UpdateRoleResponse> = ApiRequest::put(&api_endpoint.to_url())
            .body(openlark_core::api::RequestData::Json(serde_json::to_value(
                &request_body,
            )?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl RoleService {
    /// 创建更新自定义角色请求
    pub fn update_role_builder(
        &self,
        app_token: impl Into<String>,
        role_id: impl Into<String>,
    ) -> UpdateRoleRequest {
        UpdateRoleRequest::new(self.config.clone())
            .app_token(app_token)
            .role_id(role_id)
    }

    /// 创建更新自定义角色请求（带参数）
    pub fn update_role(
        &self,
        app_token: impl Into<String>,
        role_id: impl Into<String>,
        role_name: Option<impl Into<String>>,
        table_roles: Option<Vec<super::models::TableRole>>,
    ) -> UpdateRoleRequest {
        let mut request = UpdateRoleRequest::new(self.config.clone())
            .app_token(app_token)
            .role_id(role_id);

        if let Some(name) = role_name {
            request = request.role_name(name);
        }

        if let Some(roles) = table_roles {
            request = request.table_roles(roles);
        }

        request
    }
}
