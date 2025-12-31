//! 通用模型定义
//!
//! 该文件用于存放跨业务复用的通用数据结构（不算 API）。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 通用空 data 响应
///
/// 适用于形如 `{ "code": 0, "msg": "success", "data": {} }` 的接口响应。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmptyData {}

impl ApiResponseTrait for EmptyData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

