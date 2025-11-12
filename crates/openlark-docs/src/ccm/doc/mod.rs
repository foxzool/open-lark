#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// doc - 文档服务
//,
// 提供文档相关的功能
use openlark_core::prelude::*;
use openlark_core::service::ccm::doc::v2::DocV2Service;
/// 文档服务
#[derive(Debug, Clone)]
pub struct DocService {
}

impl DocService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v2版本API
pub mod v2;
}