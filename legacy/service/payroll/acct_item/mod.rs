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
        SDKResult,
    },
    service::payroll::models::{AcctItem, AcctItemListRequest, PageResponse},
};

/// 算薪项服务
pub struct AcctItemService {
    pub config: Config,
}

/// 算薪项列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AcctItemListResponse {
    /// 算薪项列表
    #[serde(flatten)]
    pub acct_items: PageResponse<AcctItem>,
}

impl ApiResponseTrait for AcctItemListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AcctItemService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量查询算薪项
    ///
    /// 该接口用于批量查询企业配置的算薪项信息，包括算薪项的
    /// 基本配置、计算方式、税收和社保相关设置等。算薪项是
    /// 发薪系统中的基础配置，用于定义各种收入和扣除项目。
    ///
    /// # 参数
    ///
    /// - `request`: 算薪项列表查询请求参数，包括：
    ///   - `page_size`: 分页大小
    ///   - `page_token`: 分页标记
    ///   - `item_type`: 算薪项类型筛选
    ///   - `paygroup_id`: 薪资组ID筛选
    ///   - `status`: 状态筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的算薪项列表，包括：
    /// - 算薪项基本信息（ID、名称、类型、分类等）
    /// - 计算配置（计算方式、公式等）
    /// - 税务和社保设置（是否参与个税、社保计算等）
    /// - 显示配置（显示顺序、状态等）
    /// - 创建和更新时间
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::payroll::models::AcctItemListRequest;
    ///
    /// let request = AcctItemListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     item_type: Some("income".to_string()),
    ///     paygroup_id: Some("paygroup_123".to_string()),
    ///     status: Some("active".to_string()),
    /// };
    ///
    /// let response = client.payroll.acct_item.list_acct_items(request, None).await?;
    /// ```
    pub async fn list_acct_items(
        &self,
        request: AcctItemListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AcctItemListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: PAYROLL_V1_ACCT_ITEMS.to_string(),
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

        if let Some(item_type) = request.item_type {
            api_req.query_params.insert("item_type", item_type);
        }

        if let Some(paygroup_id) = request.paygroup_id {
            api_req.query_params.insert("paygroup_id", paygroup_id);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
