//! 自定义角色服务测试模块

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    #[test]
    fn test_role_models() {
        // 测试角色模型创建
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        assert_eq!(config.app_id(), "test_app_id");
        assert_eq!(config.app_secret(), "test_app_secret");
    }

    #[test]
    fn test_table_role_creation() {
        let table_role = super::super::models::TableRole::new(1);
        assert_eq!(table_role.table_perm, 1);
        assert!(table_role.table_name.is_none());
        assert!(table_role.table_id.is_none());
    }

    #[test]
    fn test_role_struct_creation() {
        use super::super::models::{Role, TableRole};

        let role = Role {
            role_name: "测试角色".to_string(),
            role_id: None,
            table_roles: vec![TableRole::new(1)],
            block_roles: None,
        };

        assert_eq!(role.role_name, "测试角色");
        assert_eq!(role.table_roles.len(), 1);
    }
}
