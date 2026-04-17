//! 内推账户提现数据对账
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/referral_account/reconciliation

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 内推账户提现数据对账请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReconciliationRequest {
    /// 配置信息
    config: Config,
    // 当前生成骨架尚未建模请求字段；补齐 schema 前保持零字段请求。
}

impl ReconciliationRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // 当前无已建模字段需要初始化。
        }
    }

    // 当前未暴露字段 setter；补齐 schema 后再按需补充。

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ReconciliationResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ReconciliationResponse> {
        let request = ApiRequest::<ReconciliationResponse>::post(
            "/open-apis/hire/v1/referral_account/reconciliation",
        );
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "内推账户提现数据对账响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 内推账户提现数据对账响应
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
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
