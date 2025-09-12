use reqwest::Method;
use serde_json::json;

use crate::{
    core::{
        api_resp::BaseResponse, config::Config, constants::AccessTokenType, http::Transport,
        req_option::RequestOption, SDKResult,
    },
    impl_executable_builder_owned,
};

use super::models::{
    QueryStatsFieldsRequest, QueryStatsFieldsRespData, QueryStatsSettingsRequest,
    QueryStatsSettingsRespData, QueryUserStatsDataRequest, QueryUserStatsDataRespData,
    UpdateUserStatsDataRequest, UpdateUserStatsDataRespData,
};

/// 用户统计数据服务
pub struct UserStatsDataService {
    pub config: Config,
}

impl UserStatsDataService {
    /// 更新统计设置
    ///
    /// 该接口用于更新考勤统计的设置，包括统计范围、日期、用户和字段等。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/update>
    pub async fn update(
        &self,
        request: UpdateUserStatsDataRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateUserStatsDataRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/attendance/v1/user_stats_datas/update".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "stats_setting": request.stats_setting
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询统计设置
    ///
    /// 该接口用于查询当前考勤统计的设置信息。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/query>
    pub async fn query_settings(
        &self,
        request: QueryStatsSettingsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryStatsSettingsRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/attendance/v1/user_stats_datas/query".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询统计表头
    ///
    /// 该接口用于查询考勤统计的可用字段信息，包括字段名称、类型等。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/query-2>
    pub async fn query_fields(
        &self,
        request: QueryStatsFieldsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryStatsFieldsRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/attendance/v1/user_stats_datas/query_fields".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(locale) = request.locale {
            api_req.query_params.insert("locale", locale);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询统计数据
    ///
    /// 该接口用于查询用户的考勤统计数据，支持按日期范围、用户和字段进行筛选。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/query-3>
    pub async fn query_data(
        &self,
        request: QueryUserStatsDataRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryUserStatsDataRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/attendance/v1/user_stats_datas/query_data".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(locale) = request.locale {
            api_req.query_params.insert("locale", locale);
        }

        // 构建请求体
        let body = json!({
            "start_date": request.start_date,
            "end_date": request.end_date,
            "user_ids": request.user_ids,
            "need_fields": request.need_fields
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// Builder implementations
impl_executable_builder_owned!(
    UpdateUserStatsDataRequest,
    UserStatsDataService,
    UpdateUserStatsDataRequest,
    BaseResponse<UpdateUserStatsDataRespData>,
    update
);

impl_executable_builder_owned!(
    QueryStatsSettingsRequest,
    UserStatsDataService,
    QueryStatsSettingsRequest,
    BaseResponse<QueryStatsSettingsRespData>,
    query_settings
);

impl_executable_builder_owned!(
    QueryStatsFieldsRequest,
    UserStatsDataService,
    QueryStatsFieldsRequest,
    BaseResponse<QueryStatsFieldsRespData>,
    query_fields
);

impl_executable_builder_owned!(
    QueryUserStatsDataRequest,
    UserStatsDataService,
    QueryUserStatsDataRequest,
    BaseResponse<QueryUserStatsDataRespData>,
    query_data
);
