#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 地址簿服务模块
//!
//! 提供飞书地址簿相关的API功能，包括：
//! - 联系人管理
//! - 分组管理
//! - 地址簿同步

use crate::config::Config;

/// 地址簿服务
#[derive(Debug, Clone)]
pub struct AddressBookService {
    pub config: Config,
    pub v1: v1::AddressBookServiceV1,
}

impl AddressBookService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: v1::AddressBookServiceV1::new(config),
        }
    }
}

pub mod v1;