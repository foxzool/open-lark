/// 获取妙记统计数据
///
/// 通过这个接口，可以获得妙记的访问情况统计，包含PV、UV、访问过的 user id、访问过的 user timestamp。
/// docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-statistics/get
/// 文档参考：https://open.feishu.cn/document/server-docs/minutes-v1/minute-statistics/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::minutes::v1::minute::models::MinuteStatistics;

/// 获取妙记统计数据请求
pub struct GetMinuteStatisticsRequest {
    minute_token: String,
    config: Config,
}

/// 获取妙记统计数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinuteStatisticsResponse {
    /// 统计数据
    pub statistics: MinuteStatistics,
}

impl ApiResponseTrait for GetMinuteStatisticsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMinuteStatisticsRequest {
    /// 创建获取妙记统计数据请求
    pub fn new(config: Config) -> Self {
        Self {
            minute_token: String::new(),
            config,
        }
    }

    /// 设置妙记Token
    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = minute_token.into();
        self
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/minutes-v1/minute-statistics/get
    pub async fn execute(self) -> SDKResult<GetMinuteStatisticsResponse> {
        // 验证必填字段
        validate_required!(self.minute_token, "妙记Token不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::MinutesApiV1;
        let api_endpoint = MinutesApiV1::StatisticsGet(self.minute_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetMinuteStatisticsResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
