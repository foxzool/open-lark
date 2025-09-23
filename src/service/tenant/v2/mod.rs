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

    /// 使用共享配置创建 v2 服务集合（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            tenant: tenant::TenantService::new_from_shared(shared.clone()),
            tenant_product_assign_info:
                tenant_product_assign_info::TenantProductAssignInfoService::new_from_shared(shared),
        }
    }
}
