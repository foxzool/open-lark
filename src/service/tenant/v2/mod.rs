pub mod tenant;
pub mod tenant_product_assign_info;

use crate::core::config::Config;

/// Tenant API v2版本服务
pub struct V2 {
    /// 企业信息服务
    pub tenant: tenant::TenantService,
    /// 企业席位信息服务
    pub tenant_product_assign_info: tenant_product_assign_info::TenantProductAssignInfoService,
}

impl V2 {
    pub fn new(config: Config) -> Self {
        Self {
            tenant: tenant::TenantService::new(config.clone()),
            tenant_product_assign_info:
                tenant_product_assign_info::TenantProductAssignInfoService::new(config),
        }
    }
}
