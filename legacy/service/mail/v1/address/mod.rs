// 邮箱地址查询模块 - 占位符实现
use crate::core::config::Config;

pub struct AddressService {
    pub config: Config,
}

impl AddressService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
