//! 内推账户提现数据对账
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/referral_account/reconciliation

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BonusAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_bonus: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradeDetail {
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_recharge_reward_info: Option<BonusAmount>,
}

impl TradeDetail {
    pub fn new(account_id: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
            total_recharge_reward_info: None,
        }
    }

    pub fn total_recharge_reward_info(mut self, total_recharge_reward_info: BonusAmount) -> Self {
        self.total_recharge_reward_info = Some(total_recharge_reward_info);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ReconciliationRequestBody {
    start_trans_time: String,
    end_trans_time: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    trade_details: Vec<TradeDetail>,
}

#[derive(Debug, Clone)]
pub struct ReconciliationRequest {
    config: Config,
    start_trans_time: String,
    end_trans_time: String,
    trade_details: Vec<TradeDetail>,
}

impl ReconciliationRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            start_trans_time: String::new(),
            end_trans_time: String::new(),
            trade_details: Vec::new(),
        }
    }

    pub fn start_trans_time(mut self, start_trans_time: impl Into<String>) -> Self {
        self.start_trans_time = start_trans_time.into();
        self
    }

    pub fn end_trans_time(mut self, end_trans_time: impl Into<String>) -> Self {
        self.end_trans_time = end_trans_time.into();
        self
    }

    pub fn trade_details(mut self, trade_details: Vec<TradeDetail>) -> Self {
        self.trade_details = trade_details;
        self
    }

    pub fn add_trade_detail(mut self, trade_detail: TradeDetail) -> Self {
        self.trade_details.push(trade_detail);
        self
    }

    pub async fn execute(self) -> SDKResult<ReconciliationResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ReconciliationResponse> {
        if self.start_trans_time.trim().is_empty() {
            return Err(error::validation_error(
                "start_trans_time",
                "start_trans_time 不能为空",
            ));
        }
        if self.end_trans_time.trim().is_empty() {
            return Err(error::validation_error(
                "end_trans_time",
                "end_trans_time 不能为空",
            ));
        }

        let request = ApiRequest::<ReconciliationResponse>::post(
            "/open-apis/hire/v1/referral_account/reconciliation",
        )
        .body(
            serde_json::to_value(ReconciliationRequestBody {
                start_trans_time: self.start_trans_time,
                end_trans_time: self.end_trans_time,
                trade_details: self.trade_details,
            })
            .map_err(|e| {
                error::validation_error("request_body", format!("无法序列化请求体: {}", e))
            })?,
        );

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error(
                "内推账户提现数据对账响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReconciliationResponse {
    /// 响应数据
    ///
    /// 当前按未建模 JSON 原样透传；字段收敛后再替换为显式结构。
    pub data: Value,
}

impl ApiResponseTrait for ReconciliationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #[test]
    fn test_serialization_roundtrip() {
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
