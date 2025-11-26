//! 门禁用户管理 API
//!
//! 提供用户信息的增删改查功能。

use std::sync::Arc;

/// 用户管理服务
#[derive(Debug)]
pub struct UsersService {
    config: Arc<crate::models::SecurityConfig>,
}

impl UsersService {
    /// 创建新的用户管理服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self { config }
    }

    /// 获取单个用户信息
    pub fn get(&self) -> GetUserBuilder {
        GetUserBuilder {
            config: self.config.clone(),
            user_id: String::new(),
        }
    }

    /// 获取用户列表
    pub fn list(&self) -> ListUsersBuilder {
        ListUsersBuilder {
            config: self.config.clone(),
            page_size: Some(20),
            page_token: None,
            department_id: None,
            status: None,
        }
    }

    /// 修改用户部分信息
    pub fn patch(&self) -> PatchUserBuilder {
        PatchUserBuilder {
            config: self.config.clone(),
            user_id: String::new(),
            name: None,
            email: None,
            mobile: None,
            department_ids: None,
            status: None,
            rule_ids: None,
        }
    }
}

/// 获取单个用户信息构建器
#[derive(Debug)]
pub struct GetUserBuilder {
    config: Arc<crate::models::SecurityConfig>,
    user_id: String,
}

impl GetUserBuilder {
    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 发送请求获取用户信息
    pub async fn send(self) -> crate::SecurityResult<crate::models::acs::UserInfo> {
        let url = format!(
            "{}/open-apis/acs/v1/users/{}",
            self.config.base_url, self.user_id
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::acs::UserInfo> =
                response.json().await?;
            match api_response.data {
                Some(user) => Ok(user),
                None => Err(crate::SecurityError::APIError {
                    code: api_response.code,
                    message: api_response.msg,
                }),
            }
        } else {
            Err(crate::SecurityError::APIError {
                code: response.status().as_u16() as i32,
                message: format!(
                    "HTTP {}: {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            })
        }
    }
}

/// 获取用户列表构建器
#[derive(Debug)]
pub struct ListUsersBuilder {
    config: Arc<crate::models::SecurityConfig>,
    page_size: Option<i32>,
    page_token: Option<String>,
    department_id: Option<String>,
    status: Option<crate::models::Status>,
}

impl ListUsersBuilder {
    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置部门ID过滤
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = Some(department_id.into());
        self
    }

    /// 设置状态过滤
    pub fn status(mut self, status: crate::models::Status) -> Self {
        self.status = Some(status);
        self
    }

    /// 发送请求获取用户列表
    pub async fn send(self) -> crate::SecurityResult<crate::models::acs::UserListResponse> {
        let mut url = format!("{}/open-apis/acs/v1/users", self.config.base_url);
        let mut query_params = Vec::new();

        if let Some(page_size) = self.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &self.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(department_id) = &self.department_id {
            query_params.push(format!("department_id={}", department_id));
        }
        if let Some(status) = &self.status {
            query_params.push(format!(
                "status={}",
                serde_json::to_string(status).unwrap_or_default()
            ));
        }

        if !query_params.is_empty() {
            url.push_str(&format!("?{}", query_params.join("&")));
        }

        let response = reqwest::Client::new()
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::acs::UserListResponse> =
                response.json().await?;
            match api_response.data {
                Some(users) => Ok(users),
                None => Err(crate::SecurityError::APIError {
                    code: api_response.code,
                    message: api_response.msg,
                }),
            }
        } else {
            Err(crate::SecurityError::APIError {
                code: response.status().as_u16() as i32,
                message: format!(
                    "HTTP {}: {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            })
        }
    }
}

/// 修改用户信息构建器
#[derive(Debug)]
pub struct PatchUserBuilder {
    config: Arc<crate::models::SecurityConfig>,
    user_id: String,
    name: Option<String>,
    email: Option<String>,
    mobile: Option<String>,
    department_ids: Option<Vec<String>>,
    status: Option<crate::models::Status>,
    rule_ids: Option<Vec<String>>,
}

impl PatchUserBuilder {
    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置用户姓名
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置用户邮箱
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// 设置用户手机号
    pub fn mobile(mut self, mobile: impl Into<String>) -> Self {
        self.mobile = Some(mobile.into());
        self
    }

    /// 设置部门ID列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.department_ids = Some(department_ids);
        self
    }

    /// 设置用户状态
    pub fn status(mut self, status: crate::models::Status) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置权限组ID列表
    pub fn rule_ids(mut self, rule_ids: Vec<String>) -> Self {
        self.rule_ids = Some(rule_ids);
        self
    }

    /// 发送请求修改用户信息
    pub async fn send(self) -> crate::SecurityResult<crate::models::acs::UserInfo> {
        let url = format!(
            "{}/open-apis/acs/v1/users/{}",
            self.config.base_url, self.user_id
        );

        let mut request_body = serde_json::Map::new();

        if let Some(name) = self.name {
            request_body.insert("name".to_string(), serde_json::Value::String(name));
        }
        if let Some(email) = self.email {
            request_body.insert("email".to_string(), serde_json::Value::String(email));
        }
        if let Some(mobile) = self.mobile {
            request_body.insert("mobile".to_string(), serde_json::Value::String(mobile));
        }
        if let Some(department_ids) = self.department_ids {
            request_body.insert(
                "department_ids".to_string(),
                serde_json::Value::Array(
                    department_ids
                        .into_iter()
                        .map(serde_json::Value::String)
                        .collect(),
                ),
            );
        }
        if let Some(status) = self.status {
            request_body.insert(
                "status".to_string(),
                serde_json::to_value(status).unwrap_or(serde_json::Value::Null),
            );
        }
        if let Some(rule_ids) = self.rule_ids {
            request_body.insert(
                "rule_ids".to_string(),
                serde_json::Value::Array(
                    rule_ids
                        .into_iter()
                        .map(serde_json::Value::String)
                        .collect(),
                ),
            );
        }

        let response = reqwest::Client::new()
            .patch(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::acs::UserInfo> =
                response.json().await?;
            match api_response.data {
                Some(user) => Ok(user),
                None => Err(crate::SecurityError::APIError {
                    code: api_response.code,
                    message: api_response.msg,
                }),
            }
        } else {
            Err(crate::SecurityError::APIError {
                code: response.status().as_u16() as i32,
                message: format!(
                    "HTTP {}: {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            })
        }
    }
}

/// 获取应用访问令牌的辅助函数
async fn get_app_token(_config: &crate::models::SecurityConfig) -> crate::SecurityResult<String> {
    // 这里应该调用认证服务获取应用访问令牌
    // 为了简化实现，暂时返回占位符
    // TODO: 集成 openlark-auth 服务
    Ok("placeholder_app_token".to_string())
}
