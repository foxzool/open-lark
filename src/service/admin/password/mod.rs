#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Admin子模块 - 简化实现
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::api_resp::{ApiResponseTrait, ResponseFormat};
/// 简化的服务结构体
#[derive(Debug, Clone)]
pub struct SimpleService {
pub config: Config,
}
impl SimpleService {
}
    #[derive(Debug, Deserialize, Serialize)]
pub struct SimpleResponse;
impl ApiResponseTrait for SimpleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
}
// Type alias for compatibility
pub type ServiceType = SimpleService;
pub type ResponseType = SimpleResponse;
}