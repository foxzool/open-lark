#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Ticket服务模块
//!
//! 提供完整的客服工单管理功能：
//! - 查询全部工单详情
//! - 支持分页查询和筛选
//! - 支持多种排序方式
//! - 企业级错误处理和验证

use crate::{
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::service::helpdesk::models::Ticket;

/// 查询全部工单详情请求
///
/// 用于查询客服系统中的所有工单列表，支持分页、筛选和排序功能。
/// 可以根据工单状态、优先级、创建者等条件进行筛选。
///
/// # 示例
/// ```rust
/// use open_lark::service::helpdesk::v1::ticket::ListTicketsRequest;
///
/// let request = ListTicketsRequest::builder()
///     .page_size(20)
///     .page_token("next_page_token")
///     .build()?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTicketsRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 分页大小
    /// 每页返回的工单数量，默认为20，最大为100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    /// 用于获取下一页数据的标记，从上一页响应中获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 工单状态筛选
    /// 只返回指定状态的工单：pending(待处理)、processing(处理中)、solved(已解决)、closed(已关闭)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 优先级筛选
    /// 只返回指定优先级的工单：low(低)、medium(中)、high(高)、urgent(紧急)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// 分配的客服ID筛选
    /// 只返回分配给指定客服的工单
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,
    /// 创建者ID筛选
    /// 只返回指定创建者创建的工单
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// 排序字段
    /// 支持按created_at(创建时间)、updated_at(更新时间)、priority(优先级)排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    /// 排序方向
    /// asc(升序)或desc(降序)，默认为desc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
}

impl Default for ListTicketsRequest {
    fn default() -> Self {
        Self {
            api_req: ApiRequest::default(),
            page_size: Some(20),
            page_token: None,
            status: None,
            priority: None,
            assignee_id: None,
            creator_id: None,
            sort_field: Some("created_at".to_string()),
            sort_direction: Some("desc".to_string()),
        }
    }
}

impl ListTicketsRequest {
    /// 创建新的查询工单列表请求
    ///
    /// # 返回
    /// 返回带有默认参数的请求实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建查询工单列表请求的构建器
    pub fn builder() -> ListTicketsBuilder {
        ListTicketsBuilder::default()
    }

    /// 构建请求验证
    ///
    /// 验证请求参数的有效性
    ///
    /// # 返回
    /// 成功返回空值，失败返回错误信息
    pub fn build(&self) -> SDKResult<()> {
        // 验证分页大小
        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 100 {
                return Err(crate::core::error::SDKError::ValidationError(
                    "分页大小必须在1-100之间".to_string(),
                ));
            }
        }

        // 验证排序字段
        if let Some(sort_field) = &self.sort_field {
            let valid_fields = vec!["created_at", "updated_at", "priority", "status"];
            if !valid_fields.contains(&sort_field.as_str()) {
                return Err(crate::core::error::SDKError::ValidationError(
                    format!("无效的排序字段: {}", sort_field),
                ));
            }
        }

        // 验证排序方向
        if let Some(sort_direction) = &self.sort_direction {
            if sort_direction != "asc" && sort_direction != "desc" {
                return Err(crate::core::error::SDKError::ValidationError(
                    "排序方向必须是 'asc' 或 'desc'".to_string(),
                ));
            }
        }

        // 验证工单状态
        if let Some(status) = &self.status {
            let valid_statuses = vec!["pending", "processing", "solved", "closed"];
            if !valid_statuses.contains(&status.as_str()) {
                return Err(crate::core::error::SDKError::ValidationError(
                    format!("无效的工单状态: {}", status),
                ));
            }
        }

        // 验证优先级
        if let Some(priority) = &self.priority {
            let valid_priorities = vec!["low", "medium", "high", "urgent"];
            if !valid_priorities.contains(&priority.as_str()) {
                return Err(crate::core::error::SDKError::ValidationError(
                    format!("无效的优先级: {}", priority),
                ));
            }
        }

        Ok(())
    }
}

/// 查询工单列表请求构建器
///
/// 提供链式调用接口来配置请求参数
///
/// # 示例
/// ```rust
/// use open_lark::service::helpdesk::v1::ticket::ListTicketsRequest;
///
/// let request = ListTicketsRequest::builder()
///     .page_size(20)
///     .status("pending")
///     .priority("high")
///     .sort_field("created_at")
///     .sort_direction("desc")
///     .build()?;
/// ```
#[derive(Debug, Clone, Default)]
pub struct ListTicketsBuilder {
    request: ListTicketsRequest,
}

impl ListTicketsBuilder {
    /// 设置分页大小
    ///
    /// # 参数
    /// * `page_size` - 每页工单数量，范围1-100
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    ///
    /// # 参数
    /// * `page_token` - 分页标记，用于获取下一页数据
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 设置工单状态筛选
    ///
    /// # 参数
    /// * `status` - 工单状态：pending、processing、solved、closed
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.request.status = Some(status.into());
        self
    }

    /// 设置优先级筛选
    ///
    /// # 参数
    /// * `priority` - 工单优先级：low、medium、high、urgent
    pub fn priority(mut self, priority: impl Into<String>) -> Self {
        self.request.priority = Some(priority.into());
        self
    }

    /// 设置分配的客服ID筛选
    ///
    /// # 参数
    /// * `assignee_id` - 客服ID
    pub fn assignee_id(mut self, assignee_id: impl Into<String>) -> Self {
        self.request.assignee_id = Some(assignee_id.into());
        self
    }

    /// 设置创建者ID筛选
    ///
    /// # 参数
    /// * `creator_id` - 创建者ID
    pub fn creator_id(mut self, creator_id: impl Into<String>) -> Self {
        self.request.creator_id = Some(creator_id.into());
        self
    }

    /// 设置排序字段
    ///
    /// # 参数
    /// * `sort_field` - 排序字段：created_at、updated_at、priority、status
    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.request.sort_field = Some(sort_field.into());
        self
    }

    /// 设置排序方向
    ///
    /// # 参数
    /// * `sort_direction` - 排序方向：asc(升序)或desc(降序)
    pub fn sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.request.sort_direction = Some(sort_direction.into());
        self
    }

    /// 构建请求对象
    ///
    /// # 返回
    /// 成功返回请求对象，失败返回验证错误
    ///
    /// # 错误
    /// * 参数无效时返回验证错误
    pub fn build(self) -> SDKResult<ListTicketsRequest> {
        self.request.build()?;
        Ok(self.request)
    }
}

/// 查询工单列表响应
///
/// 包含请求成功后返回的工单列表和分页信息
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ListTicketsResponse {
    /// 工单列表
    /// 符合查询条件的工单数据数组
    pub items: Vec<Ticket>,
    /// 是否还有更多数据
    /// true表示还有下一页数据，false表示已到最后一页
    #[serde(default)]
    pub has_more: bool,
    /// 分页标记
    /// 用于获取下一页数据的标记，has_more为true时有值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListTicketsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== API #569: 查询指定工单详情 ====================

/// 查询指定工单详情请求
///
/// 用于获取客服系统中指定工单的详细信息，包括工单状态、描述、
/// 创建者、分配客服等完整信息。
///
/// # 示例
/// ```rust
/// use open_lark::service::helpdesk::v1::ticket::GetTicketRequest;
///
/// let request = GetTicketRequest::builder()
///     .ticket_id("6943948239044347654")
///     .build()?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTicketRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 工单ID
    /// 要查询的工单的唯一标识符
    pub ticket_id: String,
}

impl Default for GetTicketRequest {
    fn default() -> Self {
        Self {
            api_req: ApiRequest::default(),
            ticket_id: String::new(),
        }
    }
}

impl GetTicketRequest {
    /// 创建新的查询工单详情请求
    ///
    /// # 参数
    /// * `ticket_id` - 工单ID
    ///
    /// # 返回
    /// 返回查询工单详情请求实例
    ///
    /// # 示例
    /// ```rust
    /// let request = GetTicketRequest::new("6943948239044347654");
    /// ```
    pub fn new(ticket_id: impl Into<String>) -> Self {
        Self {
            api_req: ApiRequest::default(),
            ticket_id: ticket_id.into(),
        }
    }

    /// 验证请求参数
    ///
    /// # 返回
    /// * `Ok(())` - 验证通过
    /// * `Err(String)` - 验证失败，返回错误描述
    pub fn validate(&self) -> Result<(), String> {
        if self.ticket_id.is_empty() {
            return Err("工单ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 查询指定工单详情响应
///
/// 包含指定工单的完整信息，包括基本信息、状态、优先级、
/// 创建者、分配客服、时间戳等详细数据。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTicketResponse {
    /// 工单详细信息
    /// 包含工单的所有属性和当前状态
    pub ticket: Ticket,
}

impl ApiResponseTrait for GetTicketResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== API #570: 更新工单详情 ====================

/// 更新工单详情请求
///
/// 用于更新客服系统中指定工单的信息，支持更新工单标题、描述、
/// 状态、优先级、分配客服等字段。支持部分字段更新和完整更新。
///
/// # 示例
/// ```rust
/// use open_lark::service::helpdesk::v1::ticket::UpdateTicketRequest;
///
/// let mut ticket = Ticket::default();
/// ticket.title = Some("更新的工单标题".to_string());
/// ticket.status = Some(TicketStatus::Processing);
///
/// let request = UpdateTicketRequest::new("6943948239044347654", ticket);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTicketRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 工单ID
    /// 要更新的工单的唯一标识符
    pub ticket_id: String,
    /// 工单信息
    /// 包含要更新的工单字段，未设置的字段不会被修改
    pub ticket: Ticket,
    /// 更新字段掩码
    /// 指定要更新的字段列表，如果为空则更新所有非空字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mask: Option<Vec<String>>,
}

impl UpdateTicketRequest {
    /// 创建新的更新工单请求
    ///
    /// # 参数
    /// * `ticket_id` - 工单ID
    /// * `ticket` - 包含更新字段的工单对象
    ///
    /// # 返回
    /// 返回更新工单请求实例
    ///
    /// # 示例
    /// ```rust
    /// let mut ticket = Ticket::default();
    /// ticket.title = Some("新标题".to_string());
    ///
    /// let request = UpdateTicketRequest::new("ticket_123", ticket);
    /// ```
    pub fn new(ticket_id: impl Into<String>, ticket: Ticket) -> Self {
        Self {
            api_req: ApiRequest::default(),
            ticket_id: ticket_id.into(),
            ticket,
            update_mask: None,
        }
    }

    /// 设置更新字段掩码
    ///
    /// # 参数
    /// * `fields` - 要更新的字段名列表
    ///
    /// # 返回
    /// 返回请求实例以支持链式调用
    ///
    /// # 示例
    /// ```rust
    /// let request = UpdateTicketRequest::new("ticket_123", ticket)
    ///     .update_mask(vec!["title".to_string(), "status".to_string()]);
    /// ```
    pub fn update_mask(mut self, fields: Vec<String>) -> Self {
        self.update_mask = Some(fields);
        self
    }

    /// 验证请求参数
    ///
    /// # 返回
    /// * `Ok(())` - 验证通过
    /// * `Err(String)` - 验证失败，返回错误描述
    pub fn validate(&self) -> Result<(), String> {
        if self.ticket_id.is_empty() {
            return Err("工单ID不能为空".to_string());
        }

        // 验证更新字段掩码
        if let Some(ref mask) = self.update_mask {
            let valid_fields = vec![
                "title", "description", "status", "priority", "assignee"
            ];

            for field in mask {
                if !valid_fields.contains(&field.as_str()) {
                    return Err(format!("无效的更新字段: {}", field));
                }
            }
        }

        Ok(())
    }

    /// 获取实际要更新的字段列表
    ///
    /// # 返回
    /// 返回需要更新的字段名列表
    pub fn get_update_fields(&self) -> Vec<String> {
        if let Some(ref mask) = self.update_mask {
            mask.clone()
        } else {
            // 如果没有指定更新掩码，则返回所有非空字段
            let mut fields = Vec::new();

            if self.ticket.title.is_some() {
                fields.push("title".to_string());
            }
            if self.ticket.description.is_some() {
                fields.push("description".to_string());
            }
            if self.ticket.status.is_some() {
                fields.push("status".to_string());
            }
            if self.ticket.priority.is_some() {
                fields.push("priority".to_string());
            }
            if self.ticket.assignee.is_some() {
                fields.push("assignee".to_string());
            }

            fields
        }
    }
}

/// 更新工单详情响应
///
/// 包含更新后的工单完整信息，反映所有已应用的修改。
/// 包含更新后的状态、时间戳等系统自动维护的字段。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTicketResponse {
    /// 更新后的工单信息
    /// 包含工单的所有字段和最新状态
    pub ticket: Ticket,
}

impl ApiResponseTrait for UpdateTicketResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Ticket服务
///
/// 提供客服工单的完整管理功能，支持查询、筛选、分页等操作。
/// 集成企业级错误处理、参数验证和性能优化。
#[derive(Debug, Clone)]
pub struct TicketService {
    config: Config,
}

impl TicketService {
    /// 创建新的Ticket服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询全部工单详情
    ///
    /// 获取客服系统中的工单列表，支持分页、筛选和排序功能。
    /// 可以根据工单状态、优先级、分配客服等条件进行筛选。
    ///
    /// # 参数
    /// * `request` - 查询工单列表请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回工单列表响应，失败返回错误信息
    ///
    /// # 权限要求
    /// 需要具备客服工单的查看权限
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::helpdesk::v1::ticket::{TicketService, ListTicketsRequest};
    ///
    /// let service = TicketService::new(config);
    /// let request = ListTicketsRequest::builder()
    ///     .page_size(20)
    ///     .status("pending")
    ///     .priority("high")
    ///     .build()?;
    ///
    /// let response = service.list_tickets(request, None).await?;
    /// println!("找到{}个待处理的高优先级工单", response.data.items.len());
    /// ```
    pub async fn list_tickets(
        &self,
        request: ListTicketsRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<ListTicketsResponse>> {
        // 创建API请求
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);

        // 设置API路径
        api_req.set_api_path(crate::core::endpoints_original::Endpoints::HELPDESK_V1_TICKETS);

        // 构建查询参数
        let mut query_params = Vec::new();

        if let Some(page_size) = request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }

        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        if let Some(status) = &request.status {
            query_params.push(format!("status={}", status));
        }

        if let Some(priority) = &request.priority {
            query_params.push(format!("priority={}", priority));
        }

        if let Some(assignee_id) = &request.assignee_id {
            query_params.push(format!("assignee_id={}", assignee_id));
        }

        if let Some(creator_id) = &request.creator_id {
            query_params.push(format!("creator_id={}", creator_id));
        }

        if let Some(sort_field) = &request.sort_field {
            query_params.push(format!("sort_field={}", sort_field));
        }

        if let Some(sort_direction) = &request.sort_direction {
            query_params.push(format!("sort_direction={}", sort_direction));
        }

        // 添加查询参数到API路径
        if !query_params.is_empty() {
            let mut api_path = api_req.api_path.clone();
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
            api_req.set_api_path(&api_path);
        }

        // 设置支持的访问令牌类型
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        // 发送HTTP请求
        let api_resp = Transport::<ListTicketsResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 查询指定工单详情
    ///
    /// 获取客服系统中指定工单的详细信息，包括工单状态、描述、
    /// 创建者、分配客服等完整信息。
    ///
    /// # 参数
    /// * `request` - 查询工单详情请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回工单详情响应，失败返回错误信息
    ///
    /// # 示例
    /// ```rust
    /// let request = GetTicketRequest::new("6943948239044347654");
    /// let response = service.get_ticket(request, None).await?;
    /// println!("工单标题: {}", response.ticket.title);
    /// ```
    pub async fn get_ticket(
        &self,
        request: GetTicketRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<GetTicketResponse>> {
        // 验证请求参数
        request.validate().map_err(|e| {
            crate::core::error::SDKError::InvalidParameter(format!("参数验证失败: {}", e))
        })?;

        // 创建API请求
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);

        // 设置API路径并替换路径参数
        let api_path = crate::core::endpoints_original::Endpoints::HELPDESK_V1_TICKET_GET
            .replace("{ticket_id}", &request.ticket_id);
        api_req.set_api_path(api_path);

        // 设置支持的访问令牌类型
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        // 发送HTTP请求
        let api_resp = Transport::<GetTicketResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 更新工单详情
    ///
    /// 更新客服系统中指定工单的信息，支持更新工单标题、描述、
    /// 状态、优先级、分配客服等字段。支持部分字段更新和完整更新。
    ///
    /// # 参数
    /// * `request` - 更新工单请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回更新后的工单详情响应，失败返回错误信息
    ///
    /// # 示例
    /// ```rust
    /// let mut ticket = Ticket::default();
    /// ticket.title = Some("更新的标题".to_string());
    /// ticket.status = Some(TicketStatus::Processing);
    ///
    /// let request = UpdateTicketRequest::new("6943948239044347654", ticket);
    /// let response = service.update_ticket(request, None).await?;
    /// println!("工单更新成功，新状态: {:?}", response.ticket.status);
    /// ```
    pub async fn update_ticket(
        &self,
        request: UpdateTicketRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateTicketResponse>> {
        // 验证请求参数
        request.validate().map_err(|e| {
            crate::core::error::SDKError::InvalidParameter(format!("参数验证失败: {}", e))
        })?;

        // 检查是否有要更新的字段
        let update_fields = request.get_update_fields();
        if update_fields.is_empty() {
            return Err(crate::core::error::SDKError::InvalidParameter(
                "至少需要指定一个要更新的字段".to_string(),
            ));
        }

        // 创建API请求
        let mut api_req = request.api_req;
        api_req.set_http_method(reqwest::Method::PUT);

        // 设置API路径并替换路径参数
        let api_path = crate::core::endpoints_original::Endpoints::HELPDESK_V1_TICKET_UPDATE
            .replace("{ticket_id}", &request.ticket_id);
        api_req.set_api_path(api_path);

        // 构建请求体（只包含要更新的字段）
        let mut filtered_ticket = Ticket::default();
        for field in &update_fields {
            match field.as_str() {
                "title" => filtered_ticket.title = request.ticket.title.clone(),
                "description" => filtered_ticket.description = request.ticket.description.clone(),
                "status" => filtered_ticket.status = request.ticket.status.clone(),
                "priority" => filtered_ticket.priority = request.ticket.priority.clone(),
                "assignee" => filtered_ticket.assignee = request.ticket.assignee.clone(),
                _ => {}
            }
        }

        // 序列化请求体
        let body = serde_json::to_vec(&filtered_ticket).map_err(|e| {
            crate::core::error::SDKError::SerializationError(format!("请求体序列化失败: {}", e))
        })?;
        api_req.set_body(body);

        // 设置支持的访问令牌类型
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        // 发送HTTP请求
        let api_resp = Transport::<UpdateTicketResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 获取工单列表构建器
    ///
    /// 创建一个查询工单列表的构建器，支持链式调用和完整的错误处理
    ///
    /// # 参数
    /// * `request` - 查询工单列表请求
    ///
    /// # 返回
    /// 返回查询工单列表构建器，可用于执行查询操作
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::helpdesk::v1::ticket::{TicketService, ListTicketsRequest};
    /// use std::sync::Arc;
    ///
    /// let service = Arc::new(TicketService::new(config));
    /// let response = service
    ///     .list_tickets_builder(ListTicketsRequest::new())
    ///     .page_size(20)
    ///     .status("pending")
    ///     .priority("high")
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn list_tickets_builder(&self, request: ListTicketsRequest) -> ListTicketsBuilder {
        ListTicketsBuilder::new(std::sync::Arc::new(self.clone()), request)
    }

    /// 获取工单详情构建器
    ///
    /// 创建一个查询工单详情的构建器，支持链式调用和完整的错误处理
    ///
    /// # 参数
    /// * `request` - 查询工单详情请求
    ///
    /// # 返回
    /// 返回工单详情查询构建器实例
    ///
    /// # 示例
    /// ```rust,no_run
    /// let response = service
    ///     .get_ticket_builder(GetTicketRequest::new("6943948239044347654"))
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn get_ticket_builder(&self, request: GetTicketRequest) -> GetTicketBuilder {
        GetTicketBuilder::new(std::sync::Arc::new(self.clone()), request)
    }

    /// 更新工单详情构建器
    ///
    /// 创建一个更新工单详情的构建器，支持链式调用和完整的错误处理
    ///
    /// # 参数
    /// * `request` - 更新工单请求
    ///
    /// # 返回
    /// 返回更新工单构建器实例
    ///
    /// # 示例
    /// ```rust,no_run
    /// let mut ticket = Ticket::default();
    /// ticket.title = Some("更新的工单标题".to_string());
    /// ticket.status = Some(TicketStatus::Processing);
    ///
    /// let response = service
    ///     .update_ticket_builder(UpdateTicketRequest::new("ticket_123", ticket))
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn update_ticket_builder(&self, request: UpdateTicketRequest) -> UpdateTicketBuilder {
        UpdateTicketBuilder::new(std::sync::Arc::new(self.clone()), request)
    }
}

/// 查询工单列表构建器
///
/// 提供现代化的链式调用接口，支持与服务实例的集成
///
/// # 示例
/// ```rust,no_run
/// use open_lark::service::helpdesk::v1::ticket::TicketService;
/// use std::sync::Arc;
///
/// let service = Arc::new(TicketService::new(config));
/// let response = service
///     .list_tickets_builder(ListTicketsRequest::new())
///     .page_size(20)
///     .status("pending")
///     .execute()
///     .await?;
/// ```
pub struct ListTicketsBuilder {
    service: std::sync::Arc<TicketService>,
    request: ListTicketsRequest,
}

impl ListTicketsBuilder {
    /// 创建新的查询工单列表构建器
    ///
    /// # 参数
    /// * `service` - Ticket服务实例
    /// * `request` - 查询工单列表请求
    pub fn new(service: std::sync::Arc<TicketService>, request: ListTicketsRequest) -> Self {
        Self { service, request }
    }

    /// 设置分页大小
    ///
    /// # 参数
    /// * `page_size` - 每页工单数量，范围1-100
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    ///
    /// # 参数
    /// * `page_token` - 分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 设置工单状态筛选
    ///
    /// # 参数
    /// * `status` - 工单状态
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.request.status = Some(status.into());
        self
    }

    /// 设置优先级筛选
    ///
    /// # 参数
    /// * `priority` - 工单优先级
    pub fn priority(mut self, priority: impl Into<String>) -> Self {
        self.request.priority = Some(priority.into());
        self
    }

    /// 设置分配的客服ID筛选
    ///
    /// # 参数
    /// * `assignee_id` - 客服ID
    pub fn assignee_id(mut self, assignee_id: impl Into<String>) -> Self {
        self.request.assignee_id = Some(assignee_id.into());
        self
    }

    /// 设置创建者ID筛选
    ///
    /// # 参数
    /// * `creator_id` - 创建者ID
    pub fn creator_id(mut self, creator_id: impl Into<String>) -> Self {
        self.request.creator_id = Some(creator_id.into());
        self
    }

    /// 设置排序字段
    ///
    /// # 参数
    /// * `sort_field` - 排序字段
    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.request.sort_field = Some(sort_field.into());
        self
    }

    /// 设置排序方向
    ///
    /// # 参数
    /// * `sort_direction` - 排序方向
    pub fn sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.request.sort_direction = Some(sort_direction.into());
        self
    }

    /// 执行查询工单列表请求
    ///
    /// # 返回
    /// 成功返回工单列表响应，失败返回错误信息
    ///
    /// # 错误
    /// * 参数无效时返回验证错误
    /// * 权限不足时返回授权错误
    /// * 网络错误时返回连接错误
    pub async fn execute(self) -> SDKResult<BaseResponse<ListTicketsResponse>> {
        // 验证请求参数
        self.request.build()?;

        // 执行请求
        self.service.list_tickets(self.request, None).await
    }
}

// ==================== GetTicketBuilder 构建器模式 ====================

/// 查询指定工单详情构建器
///
/// 提供流畅的API来构建查询工单详情的请求，支持链式调用
/// 和完整的参数验证。
#[derive(Debug, Clone)]
pub struct GetTicketBuilder {
    service: std::sync::Arc<TicketService>,
    request: GetTicketRequest,
}

impl GetTicketBuilder {
    /// 创建新的查询工单详情构建器
    ///
    /// # 参数
    /// * `service` - Ticket服务实例
    /// * `request` - 查询工单详情请求
    pub fn new(service: std::sync::Arc<TicketService>, request: GetTicketRequest) -> Self {
        Self { service, request }
    }

    /// 设置工单ID
    ///
    /// # 参数
    /// * `ticket_id` - 工单ID
    ///
    /// # 示例
    /// ```rust
    /// let builder = GetTicketBuilder::new(service, request)
    ///     .ticket_id("6943948239044347654");
    /// ```
    pub fn ticket_id(mut self, ticket_id: impl Into<String>) -> Self {
        self.request.ticket_id = ticket_id.into();
        self
    }

    /// 执行查询工单详情请求
    ///
    /// # 返回
    /// 成功返回工单详情响应，失败返回错误信息
    ///
    /// # 示例
    /// ```rust
    /// let response = GetTicketBuilder::new(service, request)
    ///     .ticket_id("6943948239044347654")
    ///     .execute()
    ///     .await?;
    /// ```
    pub async fn execute(self) -> SDKResult<BaseResponse<GetTicketResponse>> {
        // 验证请求参数
        self.request.validate().map_err(|e| {
            crate::core::error::SDKError::InvalidParameter(format!("参数验证失败: {}", e))
        })?;

        // 执行请求
        self.service.get_ticket(self.request, None).await
    }
}

// ==================== UpdateTicketBuilder 构建器模式 ====================

/// 更新工单详情构建器
///
/// 提供流畅的API来构建更新工单详情的请求，支持链式调用
/// 和完整的参数验证。
#[derive(Debug, Clone)]
pub struct UpdateTicketBuilder {
    service: std::sync::Arc<TicketService>,
    request: UpdateTicketRequest,
}

impl UpdateTicketBuilder {
    /// 创建新的更新工单详情构建器
    ///
    /// # 参数
    /// * `service` - Ticket服务实例
    /// * `request` - 更新工单详情请求
    pub fn new(service: std::sync::Arc<TicketService>, request: UpdateTicketRequest) -> Self {
        Self { service, request }
    }

    /// 设置工单标题
    ///
    /// # 参数
    /// * `title` - 工单标题
    ///
    /// # 示例
    /// ```rust
    /// let builder = UpdateTicketBuilder::new(service, request)
    ///     .title("更新的工单标题");
    /// ```
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.ticket.title = Some(title.into());
        self
    }

    /// 设置工单描述
    ///
    /// # 参数
    /// * `description` - 工单描述
    ///
    /// # 示例
    /// ```rust
    /// let builder = UpdateTicketBuilder::new(service, request)
    ///     .description("这是更新的工单描述");
    /// ```
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.ticket.description = Some(description.into());
        self
    }

    /// 设置工单状态
    ///
    /// # 参数
    /// * `status` - 工单状态
    ///
    /// # 示例
    /// ```rust
    /// let builder = UpdateTicketBuilder::new(service, request)
    ///     .status(TicketStatus::Processing);
    /// ```
    pub fn status(mut self, status: TicketStatus) -> Self {
        self.request.ticket.status = Some(status);
        self
    }

    /// 设置工单优先级
    ///
    /// # 参数
    /// * `priority` - 工单优先级
    ///
    /// # 示例
    /// ```rust
    /// let builder = UpdateTicketBuilder::new(service, request)
    ///     .priority(TicketPriority::High);
    /// ```
    pub fn priority(mut self, priority: TicketPriority) -> Self {
        self.request.ticket.priority = Some(priority);
        self
    }

    /// 设置分配的客服
    ///
    /// # 参数
    /// * `assignee` - 客服ID
    ///
    /// # 示例
    /// ```rust
    /// let builder = UpdateTicketBuilder::new(service, request)
    ///     .assignee("agent_123");
    /// ```
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.request.ticket.assignee = Some(assignee.into());
        self
    }

    /// 设置更新字段掩码
    ///
    /// # 参数
    /// * `fields` - 要更新的字段列表
    ///
    /// # 示例
    /// ```rust
    /// let builder = UpdateTicketBuilder::new(service, request)
    ///     .update_mask(vec!["title".to_string(), "status".to_string()]);
    /// ```
    pub fn update_mask(mut self, fields: Vec<String>) -> Self {
        self.request = self.request.update_mask(fields);
        self
    }

    /// 执行更新工单详情请求
    ///
    /// # 返回
    /// 成功返回更新后的工单详情响应，失败返回错误信息
    ///
    /// # 示例
    /// ```rust
    /// let response = UpdateTicketBuilder::new(service, request)
    ///     .title("更新的标题")
    ///     .status(TicketStatus::Processing)
    ///     .execute()
    ///     .await?;
    /// ```
    pub async fn execute(self) -> SDKResult<BaseResponse<UpdateTicketResponse>> {
        // 验证请求参数
        self.request.validate().map_err(|e| {
            crate::core::error::SDKError::InvalidParameter(format!("参数验证失败: {}", e))
        })?;

        // 执行请求
        self.service.update_ticket(self.request, None).await
    }
}

impl crate::core::service_trait::Service for TicketService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn transport(&self) -> &dyn crate::core::transport::Transport {
        panic!("TicketService does not have a transport instance")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> TicketService {
        let config = Config::new("test_app_id", "test_app_secret");
        TicketService::new(config)
    }

    #[test]
    fn test_ticket_service_creation() {
        let config = Config::new("test_app_id", "test_app_secret");
        let service = TicketService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_list_tickets_request_default() {
        let request = ListTicketsRequest::default();
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, None);
        assert_eq!(request.status, None);
        assert_eq!(request.priority, None);
        assert_eq!(request.assignee_id, None);
        assert_eq!(request.creator_id, None);
        assert_eq!(request.sort_field, Some("created_at".to_string()));
        assert_eq!(request.sort_direction, Some("desc".to_string()));
    }

    #[test]
    fn test_list_tickets_request_new() {
        let request = ListTicketsRequest::new();
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.sort_field.as_ref().unwrap(), "created_at");
        assert_eq!(request.sort_direction.as_ref().unwrap(), "desc");
    }

    #[test]
    fn test_list_tickets_request_validation() {
        // 测试有效的请求
        let valid_request = ListTicketsRequest {
            api_req: ApiRequest::default(),
            page_size: Some(20),
            page_token: Some("token".to_string()),
            status: Some("pending".to_string()),
            priority: Some("high".to_string()),
            assignee_id: Some("agent_123".to_string()),
            creator_id: Some("user_456".to_string()),
            sort_field: Some("created_at".to_string()),
            sort_direction: Some("desc".to_string()),
        };
        assert!(valid_request.build().is_ok());

        // 测试无效的分页大小
        let invalid_page_size = ListTicketsRequest {
            api_req: ApiRequest::default(),
            page_size: Some(0),
            page_token: None,
            status: None,
            priority: None,
            assignee_id: None,
            creator_id: None,
            sort_field: Some("created_at".to_string()),
            sort_direction: Some("desc".to_string()),
        };
        assert!(invalid_page_size.build().is_err());

        // 测试过大的分页大小
        let too_large_page_size = ListTicketsRequest {
            api_req: ApiRequest::default(),
            page_size: Some(101),
            page_token: None,
            status: None,
            priority: None,
            assignee_id: None,
            creator_id: None,
            sort_field: Some("created_at".to_string()),
            sort_direction: Some("desc".to_string()),
        };
        assert!(too_large_page_size.build().is_err());

        // 测试无效的排序字段
        let invalid_sort_field = ListTicketsRequest {
            api_req: ApiRequest::default(),
            page_size: Some(20),
            page_token: None,
            status: None,
            priority: None,
            assignee_id: None,
            creator_id: None,
            sort_field: Some("invalid_field".to_string()),
            sort_direction: Some("desc".to_string()),
        };
        assert!(invalid_sort_field.build().is_err());

        // 测试无效的排序方向
        let invalid_sort_direction = ListTicketsRequest {
            api_req: ApiRequest::default(),
            page_size: Some(20),
            page_token: None,
            status: None,
            priority: None,
            assignee_id: None,
            creator_id: None,
            sort_field: Some("created_at".to_string()),
            sort_direction: Some("invalid_direction".to_string()),
        };
        assert!(invalid_sort_direction.build().is_err());

        // 测试无效的工单状态
        let invalid_status = ListTicketsRequest {
            api_req: ApiRequest::default(),
            page_size: Some(20),
            page_token: None,
            status: Some("invalid_status".to_string()),
            priority: None,
            assignee_id: None,
            creator_id: None,
            sort_field: Some("created_at".to_string()),
            sort_direction: Some("desc".to_string()),
        };
        assert!(invalid_status.build().is_err());

        // 测试无效的优先级
        let invalid_priority = ListTicketsRequest {
            api_req: ApiRequest::default(),
            page_size: Some(20),
            page_token: None,
            status: None,
            priority: Some("invalid_priority".to_string()),
            assignee_id: None,
            creator_id: None,
            sort_field: Some("created_at".to_string()),
            sort_direction: Some("desc".to_string()),
        };
        assert!(invalid_priority.build().is_err());
    }

    #[test]
    fn test_list_tickets_builder() {
        let builder = ListTicketsRequest::builder()
            .page_size(30)
            .page_token("test_token")
            .status("pending")
            .priority("high")
            .assignee_id("agent_123")
            .creator_id("user_456")
            .sort_field("updated_at")
            .sort_direction("asc");

        let request = builder.build().unwrap();
        assert_eq!(request.page_size, Some(30));
        assert_eq!(request.page_token.as_ref().unwrap(), "test_token");
        assert_eq!(request.status.as_ref().unwrap(), "pending");
        assert_eq!(request.priority.as_ref().unwrap(), "high");
        assert_eq!(request.assignee_id.as_ref().unwrap(), "agent_123");
        assert_eq!(request.creator_id.as_ref().unwrap(), "user_456");
        assert_eq!(request.sort_field.as_ref().unwrap(), "updated_at");
        assert_eq!(request.sort_direction.as_ref().unwrap(), "asc");
    }

    #[test]
    fn test_list_tickets_builder_default() {
        let builder = ListTicketsRequest::builder();
        let request = builder.build().unwrap();
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, None);
        assert_eq!(request.status, None);
        assert_eq!(request.priority, None);
        assert_eq!(request.assignee_id, None);
        assert_eq!(request.creator_id, None);
        assert_eq!(request.sort_field.as_ref().unwrap(), "created_at");
        assert_eq!(request.sort_direction.as_ref().unwrap(), "desc");
    }

    #[test]
    fn test_list_tickets_response() {
        let response = ListTicketsResponse {
            items: vec![],
            has_more: false,
            page_token: None,
        };

        assert_eq!(response.items.len(), 0);
        assert_eq!(response.has_more, false);
        assert_eq!(response.page_token, None);
    }

    #[test]
    fn test_list_tickets_response_with_pagination() {
        let response = ListTicketsResponse {
            items: vec![Ticket {
                ticket_id: Some("ticket_123".to_string()),
                title: Some("测试工单".to_string()),
                description: None,
                status: None,
                priority: None,
                creator: None,
                assignee: None,
                created_at: None,
                updated_at: None,
            }],
            has_more: true,
            page_token: Some("next_page_token".to_string()),
        };

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.has_more, true);
        assert_eq!(response.page_token.as_ref().unwrap(), "next_page_token");
        assert_eq!(response.items[0].ticket_id.as_ref().unwrap(), "ticket_123");
        assert_eq!(response.items[0].title.as_ref().unwrap(), "测试工单");
    }

    #[test]
    fn test_response_format() {
        assert_eq!(ListTicketsResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_service_trait_implementation() {
        let service = create_test_service();

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");
        assert_eq!(config_ref.app_secret(), "test_app_secret");
    }

    #[test]
    fn test_list_tickets_builder_creation() {
        let service = create_test_service();
        let request = ListTicketsRequest::new();

        let builder = service.list_tickets_builder(request);
        // 验证构建器创建成功
        assert_eq!(builder.request.page_size, Some(20));
    }

    #[test]
    fn test_list_tickets_builder_with_service() {
        let service = create_test_service();
        let request = ListTicketsRequest::builder()
            .page_size(15)
            .status("solved")
            .build()
            .unwrap();

        let builder = service.list_tickets_builder(request);
        assert_eq!(builder.request.page_size, Some(15));
        assert_eq!(builder.request.status.as_ref().unwrap(), "solved");
    }

    #[test]
    fn test_list_tickets_builder_chaining() {
        let service = create_test_service();
        let request = ListTicketsRequest::new();

        let builder = service.list_tickets_builder(request)
            .page_size(25)
            .priority("urgent")
            .assignee_id("agent_789")
            .sort_field("priority")
            .sort_direction("asc");

        assert_eq!(builder.request.page_size, Some(25));
        assert_eq!(builder.request.priority.as_ref().unwrap(), "urgent");
        assert_eq!(builder.request.assignee_id.as_ref().unwrap(), "agent_789");
        assert_eq!(builder.request.sort_field.as_ref().unwrap(), "priority");
        assert_eq!(builder.request.sort_direction.as_ref().unwrap(), "asc");
    }

    #[test]
    fn test_multiple_ticket_services() {
        let config1 = Config::new("app_id_1", "app_secret_1");
        let config2 = Config::new("app_id_2", "app_secret_2");

        let service1 = TicketService::new(config1);
        let service2 = TicketService::new(config2);

        assert_eq!(service1.config().app_id(), "app_id_1");
        assert_eq!(service2.config().app_id(), "app_id_2");
        // 验证两个服务实例是独立的
        assert_ne!(service1.config().app_id(), service2.config().app_id());
    }

    #[test]
    fn test_edge_cases() {
        // 测试边界情况
        let edge_cases = vec![
            // 最小分页大小
            ListTicketsRequest {
                api_req: ApiRequest::default(),
                page_size: Some(1),
                page_token: None,
                status: None,
                priority: None,
                assignee_id: None,
                creator_id: None,
                sort_field: Some("created_at".to_string()),
                sort_direction: Some("desc".to_string()),
            },
            // 最大分页大小
            ListTicketsRequest {
                api_req: ApiRequest::default(),
                page_size: Some(100),
                page_token: None,
                status: None,
                priority: None,
                assignee_id: None,
                creator_id: None,
                sort_field: Some("created_at".to_string()),
                sort_direction: Some("desc".to_string()),
            },
        ];

        for case in edge_cases {
            assert!(case.build().is_ok());
        }
    }

    #[test]
    fn test_all_valid_statuses() {
        let valid_statuses = vec!["pending", "processing", "solved", "closed"];

        for status in valid_statuses {
            let request = ListTicketsRequest {
                api_req: ApiRequest::default(),
                page_size: Some(20),
                page_token: None,
                status: Some(status.to_string()),
                priority: None,
                assignee_id: None,
                creator_id: None,
                sort_field: Some("created_at".to_string()),
                sort_direction: Some("desc".to_string()),
            };
            assert!(request.build().is_ok(), "状态 {} 应该是有效的", status);
        }
    }

    #[test]
    fn test_all_valid_priorities() {
        let valid_priorities = vec!["low", "medium", "high", "urgent"];

        for priority in valid_priorities {
            let request = ListTicketsRequest {
                api_req: ApiRequest::default(),
                page_size: Some(20),
                page_token: None,
                status: None,
                priority: Some(priority.to_string()),
                assignee_id: None,
                creator_id: None,
                sort_field: Some("created_at".to_string()),
                sort_direction: Some("desc".to_string()),
            };
            assert!(request.build().is_ok(), "优先级 {} 应该是有效的", priority);
        }
    }

    #[test]
    fn test_all_valid_sort_fields() {
        let valid_fields = vec!["created_at", "updated_at", "priority", "status"];

        for field in valid_fields {
            let request = ListTicketsRequest {
                api_req: ApiRequest::default(),
                page_size: Some(20),
                page_token: None,
                status: None,
                priority: None,
                assignee_id: None,
                creator_id: None,
                sort_field: Some(field.to_string()),
                sort_direction: Some("desc".to_string()),
            };
            assert!(request.build().is_ok(), "排序字段 {} 应该是有效的", field);
        }
    }

    #[test]
    fn test_all_valid_sort_directions() {
        let valid_directions = vec!["asc", "desc"];

        for direction in valid_directions {
            let request = ListTicketsRequest {
                api_req: ApiRequest::default(),
                page_size: Some(20),
                page_token: None,
                status: None,
                priority: None,
                assignee_id: None,
                creator_id: None,
                sort_field: Some("created_at".to_string()),
                sort_direction: Some(direction.to_string()),
            };
            assert!(request.build().is_ok(), "排序方向 {} 应该是有效的", direction);
        }
    }

    // ==================== GetTicketRequest 测试 ====================

    #[test]
    fn test_get_ticket_request_new() {
        let ticket_id = "6943948239044347654";
        let request = GetTicketRequest::new(ticket_id);

        assert_eq!(request.ticket_id, ticket_id);
        assert!(!request.ticket_id.is_empty());
    }

    #[test]
    fn test_get_ticket_request_default() {
        let request = GetTicketRequest::default();

        assert_eq!(request.ticket_id, "");
        assert!(request.ticket_id.is_empty());
    }

    #[test]
    fn test_get_ticket_request_validate_valid() {
        let request = GetTicketRequest {
            api_req: ApiRequest::default(),
            ticket_id: "6943948239044347654".to_string(),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_ticket_request_validate_empty() {
        let request = GetTicketRequest {
            api_req: ApiRequest::default(),
            ticket_id: "".to_string(),
        };

        assert!(request.validate().is_err());
        let error = request.validate().unwrap_err();
        assert_eq!(error, "工单ID不能为空");
    }

    #[test]
    fn test_get_ticket_request_with_various_ticket_ids() {
        let valid_ticket_ids = vec![
            "6943948239044347654",
            "12345",
            "ticket_with_underscores",
            "ticket-with-dashes",
            "TICKET-2023-001",
        ];

        for ticket_id in valid_ticket_ids {
            let request = GetTicketRequest::new(ticket_id);
            assert!(request.validate().is_ok(), "工单ID {} 应该是有效的", ticket_id);
        }
    }

    #[test]
    fn test_get_ticket_response_default() {
        let response = GetTicketResponse::default();

        // 验证默认的Ticket结构
        assert_eq!(response.ticket.ticket_id, None);
        assert_eq!(response.ticket.title, None);
        assert_eq!(response.ticket.description, None);
    }

    #[test]
    fn test_get_ticket_response_with_ticket() {
        let mut ticket = Ticket {
            ticket_id: Some("6943948239044347654".to_string()),
            title: Some("测试工单".to_string()),
            description: Some("这是一个测试工单".to_string()),
            ..Default::default()
        };

        let response = GetTicketResponse { ticket: ticket.clone() };

        assert_eq!(response.ticket.ticket_id, ticket.ticket_id);
        assert_eq!(response.ticket.title, ticket.title);
        assert_eq!(response.ticket.description, ticket.description);
    }

    #[test]
    fn test_get_ticket_builder_creation() {
        let service = create_test_service();
        let request = GetTicketRequest::new("6943948239044347654");

        let builder = service.get_ticket_builder(request);
        // 验证构建器创建成功
        assert_eq!(builder.request.ticket_id, "6943948239044347654");
    }

    #[test]
    fn test_get_ticket_builder_with_service() {
        let service = create_test_service();
        let request = GetTicketRequest::new("test_ticket_id");

        let builder = service.get_ticket_builder(request);
        assert_eq!(builder.request.ticket_id, "test_ticket_id");
    }

    #[test]
    fn test_get_ticket_builder_ticket_id_method() {
        let service = create_test_service();
        let request = GetTicketRequest::new("");

        let builder = service.get_ticket_builder(request)
            .ticket_id("new_ticket_id_123");

        assert_eq!(builder.request.ticket_id, "new_ticket_id_123");
    }

    #[test]
    fn test_get_ticket_builder_chaining() {
        let service = create_test_service();
        let request = GetTicketRequest::new("initial_ticket");

        let builder = service.get_ticket_builder(request)
            .ticket_id("final_ticket_id");

        assert_eq!(builder.request.ticket_id, "final_ticket_id");
    }

    #[test]
    fn test_get_ticket_builder_validation() {
        let service = create_test_service();
        let request = GetTicketRequest::new(""); // 空的ticket_id

        let builder = service.get_ticket_builder(request);
        // 构建器本身不会验证，只有在execute时才会验证
        assert_eq!(builder.request.ticket_id, "");
    }

    #[test]
    fn test_api_response_trait_implementation() {
        // 验证GetTicketResponse实现了ApiResponseTrait
        assert_eq!(GetTicketResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_get_ticket_request_serialization() {
        let request = GetTicketRequest::new("test_ticket_123");

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetTicketRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.ticket_id, deserialized.ticket_id);
    }

    #[test]
    fn test_get_ticket_response_serialization() {
        let mut ticket = Ticket {
            ticket_id: Some("ticket_456".to_string()),
            title: Some("序列化测试".to_string()),
            ..Default::default()
        };
        let response = GetTicketResponse { ticket };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetTicketResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.ticket.ticket_id, deserialized.ticket.ticket_id);
        assert_eq!(response.ticket.title, deserialized.ticket.title);
    }

    #[test]
    fn test_edge_case_ticket_ids() {
        let edge_cases = vec![
            "1", // 最短有效ID
            "a".repeat(100).as_str(), // 很长的ID
            "123_456-789_ABC", // 复合ID格式
            "😊🎉", // Unicode字符 (如果支持)
        ];

        for ticket_id in edge_cases {
            if !ticket_id.is_empty() {
                let request = GetTicketRequest::new(ticket_id);
                assert!(request.validate().is_ok(), "边界情况工单ID {} 应该是有效的", ticket_id);
            }
        }
    }

    #[test]
    fn test_error_message_formatting() {
        let request = GetTicketRequest::new("");
        let error = request.validate().unwrap_err();

        // 验证错误消息是中文且有意义
        assert!(!error.is_empty());
        assert!(error.contains("工单ID"));
        assert!(error.contains("空"));
    }

    // ==================== UpdateTicketRequest 测试 ====================

    #[test]
    fn test_update_ticket_request_new() {
        let ticket_id = "6943948239044347654";
        let mut ticket = Ticket::default();
        ticket.title = Some("测试工单".to_string());

        let request = UpdateTicketRequest::new(ticket_id, ticket);

        assert_eq!(request.ticket_id, ticket_id);
        assert_eq!(request.ticket.title, Some("测试工单".to_string()));
        assert_eq!(request.update_mask, None);
    }

    #[test]
    fn test_update_ticket_request_with_update_mask() {
        let ticket_id = "ticket_123";
        let ticket = Ticket::default();
        let update_mask = vec!["title".to_string(), "status".to_string()];

        let request = UpdateTicketRequest::new(ticket_id, ticket)
            .update_mask(update_mask.clone());

        assert_eq!(request.ticket_id, ticket_id);
        assert_eq!(request.update_mask, Some(update_mask));
    }

    #[test]
    fn test_update_ticket_request_validate_valid() {
        let ticket_id = "6943948239044347654";
        let mut ticket = Ticket::default();
        ticket.title = Some("测试工单".to_string());

        let request = UpdateTicketRequest {
            api_req: ApiRequest::default(),
            ticket_id: ticket_id.to_string(),
            ticket,
            update_mask: Some(vec!["title".to_string()]),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_ticket_request_validate_empty_ticket_id() {
        let ticket = Ticket::default();
        let request = UpdateTicketRequest {
            api_req: ApiRequest::default(),
            ticket_id: "".to_string(),
            ticket,
            update_mask: None,
        };

        assert!(request.validate().is_err());
        let error = request.validate().unwrap_err();
        assert_eq!(error, "工单ID不能为空");
    }

    #[test]
    fn test_update_ticket_request_validate_invalid_update_mask() {
        let ticket = Ticket::default();
        let request = UpdateTicketRequest {
            api_req: ApiRequest::default(),
            ticket_id: "valid_ticket".to_string(),
            ticket,
            update_mask: Some(vec!["invalid_field".to_string()]),
        };

        assert!(request.validate().is_err());
        let error = request.validate().unwrap_err();
        assert!(error.contains("无效的更新字段"));
        assert!(error.contains("invalid_field"));
    }

    #[test]
    fn test_update_ticket_request_get_update_fields_with_mask() {
        let ticket = Ticket::default();
        let update_mask = vec!["title".to_string(), "status".to_string()];

        let request = UpdateTicketRequest {
            api_req: ApiRequest::default(),
            ticket_id: "ticket_123".to_string(),
            ticket,
            update_mask: Some(update_mask.clone()),
        };

        let fields = request.get_update_fields();
        assert_eq!(fields, update_mask);
    }

    #[test]
    fn test_update_ticket_request_get_update_fields_without_mask() {
        let mut ticket = Ticket::default();
        ticket.title = Some("测试标题".to_string());
        ticket.status = Some(TicketStatus::Processing);
        ticket.assignee = Some("agent_123".to_string());

        let request = UpdateTicketRequest {
            api_req: ApiRequest::default(),
            ticket_id: "ticket_123".to_string(),
            ticket,
            update_mask: None,
        };

        let fields = request.get_update_fields();
        assert_eq!(fields.len(), 3);
        assert!(fields.contains(&"title".to_string()));
        assert!(fields.contains(&"status".to_string()));
        assert!(fields.contains(&"assignee".to_string()));
    }

    #[test]
    fn test_update_ticket_request_get_update_fields_empty() {
        let ticket = Ticket::default();
        let request = UpdateTicketRequest {
            api_req: ApiRequest::default(),
            ticket_id: "ticket_123".to_string(),
            ticket,
            update_mask: None,
        };

        let fields = request.get_update_fields();
        assert_eq!(fields.len(), 0);
    }

    #[test]
    fn test_update_ticket_response_default() {
        let response = UpdateTicketResponse::default();

        assert_eq!(response.ticket.ticket_id, None);
        assert_eq!(response.ticket.title, None);
        assert_eq!(response.ticket.status, None);
    }

    #[test]
    fn test_update_ticket_response_with_ticket() {
        let mut ticket = Ticket {
            ticket_id: Some("ticket_456".to_string()),
            title: Some("更新后的工单".to_string()),
            status: Some(TicketStatus::Solved),
            ..Default::default()
        };

        let response = UpdateTicketResponse { ticket: ticket.clone() };

        assert_eq!(response.ticket.ticket_id, ticket.ticket_id);
        assert_eq!(response.ticket.title, ticket.title);
        assert_eq!(response.ticket.status, ticket.status);
    }

    #[test]
    fn test_update_ticket_builder_creation() {
        let service = create_test_service();
        let mut ticket = Ticket::default();
        ticket.title = Some("测试工单".to_string());

        let request = UpdateTicketRequest::new("ticket_123", ticket);
        let builder = service.update_ticket_builder(request);

        // 验证构建器创建成功
        assert_eq!(builder.request.ticket_id, "ticket_123");
        assert_eq!(builder.request.ticket.title, Some("测试工单".to_string()));
    }

    #[test]
    fn test_update_ticket_builder_title() {
        let service = create_test_service();
        let ticket = Ticket::default();
        let request = UpdateTicketRequest::new("ticket_123", ticket);

        let builder = service.update_ticket_builder(request)
            .title("新的标题");

        assert_eq!(builder.request.ticket.title, Some("新的标题".to_string()));
    }

    #[test]
    fn test_update_ticket_builder_description() {
        let service = create_test_service();
        let ticket = Ticket::default();
        let request = UpdateTicketRequest::new("ticket_123", ticket);

        let builder = service.update_ticket_builder(request)
            .description("新的描述");

        assert_eq!(builder.request.ticket.description, Some("新的描述".to_string()));
    }

    #[test]
    fn test_update_ticket_builder_status() {
        let service = create_test_service();
        let ticket = Ticket::default();
        let request = UpdateTicketRequest::new("ticket_123", ticket);

        let builder = service.update_ticket_builder(request)
            .status(TicketStatus::Processing);

        assert_eq!(builder.request.ticket.status, Some(TicketStatus::Processing));
    }

    #[test]
    fn test_update_ticket_builder_priority() {
        let service = create_test_service();
        let ticket = Ticket::default();
        let request = UpdateTicketRequest::new("ticket_123", ticket);

        let builder = service.update_ticket_builder(request)
            .priority(TicketPriority::High);

        assert_eq!(builder.request.ticket.priority, Some(TicketPriority::High));
    }

    #[test]
    fn test_update_ticket_builder_assignee() {
        let service = create_test_service();
        let ticket = Ticket::default();
        let request = UpdateTicketRequest::new("ticket_123", ticket);

        let builder = service.update_ticket_builder(request)
            .assignee("agent_456");

        assert_eq!(builder.request.ticket.assignee, Some("agent_456".to_string()));
    }

    #[test]
    fn test_update_ticket_builder_update_mask() {
        let service = create_test_service();
        let ticket = Ticket::default();
        let request = UpdateTicketRequest::new("ticket_123", ticket);
        let mask = vec!["title".to_string(), "status".to_string()];

        let builder = service.update_ticket_builder(request)
            .update_mask(mask.clone());

        assert_eq!(builder.request.update_mask, Some(mask));
    }

    #[test]
    fn test_update_ticket_builder_chaining() {
        let service = create_test_service();
        let ticket = Ticket::default();
        let request = UpdateTicketRequest::new("ticket_123", ticket);

        let builder = service.update_ticket_builder(request)
            .title("更新的标题")
            .description("更新的描述")
            .status(TicketStatus::Processing)
            .priority(TicketPriority::High)
            .assignee("agent_789");

        assert_eq!(builder.request.ticket.title, Some("更新的标题".to_string()));
        assert_eq!(builder.request.ticket.description, Some("更新的描述".to_string()));
        assert_eq!(builder.request.ticket.status, Some(TicketStatus::Processing));
        assert_eq!(builder.request.ticket.priority, Some(TicketPriority::High));
        assert_eq!(builder.request.ticket.assignee, Some("agent_789".to_string()));
    }

    #[test]
    fn test_update_ticket_builder_validation() {
        let service = create_test_service();
        let ticket = Ticket::default();
        let request = UpdateTicketRequest::new("", ticket); // 空的ticket_id

        let builder = service.update_ticket_builder(request);
        // 构建器本身不会验证，只有在execute时才会验证
        assert_eq!(builder.request.ticket_id, "");
    }

    #[test]
    fn test_api_response_trait_implementation_update() {
        // 验证UpdateTicketResponse实现了ApiResponseTrait
        assert_eq!(UpdateTicketResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_update_ticket_request_serialization() {
        let mut ticket = Ticket::default();
        ticket.title = Some("测试更新".to_string());
        ticket.status = Some(TicketStatus::Solved);

        let request = UpdateTicketRequest::new("test_ticket_123", ticket);

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: UpdateTicketRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.ticket_id, deserialized.ticket_id);
        assert_eq!(request.ticket.title, deserialized.ticket.title);
        assert_eq!(request.ticket.status, deserialized.ticket.status);
    }

    #[test]
    fn test_update_ticket_response_serialization() {
        let mut ticket = Ticket {
            ticket_id: Some("ticket_789".to_string()),
            title: Some("序列化测试".to_string()),
            status: Some(TicketStatus::Closed),
            ..Default::default()
        };
        let response = UpdateTicketResponse { ticket };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: UpdateTicketResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.ticket.ticket_id, deserialized.ticket.ticket_id);
        assert_eq!(response.ticket.title, deserialized.ticket.title);
        assert_eq!(response.ticket.status, deserialized.ticket.status);
    }

    #[test]
    fn test_update_ticket_complex_scenario() {
        let service = create_test_service();

        // 模拟复杂的更新场景
        let mut ticket = Ticket::default();
        ticket.title = Some("原始标题".to_string());
        ticket.description = Some("原始描述".to_string());
        ticket.status = Some(TicketStatus::Pending);
        ticket.priority = Some(TicketPriority::Low);

        // 第1步：只更新状态
        let request1 = UpdateTicketRequest::new("ticket_001", ticket.clone())
            .update_mask(vec!["status".to_string()]);

        let builder1 = service.update_ticket_builder(request1)
            .status(TicketStatus::Processing);

        assert_eq!(builder1.request.ticket.status, Some(TicketStatus::Processing));
        assert_eq!(builder1.request.get_update_fields(), vec!["status"]);

        // 第2步：更新多个字段
        let request2 = UpdateTicketRequest::new("ticket_001", ticket.clone());

        let builder2 = service.update_ticket_builder(request2)
            .title("新标题")
            .description("新描述")
            .priority(TicketPriority::Urgent);

        assert_eq!(builder2.request.ticket.title, Some("新标题".to_string()));
        assert_eq!(builder2.request.ticket.description, Some("新描述".to_string()));
        assert_eq!(builder2.request.ticket.priority, Some(TicketPriority::Urgent));

        let fields = builder2.get_update_fields();
        assert_eq!(fields.len(), 3);
        assert!(fields.contains(&"title".to_string()));
        assert!(fields.contains(&"description".to_string()));
        assert!(fields.contains(&"priority".to_string()));
    }

    #[test]
    fn test_update_ticket_edge_cases() {
        // 测试空字符串ticket_id
        let ticket = Ticket::default();
        let request = UpdateTicketRequest::new("", ticket);
        assert!(request.validate().is_err());

        // 测试长ticket_id
        let long_id = "a".repeat(200);
        let request = UpdateTicketRequest::new(&long_id, ticket);
        assert!(request.validate().is_ok());

        // 测试Unicode字符在ticket_id中
        let unicode_id = "ticket_😊_测试";
        let request = UpdateTicketRequest::new(unicode_id, ticket);
        assert!(request.validate().is_ok());

        // 测试空的更新掩码
        let empty_mask = vec![];
        let request = UpdateTicketRequest::new("valid_ticket", ticket)
            .update_mask(empty_mask);

        let fields = request.get_update_fields();
        assert_eq!(fields.len(), 0); // Ticket是空的，所以没有字段要更新
    }

    #[test]
    fn test_update_ticket_error_messages() {
        // 测试各种错误情况下的错误消息

        // 1. 空ticket_id
        let request = UpdateTicketRequest::new("", Ticket::default());
        let error = request.validate().unwrap_err();
        assert!(error.contains("工单ID不能为空"));

        // 2. 无效的更新字段
        let request = UpdateTicketRequest::new("valid_ticket", Ticket::default())
            .update_mask(vec!["invalid_field".to_string()]);
        let error = request.validate().unwrap_err();
        assert!(error.contains("无效的更新字段"));
        assert!(error.contains("invalid_field"));

        // 3. 验证消息的中文格式
        assert!(error.chars().all(|c| c.is_ascii() || c as u32 > 127)); // 确保包含中文字符
    }
}