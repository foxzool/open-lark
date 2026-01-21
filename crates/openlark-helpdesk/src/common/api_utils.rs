use openlark_core::{error, SDKResult};

pub fn extract_response_data<T>(
    response: openlark_core::api::Response<T>,
    context: &str,
) -> SDKResult<T> {
    response.data.ok_or_else(|| {
        error::validation_error(
            format!("{}响应数据为空", context).as_str(),
            "服务器没有返回有效的数据",
        )
    })
}

pub fn serialize_params<T: serde::Serialize>(
    params: &T,
    context: &str,
) -> SDKResult<serde_json::Value> {
    serde_json::to_value(params).map_err(|e| {
        error::validation_error(
            format!("{}参数序列化失败", context).as_str(),
            format!("无法序列化请求参数: {}", e).as_str(),
        )
    })
}
