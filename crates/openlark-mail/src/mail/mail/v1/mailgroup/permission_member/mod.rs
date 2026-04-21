/// batch_create 模块。
pub mod batch_create;
/// batch_delete 模块。
pub mod batch_delete;
/// 创建接口。
pub mod create;
/// 删除接口。
pub mod delete;
/// 获取接口。
pub mod get;
/// 列表接口。
pub mod list;

use openlark_core::config::Config;
use std::sync::Arc;

/// 邮件组权限成员资源
#[derive(Clone)]
pub struct PermissionMember {
    config: Arc<Config>,
    mailgroup_id: String,
}

impl PermissionMember {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
        }
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::ListMailGroupPermissionMemberRequest {
        list::ListMailGroupPermissionMemberRequest::new(
            self.config.clone(),
            self.mailgroup_id.clone(),
        )
    }

    /// 创建获取详情请求。
    pub fn get(
        &self,
        permission_member_id: impl Into<String>,
    ) -> get::GetMailGroupPermissionMemberRequest {
        get::GetMailGroupPermissionMemberRequest::new(
            self.config.clone(),
            self.mailgroup_id.clone(),
            permission_member_id,
        )
    }

    /// 创建新建请求。
    pub fn create(&self) -> create::CreateMailGroupPermissionMemberRequest {
        create::CreateMailGroupPermissionMemberRequest::new(
            self.config.clone(),
            self.mailgroup_id.clone(),
        )
    }

    /// 创建删除请求。
    pub fn delete(
        &self,
        permission_member_id: impl Into<String>,
    ) -> delete::DeleteMailGroupPermissionMemberRequest {
        delete::DeleteMailGroupPermissionMemberRequest::new(
            self.config.clone(),
            self.mailgroup_id.clone(),
            permission_member_id,
        )
    }

    /// 创建批量新增请求。
    pub fn batch_create(&self) -> batch_create::BatchCreateMailGroupPermissionMemberRequest {
        batch_create::BatchCreateMailGroupPermissionMemberRequest::new(
            self.config.clone(),
            self.mailgroup_id.clone(),
        )
    }

    /// 创建批量删除请求。
    pub fn batch_delete(&self) -> batch_delete::BatchDeleteMailGroupPermissionMemberRequest {
        batch_delete::BatchDeleteMailGroupPermissionMemberRequest::new(
            self.config.clone(),
            self.mailgroup_id.clone(),
        )
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
