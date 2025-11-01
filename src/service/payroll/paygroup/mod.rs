use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::payroll::models::{PageResponse, Paygroup, PaygroupListRequest, PaymentDaySetting},
};
use open_lark_core::core::api_req::ApiRequest;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 薪资组服务
pub struct PaygroupService {
    pub config: Config,
}

/// 薪资组列表响应
#[derive(Debug, Clone)]
pub struct PaygroupListResponse {
    /// 薪资组列表
    pub items: Vec<Paygroup>,
    /// 分页信息
    pub page: PageResponse<Paygroup>,
}

/// 薪资组响应
#[derive(Debug, Clone)]
pub struct PaygroupResponse {
    /// 薪资组信息
    pub data: Paygroup,
}

/// 创建薪资组请求
#[derive(Debug, Clone, Serialize)]
pub struct CreatePaygroupRequest {
    /// 薪资组名称
    pub name: I18nText,
    /// 薪资组类型
    pub paygroup_type: String,
    /// 发薪周期类型
    pub payment_cycle_type: String,
    /// 发薪日设置
    pub payment_day_setting: Option<PaymentDaySetting>,
    /// 描述
    pub description: Option<I18nText>,
}

/// 更新薪资组请求
#[derive(Debug, Clone, Serialize)]
pub struct UpdatePaygroupRequest {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 薪资组名称
    pub name: Option<I18nText>,
    /// 薪资组状态
    pub status: Option<String>,
    /// 发薪日设置
    pub payment_day_setting: Option<PaymentDaySetting>,
    /// 描述
    pub description: Option<I18nText>,
}

/// I18n文本（用于薪资组）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct I18nText {
    /// 中文
    pub zh_cn: Option<String>,
    /// 英文
    pub en_us: Option<String>,
    /// 日文
    pub ja_jp: Option<String>,
}

impl Service for PaygroupService {
    fn config(&self) -> &Config {
        &self.config
    }
}

impl PaygroupService {
    /// 创建薪资组服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询薪资组列表
    ///
    /// # API文档
    ///
    /// 查询薪资组列表，支持分页和状态筛选。
    ///
    /// # 参数
    ///
    /// * `request` - 查询请求参数
    ///
    /// # 返回值
    ///
    /// 返回薪资组列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::paygroup::*;
    ///
    /// let request = PaygroupListRequest {
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     status: Some("active".to_string()),
    /// };
    ///
    /// let response = service.list_paygroups(&request).await?;
    /// println!("找到 {} 个薪资组", response.page.items.len());
    /// ```
    pub async fn list_paygroups(
        &self,
        request: &PaygroupListRequest,
    ) -> SDKResult<BaseResponse<PaygroupListResponse>> {
        let mut query_params = std::collections::HashMap::new();

        if let Some(page_size) = &request.page_size {
            query_params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = &request.page_token {
            query_params.insert("page_token".to_string(), page_token.clone());
        }
        if let Some(status) = &request.status {
            query_params.insert("status".to_string(), status.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
             api_path: (""/open-apis/payroll/v4/paygroups".to_string()"),
            query_params,
            path_params: std::collections::HashMap::new(),
            
            body: Vec::new(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 获取薪资组详情
    ///
    /// # API文档
    ///
    /// 根据薪资组ID获取详细信息。
    ///
    /// # 参数
    ///
    /// * `paygroup_id` - 薪资组ID
    ///
    /// # 返回值
    ///
    /// 返回薪资组的详细信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::paygroup::*;
    ///
    /// let response = service.get_paygroup("pg_123456").await?;
    /// println!("薪资组名称: {:?}", response.data.paygroup_name.zh_cn);
    /// ```
    pub async fn get_paygroup(
        &self,
        paygroup_id: &str,
    ) -> SDKResult<BaseResponse<PaygroupResponse>> {
        let mut path_params = std::collections::HashMap::new();
        path_params.insert("paygroup_id".to_string(), paygroup_id.to_string());

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            url: self
                .config
                .build_url(""/open-apis/payroll/v4/paygroups".to_string()/{paygroup_id}"),
            query_params: std::collections::HashMap::new(),
            path_params,
            
            body: Vec::new(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 创建薪资组
    ///
    /// # API文档
    ///
    /// 创建新的薪资组，配置发薪规则和周期。
    ///
    /// # 参数
    ///
    /// * `request` - 创建薪资组请求参数
    ///
    /// # 返回值
    ///
    /// 返回创建的薪资组信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::paygroup::*;
    ///
    /// let request = CreatePaygroupRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("技术部薪资组".to_string()),
    ///         en_us: Some("Tech Department Payroll".to_string()),
    ///         ja_jp: None,
    ///     },
    ///     paygroup_type: "standard".to_string(),
    ///     payment_cycle_type: "monthly".to_string(),
    ///     payment_day_setting: Some(PaymentDaySetting {
    ///         payment_day_type: "fixed".to_string(),
    ///         payment_day: Some(15),
    ///         holiday_adjustment: true,
    ///         adjustment_rule: Some("advance".to_string()),
    ///     }),
    ///     description: Some(I18nText {
    ///         zh_cn: Some("技术部门专用薪资组".to_string()),
    ///         en_us: None,
    ///         ja_jp: None,
    ///     }),
    /// };
    ///
    /// let response = service.create_paygroup(&request).await?;
    /// println!("薪资组创建成功，ID: {}", response.data.paygroup_id);
    /// ```
    pub async fn create_paygroup(
        &self,
        request: &CreatePaygroupRequest,
    ) -> SDKResult<BaseResponse<PaygroupResponse>> {
        let api_req = ApiRequest {
            method: Method::POST,
             api_path: (""/open-apis/payroll/v4/paygroups".to_string()"),
            query_params: std::collections::HashMap::new(),
            path_params: std::collections::HashMap::new(),
            
            body: Some(serde_json::to_value(request)?),
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 更新薪资组
    ///
    /// # API文档
    ///
    /// 更新现有薪资组的配置信息。
    ///
    /// # 参数
    ///
    /// * `request` - 更新薪资组请求参数
    ///
    /// # 返回值
    ///
    /// 返回更新后的薪资组信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::paygroup::*;
    ///
    /// let request = UpdatePaygroupRequest {
    ///     paygroup_id: "pg_123456".to_string(),
    ///     name: Some(I18nText {
    ///         zh_cn: Some("技术部薪资组v2".to_string()),
    ///         en_us: Some("Tech Department Payroll v2".to_string()),
    ///         ja_jp: None,
    ///     }),
    ///     status: Some("active".to_string()),
    ///     payment_day_setting: Some(PaymentDaySetting {
    ///         payment_day_type: "last_day".to_string(),
    ///         payment_day: Some(31),
    ///         holiday_adjustment: false,
    ///         adjustment_rule: Some("no_adjustment".to_string()),
    ///     }),
    ///     description: Some(I18nText {
    ///         zh_cn: Some("更新后的技术部门薪资组".to_string()),
    ///         en_us: None,
    ///         ja_jp: None,
    ///     }),
    /// };
    ///
    /// let response = service.update_paygroup(&request).await?;
    /// println!("薪资组更新成功");
    /// ```
    pub async fn update_paygroup(
        &self,
        request: &UpdatePaygroupRequest,
    ) -> SDKResult<BaseResponse<PaygroupResponse>> {
        let mut path_params = std::collections::HashMap::new();
        path_params.insert("paygroup_id".to_string(), request.paygroup_id.clone());

        let api_req = ApiRequest {
            method: Method::PUT,
            url: self
                .config
                .build_url(""/open-apis/payroll/v4/paygroups".to_string()/{paygroup_id}"),
            query_params: std::collections::HashMap::new(),
            path_params,
            
            body: Some(serde_json::to_value(request)?),
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 删除薪资组
    ///
    /// # API文档
    ///
    /// 删除指定的薪资组，删除前请确保没有关联的员工和发薪活动。
    ///
    /// # 参数
    ///
    /// * `paygroup_id` - 薪资组ID
    ///
    /// # 返回值
    ///
    /// 返回删除操作的执行结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::paygroup::*;
    ///
    /// let response = service.delete_paygroup("pg_123456").await?;
    /// if response.data.success {
    ///     println!("薪资组删除成功");
    /// }
    /// ```
    pub async fn delete_paygroup(
        &self,
        paygroup_id: &str,
    ) -> SDKResult<BaseResponse<DeleteResponse>> {
        let mut path_params = std::collections::HashMap::new();
        path_params.insert("paygroup_id".to_string(), paygroup_id.to_string());

        let api_req = ApiRequest {
            method: Method::DELETE,
            url: self
                .config
                .build_url(""/open-apis/payroll/v4/paygroups".to_string()/{paygroup_id}"),
            query_params: std::collections::HashMap::new(),
            path_params,
            
            body: Vec::new(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 激活薪资组
    ///
    /// # API文档
    ///
    /// 激活指定的薪资组，使其可以用于发薪活动。
    ///
    /// # 参数
    ///
    /// * `paygroup_id` - 薪资组ID
    ///
    /// # 返回值
    ///
    /// 返回激活操作的执行结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::paygroup::*;
    ///
    /// let response = service.activate_paygroup("pg_123456").await?;
    /// if response.data.success {
    ///     println!("薪资组激活成功");
    /// }
    /// ```
    pub async fn activate_paygroup(
        &self,
        paygroup_id: &str,
    ) -> SDKResult<BaseResponse<OperationResponse>> {
        let mut path_params = std::collections::HashMap::new();
        path_params.insert("paygroup_id".to_string(), paygroup_id.to_string());

        let api_req = ApiRequest {
            method: Method::POST,
            url: self
                .config
                .build_url(""/open-apis/payroll/v4/paygroups".to_string()/{paygroup_id}/activate"),
            query_params: std::collections::HashMap::new(),
            path_params,
            
            body: Some(serde_json::Value::Object(serde_json::Map::new())),
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 停用薪资组
    ///
    /// # API文档
    ///
    /// 停用指定的薪资组，停用后不能用于新的发薪活动。
    ///
    /// # 参数
    ///
    /// * `paygroup_id` - 薪资组ID
    ///
    /// # 返回值
    ///
    /// 返回停用操作的执行结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::paygroup::*;
    ///
    /// let response = service.deactivate_paygroup("pg_123456").await?;
    /// if response.data.success {
    ///     println!("薪资组停用成功");
    /// }
    /// ```
    pub async fn deactivate_paygroup(
        &self,
        paygroup_id: &str,
    ) -> SDKResult<BaseResponse<OperationResponse>> {
        let mut path_params = std::collections::HashMap::new();
        path_params.insert("paygroup_id".to_string(), paygroup_id.to_string());

        let api_req = ApiRequest {
            method: Method::POST,
            url: self
                .config
                .build_url(""/open-apis/payroll/v4/paygroups".to_string()/{paygroup_id}/deactivate"),
            query_params: std::collections::HashMap::new(),
            path_params,
            
            body: Some(serde_json::Value::Object(serde_json::Map::new())),
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }
}

/// 删除操作响应
#[derive(Debug, Clone)]
pub struct DeleteResponse {
    /// 操作结果
    pub success: bool,
    /// 消息
    pub message: String,
}

/// 操作响应
#[derive(Debug, Clone)]
pub struct OperationResponse {
    /// 操作结果
    pub success: bool,
    /// 消息
    pub message: String,
}
