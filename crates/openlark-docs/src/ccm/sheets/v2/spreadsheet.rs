#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

// sheets v2 spreadsheet - 表格操作API实现,
//,
// 实现表格级别的操作，包括：,
// - 获取表格元数据,
// - 创建表格,
// - 更新表格属性,
// - 删除表格,
// - 获取表格所有工作表,
use openlark_core::{APIResult, LarkClient, RequestBuilder};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
