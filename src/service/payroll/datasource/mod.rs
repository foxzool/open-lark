use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::payroll::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::payroll::models::{Datasource, DatasourceListRequest, PageResponse},
};

/// 外部数据源服务
pub struct DatasourceService {
    pub config: Config,
}

/// 外部数据源配置列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DatasourceListResponse {
    /// 外部数据源列表
    #[serde(flatten)]
    pub datasources: PageResponse<Datasource>,
}

impl ApiResponseTrait for DatasourceListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DatasourceService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取外部数据源配置信息
    ///
    /// 该接口用于获取企业配置的外部数据源列表，包括数据源的
    /// 基本信息、字段配置、状态等详细信息。外部数据源用于
    /// 与第三方薪酬系统集成，实现算薪数据的同步。
    ///
    /// # 参数
    ///
    /// - `request`: 外部数据源列表查询请求参数，包括：
    ///   - `page_size`: 分页大小
    ///   - `page_token`: 分页标记
    ///   - `status`: 数据源状态筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的外部数据源配置列表，包括：
    /// - 数据源基本信息（ID、名称、类型、状态等）
    /// - 字段配置信息（字段ID、名称、类型、是否必填等）
    /// - 创建和更新时间
    /// - 数据源描述信息
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::payroll::models::DatasourceListRequest;
    ///
    /// let request = DatasourceListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     status: Some("active".to_string()),
    /// };
    ///
    /// let response = client.payroll.datasource.list_datasources(request, None).await?;
    /// ```
    pub async fn list_datasources(
        &self,
        request: DatasourceListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DatasourceListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: PAYROLL_V1_DATASOURCES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for DatasourceService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "datasource"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
