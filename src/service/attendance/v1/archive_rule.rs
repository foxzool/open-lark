use reqwest::Method;
use serde_json::json;

use crate::core::{
    api_resp::BaseResponse, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption, SDKResult,
};

use super::models::{
    DelArchiveReportRequest, DelArchiveReportRespData, ListArchiveRulesRequest,
    ListArchiveRulesRespData, QueryArchiveStatsFieldsRequest, QueryArchiveStatsFieldsRespData,
    UploadArchiveReportRequest, UploadArchiveReportRespData,
};

/// 归档报表服务
pub struct ArchiveRuleService {
    pub config: Config,
}

impl ArchiveRuleService {
    /// 查询归档报表表头
    ///
    /// 该接口用于查询指定归档规则的报表字段定义信息。
    ///
    /// <https://open.feishu.cn/document/attendance-v1/archive_rule/user_stats_fields_query>
    pub async fn query_user_stats_fields(
        &self,
        request: QueryArchiveStatsFieldsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryArchiveStatsFieldsRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = format!(
            "/open-apis/attendance/v1/archive_rules/{}/user_stats_fields",
            request.archive_rule_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 写入归档报表结果
    ///
    /// 该接口用于向指定归档规则写入报表数据，支持批量上传考勤统计结果。
    ///
    /// <https://open.feishu.cn/document/attendance-v1/archive_rule/upload_report>
    pub async fn upload_report(
        &self,
        request: UploadArchiveReportRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UploadArchiveReportRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/attendance/v1/archive_rules/{}/upload_report",
            request.archive_rule_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

        // 构建请求体
        let body = json!({
            "report_data": request.report_data
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 删除归档报表行数据
    ///
    /// 该接口用于删除指定的归档报表数据记录。
    ///
    /// <https://open.feishu.cn/document/attendance-v1/archive_rule/del_report>
    pub async fn del_report(
        &self,
        request: DelArchiveReportRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DelArchiveReportRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/attendance/v1/archive_rules/{}/del_report",
            request.archive_rule_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

        // 构建请求体
        let body = json!({
            "record_ids": request.record_ids
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询所有归档规则
    ///
    /// 该接口用于获取企业的所有归档规则列表，包括规则名称、状态等信息。
    ///
    /// <https://open.feishu.cn/document/attendance-v1/archive_rule/list>
    pub async fn list(
        &self,
        request: ListArchiveRulesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListArchiveRulesRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/attendance/v1/archive_rules".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}
