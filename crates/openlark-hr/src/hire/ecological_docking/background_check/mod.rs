use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::{
        api::ApiRequest,
        api::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::hire::*,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    };
use crate::hire::models::{CommonResponse, PageResponse};

/// 背调服务
pub struct BackgroundCheckService {
    pub config: Config,
}

/// 背调包信息
#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundCheckPackage {
    /// 背调包ID
    pub id: String,
    /// 背调包名称
    pub name: String,
    /// 背调包描述
    pub description: Option<String>,
    /// 背调供应商
    pub vendor: String,
    /// 背调项目列表
    pub check_items: Vec<BackgroundCheckItem>,
    /// 包价格
    pub price: Option<String>,
    /// 币种
    pub currency: Option<String>,
    /// 执行时长（天）
    pub duration_days: u32,
    /// 是否启用
    pub enabled: bool,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 背调项目
#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundCheckItem {
    /// 项目ID
    pub id: String,
    /// 项目名称
    pub name: String,
    /// 项目类型
    pub item_type: String,
    /// 项目描述
    pub description: Option<String>,
    /// 是否必须
    pub required: bool,
    /// 执行时长（天）
    pub duration_days: u32,
}

/// 背调订单信息
#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundCheckOrder {
    /// 订单ID
    pub id: String,
    /// 投递ID
    pub application_id: String,
    /// 候选人ID
    pub talent_id: String,
    /// 背调包ID
    pub package_id: String,
    /// 订单状态
    pub status: String,
    /// 背调结果
    pub result: Option<String>,
    /// 背调报告URL
    pub report_url: Option<String>,
    /// 订单金额
    pub amount: Option<String>,
    /// 币种
    pub currency: Option<String>,
    /// 开始时间
    pub start_time: Option<String>,
    /// 完成时间
    pub completion_time: Option<String>,
    /// 备注信息
    pub remark: Option<String>,
    /// 背调项目结果
    pub item_results: Vec<BackgroundCheckItemResult>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 背调项目结果
#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundCheckItemResult {
    /// 项目ID
    pub item_id: String,
    /// 项目名称
    pub item_name: String,
    /// 检查结果
    pub result: String,
    /// 结果详情
    pub details: Option<String>,
    /// 完成时间
    pub completion_time: Option<String>,
}

/// 背调供应商信息
#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundCheckVendor {
    /// 供应商ID
    pub id: String,
    /// 供应商名称
    pub name: String,
    /// 供应商描述
    pub description: Option<String>,
    /// 联系方式
    pub contact_info: Option<String>,
    /// API配置
    pub api_config: Option<serde_json::Value>,
    /// 支持的背调项目
    pub supported_items: Vec<String>,
    /// 是否启用
    pub enabled: bool,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 背调订单创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BackgroundCheckOrderCreateRequest {
    /// 投递ID
    pub application_id: String,
    /// 背调包ID
    pub package_id: String,
    /// 候选人信息
    pub candidate_info: BackgroundCheckCandidateInfo,
    /// 紧急程度
    pub urgency: Option<String>,
    /// 备注信息
    pub remark: Option<String>,
}

/// 背调候选人信息
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BackgroundCheckCandidateInfo {
    /// 候选人姓名
    pub name: String,
    /// 身份证号
    pub id_number: String,
    /// 手机号
    pub phone: String,
    /// 邮箱
    pub email: Option<String>,
    /// 工作经历
    pub work_experience: Vec<WorkExperience>,
    /// 教育经历
    pub education_experience: Vec<EducationExperience>,
}

/// 工作经历
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkExperience {
    /// 公司名称
    pub company_name: String,
    /// 职位
    pub position: String,
    /// 开始时间
    pub start_date: String,
    /// 结束时间
    pub end_date: Option<String>,
    /// 薪资
    pub salary: Option<String>,
    /// 离职原因
    pub leave_reason: Option<String>,
}

/// 教育经历
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EducationExperience {
    /// 学校名称
    pub school_name: String,
    /// 专业
    pub major: String,
    /// 学历
    pub degree: String,
    /// 开始时间
    pub start_date: String,
    /// 结束时间
    pub end_date: String,
}

/// 背调包列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BackgroundCheckPackageListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 供应商筛选
    pub vendor: Option<String>,
    /// 启用状态筛选
    pub enabled: Option<bool>,
}

/// 背调订单列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BackgroundCheckOrderListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 候选人ID
    pub talent_id: Option<String>,
    /// 订单状态
    pub status: Option<String>,
    /// 创建时间开始
    pub created_start_time: Option<String>,
    /// 创建时间结束
    pub created_end_time: Option<String>,
}

/// 背调包列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundCheckPackageListResponse {
    /// 背调包列表
    #[serde(flatten)]
    pub packages: PageResponse<BackgroundCheckPackage>,
}

impl ApiResponseTrait for BackgroundCheckPackageListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 背调订单列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundCheckOrderListResponse {
    /// 背调订单列表
    #[serde(flatten)]
    pub orders: PageResponse<BackgroundCheckOrder>,
}

impl ApiResponseTrait for BackgroundCheckOrderListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 背调订单详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundCheckOrderDetailResponse {
    /// 背调订单信息
    pub order: BackgroundCheckOrder,
}

impl ApiResponseTrait for BackgroundCheckOrderDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 背调操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundCheckOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 相关ID
    pub id: Option<String>,
}

impl ApiResponseTrait for BackgroundCheckOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BackgroundCheckService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取背调包列表
    ///
    /// 该接口用于获取可用的背调包列表，包括不同供应商
    /// 提供的背调服务包，支持按供应商、启用状态等条件筛选。
    ///
    /// # 参数
    ///
    /// - `request`: 背调包列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `vendor`: 供应商筛选
    ///   - `enabled`: 启用状态筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的背调包列表，包括：
    /// - 背调包基本信息列表
    /// - 背调项目和价格信息
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::ecological_docking::background_check::BackgroundCheckPackageListRequest;
    ///
    /// let request = BackgroundCheckPackageListRequest {
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     vendor: Some("某背调公司".to_string()),
    ///     enabled: Some(true),
    /// };
    ///
    /// let response = client.hire.ecological_docking.background_check.list_packages(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("背调包总数: {}", data.packages.items.len());
    ///     for package in &data.packages.items {
    ///         println!("背调包: {} 价格: {:?}", package.name, package.price);
    ///     }
    /// }
    /// ```
    pub async fn list_packages(
        &self,
        request: BackgroundCheckPackageListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<BackgroundCheckPackageListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_BACKGROUND_CHECK_PACKAGES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(vendor) = request.vendor {
            api_req.query_params.insert("vendor", vendor);
        }

        if let Some(enabled) = request.enabled {
            api_req.query_params.insert("enabled", enabled.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 创建背调订单
    ///
    /// 该接口用于为候选人创建背调订单，选择背调包
    /// 并提供候选人相关信息用于背调执行。
    ///
    /// # 参数
    ///
    /// - `request`: 背调订单创建请求参数，包括：
    ///   - `application_id`: 投递ID（必填）
    ///   - `package_id`: 背调包ID（必填）
    ///   - `candidate_info`: 候选人信息（必填）
    ///   - `urgency`: 紧急程度
    ///   - `remark`: 备注信息
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回背调订单创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `id`: 创建的订单ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::ecological_docking::background_check::{
    ///     BackgroundCheckOrderCreateRequest, BackgroundCheckCandidateInfo,
    ///     WorkExperience, EducationExperience
    /// };
    ///
    /// let request = BackgroundCheckOrderCreateRequest {
    ///     application_id: "app_123456".to_string(),
    ///     package_id: "pkg_789".to_string(),
    ///     candidate_info: BackgroundCheckCandidateInfo {
    ///         name: "张三".to_string(),
    ///         id_number: "123456789012345678".to_string(),
    ///         phone: "13800138000".to_string(),
    ///         email: Some("zhangsan@example.com".to_string()),
    ///         work_experience: vec![
    ///             WorkExperience {
    ///                 company_name: "某科技公司".to_string(),
    ///                 position: "高级工程师".to_string(),
    ///                 start_date: "2020-01-01".to_string(),
    ///                 end_date: Some("2023-12-31".to_string()),
    ///                 salary: Some("300000".to_string()),
    ///                 leave_reason: Some("寻求更好发展".to_string()),
    ///             },
    ///         ],
    ///         education_experience: vec![
    ///             EducationExperience {
    ///                 school_name: "某大学".to_string(),
    ///                 major: "计算机科学".to_string(),
    ///                 degree: "本科".to_string(),
    ///                 start_date: "2016-09-01".to_string(),
    ///                 end_date: "2020-06-30".to_string(),
    ///             },
    ///         ],
    ///     },
    ///     urgency: Some("normal".to_string()),
    ///     remark: Some("重要职位候选人".to_string()),
    /// };
    ///
    /// let response = client.hire.ecological_docking.background_check.create_order(request, None).await?;
    /// ```
    pub async fn create_order(
        &self,
        request: BackgroundCheckOrderCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<BackgroundCheckOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_BACKGROUND_CHECK_ORDERS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取背调订单详情
    ///
    /// 该接口用于获取指定背调订单的详细信息，包括
    /// 订单状态、背调结果、各项目完成情况等数据。
    ///
    /// # 参数
    ///
    /// - `order_id`: 背调订单ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回背调订单详细信息，包括：
    /// - 订单基本信息（状态、价格等）
    /// - 背调结果和报告
    /// - 各背调项目的完成情况
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let order_id = "order_123456";
    /// let response = client.hire.ecological_docking.background_check.get_order_detail(order_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("订单状态: {}", data.order.status);
    ///     println!("背调结果: {:?}", data.order.result);
    ///     println!("报告链接: {:?}", data.order.report_url);
    /// }
    /// ```
    pub async fn get_order_detail(
        &self,
        order_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<BackgroundCheckOrderDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_BACKGROUND_CHECK_ORDER_GET.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取背调订单列表
    ///
    /// 该接口用于获取企业的背调订单列表，支持按候选人、
    /// 状态、时间等条件筛选。返回的列表包含订单基本信息，
    /// 可用于背调进度管理和统计。
    ///
    /// # 参数
    ///
    /// - `request`: 背调订单列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `talent_id`: 候选人ID筛选
    ///   - `status`: 订单状态筛选
    ///   - `created_start_time`: 创建时间开始
    ///   - `created_end_time`: 创建时间结束
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的背调订单列表，包括：
    /// - 背调订单基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::ecological_docking::background_check::BackgroundCheckOrderListRequest;
    ///
    /// let request = BackgroundCheckOrderListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     talent_id: Some("talent_789".to_string()),
    ///     status: Some("in_progress".to_string()),
    ///     created_start_time: Some("2024-01-01T00:00:00Z".to_string()),
    ///     created_end_time: Some("2024-01-31T23:59:59Z".to_string()),
    /// };
    ///
    /// let response = client.hire.ecological_docking.background_check.list_orders(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("背调订单总数: {}", data.orders.items.len());
    ///     for order in &data.orders.items {
    ///         println!("订单: {} 状态: {}", order.id, order.status);
    ///     }
    /// }
    /// ```
    pub async fn list_orders(
        &self,
        request: BackgroundCheckOrderListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<BackgroundCheckOrderListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_BACKGROUND_CHECK_ORDERS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(talent_id) = request.talent_id {
            api_req.query_params.insert("talent_id", talent_id);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(created_start_time) = request.created_start_time {
            api_req
                .query_params
                .insert("created_start_time", created_start_time);
        }

        if let Some(created_end_time) = request.created_end_time {
            api_req
                .query_params
                .insert("created_end_time", created_end_time);
        }

        Transport::request(api_req, &self.config, option).await
    }
    /// 取消背调订单
    ///
    /// 该接口用于取消尚未完成的背调订单，
    /// 设置取消原因并处理退款。
    ///
    /// # 参数
    ///
    /// - `order_id`: 背调订单ID
    /// - `reason`: 取消原因
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let order_id = "order_123456";
    /// let reason = "候选人已withdrawn申请";
    ///
    /// let response = client.hire.ecological_docking.background_check.cancel_order(order_id, reason, None).await?;
    /// ```
    pub async fn cancel_order(
        &self,
        order_id: &str,
        reason: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<BackgroundCheckOperationResponse>> {
        #[derive(Serialize)]
        struct CancelOrderRequest {
            reason: String,
        }

        let request = CancelOrderRequest {
            reason: reason.to_string(),
        };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_BACKGROUND_CHECK_ORDER_CANCEL.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取背调报告
    ///
    /// 该接口用于下载已完成的背调报告，
    /// 返回报告文件的下载链接。
    ///
    /// # 参数
    ///
    /// - `order_id`: 背调订单ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let order_id = "order_123456";
    /// let response = client.hire.ecological_docking.background_check.get_report(order_id, None).await?;
    /// ```
    pub async fn get_report(
        &self,
        order_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<serde_json::Value>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_BACKGROUND_CHECK_ORDER_REPORT.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }
    /// 批量创建背调订单
    ///
    /// 该接口用于批量为多个候选人创建背调订单，
    /// 提高操作效率。
    ///
    /// # 参数
    ///
    /// - `orders`: 背调订单创建请求列表
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let orders = vec![
    ///     BackgroundCheckOrderCreateRequest {
    ///         application_id: "app_001".to_string(),
    ///         package_id: "pkg_789".to_string(),
    ///         // ... 其他参数
    ///     },
    ///     BackgroundCheckOrderCreateRequest {
    ///         application_id: "app_002".to_string(),
    ///         package_id: "pkg_789".to_string(),
    ///         // ... 其他参数
    ///     },
    /// ];
    ///
    /// let response = client.hire.ecological_docking.background_check.batch_create_orders(orders, None).await?;
    /// ```
    pub async fn batch_create_orders(
        &self,
        orders: Vec<BackgroundCheckOrderCreateRequest>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<BackgroundCheckOperationResponse>> {
        #[derive(Serialize)]
        struct BatchCreateRequest {
            orders: Vec<BackgroundCheckOrderCreateRequest>,
        }

        let request = BatchCreateRequest { orders };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_BACKGROUND_CHECK_ORDERS_BATCH.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
}
