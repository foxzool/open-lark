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
    service::payroll::models::{PageResponse, Paygroup, PaygroupListRequest},
};

/// 薪资组服务
pub struct PaygroupService {
    pub config: Config,
}

/// 薪资组列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PaygroupListResponse {
    /// 薪资组列表
    #[serde(flatten)]
    pub paygroups: PageResponse<Paygroup>,
}

impl ApiResponseTrait for PaygroupListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PaygroupService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取薪资组基本信息
    ///
    /// 该接口用于获取企业配置的薪资组列表，包括薪资组的
    /// 基本信息、发薪周期设置、发薪日配置等。薪资组是
    /// 发薪系统中的基础配置，用于对员工进行分组管理。
    ///
    /// # 参数
    ///
    /// - `request`: 薪资组查询请求参数，包括：
    ///   - `page_size`: 分页大小
    ///   - `page_token`: 分页标记
    ///   - `status`: 薪资组状态筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的薪资组列表，包括：
    /// - 薪资组基本信息（ID、名称、类型、状态等）
    /// - 发薪周期配置（周期类型等）
    /// - 发薪日设置（发薪日类型、具体日期、节假日调整规则等）
    /// - 关联员工数量统计
    /// - 创建和更新时间
    /// - 描述信息
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::payroll::models::PaygroupListRequest;
    ///
    /// let request = PaygroupListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     status: Some("active".to_string()),
    /// };
    ///
    /// let response = client.payroll.paygroup.list_paygroups(request, None).await?;
    /// ```
    pub async fn list_paygroups(
        &self,
        request: PaygroupListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PaygroupListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: PAYROLL_V1_PAYGROUPS.to_string(),
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

impl Service for PaygroupService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "paygroup"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
