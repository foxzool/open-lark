use serde::{Deserialize, Serialize};

/// 席位分配查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SeatAssignmentListRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 席位分配信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SeatAssignment {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 席位类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_type: Option<String>,
    /// 分配时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_time: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 席位活跃查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SeatActivityListRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

/// 席位活跃信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SeatActivity {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 活跃时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_time: Option<String>,
    /// 活跃度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_level: Option<String>,
}

/// 审计日志列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogListRequest {
    /// 应用ID
    pub app_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 操作类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
}

/// 审计日志信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    /// 日志ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_id: Option<String>,
    /// 操作用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// 操作用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<String>,
    /// 操作类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    /// 操作对象
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_object: Option<String>,
    /// 操作时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_time: Option<String>,
    /// 操作结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_result: Option<String>,
    /// 详细信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// 数据变更日志列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DataChangeLogListRequest {
    /// 应用ID
    pub app_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 对象API名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_api_name: Option<String>,
}

/// 数据变更日志信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DataChangeLog {
    /// 日志ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_id: Option<String>,
    /// 记录ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    /// 对象API名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_api_name: Option<String>,
    /// 变更类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    /// 变更时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    /// 变更用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed_by: Option<String>,
    /// 变更前数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_data: Option<serde_json::Value>,
    /// 变更后数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_data: Option<serde_json::Value>,
}

/// 角色成员授权操作请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleMemberAuthorizationRequest {
    /// 应用ID
    pub app_id: String,
    /// 角色API名称
    pub role_api_name: String,
    /// 用户ID列表
    pub user_ids: Vec<String>,
}

/// 角色成员信息
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleMember {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 角色API名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_api_name: Option<String>,
    /// 授权时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_time: Option<String>,
}

/// 记录权限用户授权操作请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordPermissionMemberAuthorizationRequest {
    /// 应用ID
    pub app_id: String,
    /// 记录权限API名称
    pub record_permission_api_name: String,
    /// 用户ID列表
    pub user_ids: Vec<String>,
    /// 记录ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_ids: Option<Vec<String>>,
}

/// OQL查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct OqlQueryRequest {
    /// 应用ID
    pub app_id: String,
    /// OQL查询语句
    pub oql: String,
    /// 查询参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

/// OQL查询结果
#[derive(Debug, Serialize, Deserialize)]
pub struct OqlQueryResult {
    /// 查询结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<serde_json::Value>>,
    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

/// 记录搜索请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordSearchRequest {
    /// 应用ID
    pub app_id: String,
    /// 对象API名称
    pub object_api_name: String,
    /// 搜索关键字
    pub keyword: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 记录查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordQueryRequest {
    /// 应用ID
    pub app_id: String,
    /// 对象API名称
    pub object_api_name: String,
    /// 记录ID
    pub record_id: String,
    /// 字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}

/// 记录创建请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordCreateRequest {
    /// 应用ID
    pub app_id: String,
    /// 对象API名称
    pub object_api_name: String,
    /// 记录数据
    pub data: serde_json::Value,
}

/// 记录更新请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordUpdateRequest {
    /// 应用ID
    pub app_id: String,
    /// 对象API名称
    pub object_api_name: String,
    /// 记录ID
    pub record_id: String,
    /// 更新数据
    pub data: serde_json::Value,
}

/// 记录删除请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordDeleteRequest {
    /// 应用ID
    pub app_id: String,
    /// 对象API名称
    pub object_api_name: String,
    /// 记录ID
    pub record_id: String,
}

/// 批量记录操作请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRecordRequest {
    /// 应用ID
    pub app_id: String,
    /// 对象API名称
    pub object_api_name: String,
    /// 记录数据列表
    pub records: Vec<serde_json::Value>,
}

/// 批量记录查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRecordQueryRequest {
    /// 应用ID
    pub app_id: String,
    /// 对象API名称
    pub object_api_name: String,
    /// 查询条件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
    /// 排序条件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<String>>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 记录信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    /// 记录ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    /// 记录数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

/// 函数执行请求
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionInvokeRequest {
    /// 应用ID
    pub app_id: String,
    /// 函数API名称
    pub function_api_name: String,
    /// 函数参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

/// 函数执行结果
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionInvokeResult {
    /// 执行结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// 执行状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 环境变量查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentVariableQueryRequest {
    /// 应用ID
    pub app_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 环境变量获取请求
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentVariableGetRequest {
    /// 应用ID
    pub app_id: String,
    /// 变量名称
    pub variable_name: String,
}

/// 环境变量信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    /// 变量名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_name: Option<String>,
    /// 变量值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_value: Option<String>,
    /// 变量描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

/// 流程执行请求
#[derive(Debug, Serialize, Deserialize)]
pub struct FlowExecuteRequest {
    /// 应用ID
    pub app_id: String,
    /// 流程API名称
    pub flow_api_name: String,
    /// 流程参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

/// 流程执行结果
#[derive(Debug, Serialize, Deserialize)]
pub struct FlowExecuteResult {
    /// 流程实例ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 执行状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 结果数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

/// 人工任务查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTaskQueryRequest {
    /// 应用ID
    pub app_id: String,
    /// 任务状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 人工任务操作请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTaskActionRequest {
    /// 应用ID
    pub app_id: String,
    /// 任务ID
    pub task_id: String,
    /// 操作意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 操作数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// 人工任务转交请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTaskTransferRequest {
    /// 应用ID
    pub app_id: String,
    /// 任务ID
    pub task_id: String,
    /// 转交目标用户ID
    pub target_user_id: String,
    /// 转交意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 人工任务加签请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTaskAddAssigneeRequest {
    /// 应用ID
    pub app_id: String,
    /// 任务ID
    pub task_id: String,
    /// 加签用户ID列表
    pub assignee_user_ids: Vec<String>,
    /// 加签意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 人工任务抄送请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTaskCcRequest {
    /// 应用ID
    pub app_id: String,
    /// 任务ID
    pub task_id: String,
    /// 抄送用户ID列表
    pub cc_user_ids: Vec<String>,
    /// 抄送意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 人工任务退回请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTaskRollbackRequest {
    /// 应用ID
    pub app_id: String,
    /// 任务ID
    pub task_id: String,
    /// 退回目标节点ID
    pub target_node_id: String,
    /// 退回意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 人工任务群聊请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTaskChatGroupRequest {
    /// 应用ID
    pub app_id: String,
    /// 任务ID
    pub task_id: String,
    /// 群聊名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// 人工任务信息
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTask {
    /// 任务ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    /// 流程实例ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 任务状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 任务处理人ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 到期时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_time: Option<String>,
    /// 任务数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// 退回位置信息
#[derive(Debug, Serialize, Deserialize)]
pub struct RollbackPoint {
    /// 节点ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 节点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    /// 节点类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
}

/// 通用分页响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<T>>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_seat_assignment_list_request() {
        let request = SeatAssignmentListRequest {
            page_size: Some(100),
            page_token: Some("token123".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("100"));
        assert!(json.contains("token123"));
    }

    #[test]
    fn test_seat_assignment_complete() {
        let assignment = SeatAssignment {
            user_id: Some("user123".to_string()),
            user_name: Some("张三".to_string()),
            seat_type: Some("premium".to_string()),
            assigned_time: Some("2024-01-01T00:00:00Z".to_string()),
            status: Some("active".to_string()),
        };
        let json = serde_json::to_string(&assignment).unwrap();
        assert!(json.contains("user123"));
        assert!(json.contains("张三"));
        assert!(json.contains("premium"));
        assert!(json.contains("active"));
    }

    #[test]
    fn test_seat_activity_list_request() {
        let request = SeatActivityListRequest {
            page_size: Some(50),
            page_token: None,
            start_time: Some("2024-01-01T00:00:00Z".to_string()),
            end_time: Some("2024-01-31T23:59:59Z".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("50"));
        assert!(json.contains("2024-01-01"));
        assert!(json.contains("2024-01-31"));
        assert!(!json.contains("page_token"));
    }

    #[test]
    fn test_seat_activity_detailed() {
        let activity = SeatActivity {
            user_id: Some("user456".to_string()),
            user_name: Some("李四".to_string()),
            seat_type: Some("standard".to_string()),
            activity_time: Some("2024-01-15T10:30:00Z".to_string()),
            activity_type: Some("login".to_string()),
            ip_address: Some("192.168.1.100".to_string()),
            device_info: Some("MacBook Pro".to_string()),
        };
        let json = serde_json::to_string(&activity).unwrap();
        assert!(json.contains("user456"));
        assert!(json.contains("李四"));
        assert!(json.contains("login"));
        assert!(json.contains("192.168.1.100"));
        assert!(json.contains("MacBook Pro"));
    }

    #[test]
    fn test_audit_log_list_request() {
        let request = AuditLogListRequest {
            page_size: Some(200),
            page_token: Some("audit_token".to_string()),
            start_time: Some("2024-01-01T00:00:00Z".to_string()),
            end_time: Some("2024-01-07T23:59:59Z".to_string()),
            user_id: Some("admin123".to_string()),
            action_type: Some("user_create".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("200"));
        assert!(json.contains("audit_token"));
        assert!(json.contains("admin123"));
        assert!(json.contains("user_create"));
    }

    #[test]
    fn test_audit_log_security_event() {
        let log = AuditLog {
            log_id: Some("log789".to_string()),
            timestamp: Some("2024-01-15T14:20:00Z".to_string()),
            user_id: Some("user789".to_string()),
            user_name: Some("王五".to_string()),
            action_type: Some("password_reset".to_string()),
            resource_type: Some("user_account".to_string()),
            resource_id: Some("acc456".to_string()),
            action_result: Some("success".to_string()),
            ip_address: Some("10.0.0.50".to_string()),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            details: Some(serde_json::json!({
                "reason": "forgot_password",
                "method": "email_verification"
            })),
        };
        let json = serde_json::to_string(&log).unwrap();
        assert!(json.contains("log789"));
        assert!(json.contains("password_reset"));
        assert!(json.contains("success"));
        assert!(json.contains("forgot_password"));
    }

    #[test]
    fn test_data_change_log_list_request() {
        let request = DataChangeLogListRequest {
            page_size: Some(75),
            page_token: None,
            start_time: Some("2024-01-10T00:00:00Z".to_string()),
            end_time: Some("2024-01-20T23:59:59Z".to_string()),
            data_type: Some("user_profile".to_string()),
            change_type: Some("update".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("75"));
        assert!(json.contains("user_profile"));
        assert!(json.contains("update"));
    }

    #[test]
    fn test_data_change_log_detailed() {
        let log = DataChangeLog {
            change_id: Some("change123".to_string()),
            timestamp: Some("2024-01-15T16:45:00Z".to_string()),
            user_id: Some("admin456".to_string()),
            data_type: Some("department".to_string()),
            data_id: Some("dept789".to_string()),
            change_type: Some("create".to_string()),
            old_value: None,
            new_value: Some(serde_json::json!({
                "name": "新技术部",
                "parent_id": "company",
                "status": "active"
            })),
            change_reason: Some("组织架构调整".to_string()),
        };
        let json = serde_json::to_string(&log).unwrap();
        assert!(json.contains("change123"));
        assert!(json.contains("department"));
        assert!(json.contains("create"));
        assert!(json.contains("新技术部"));
        assert!(json.contains("组织架构调整"));
    }

    #[test]
    fn test_role_member_authorization_request() {
        let request = RoleMemberAuthorizationRequest {
            role_id: "role123".to_string(),
            user_id: "user789".to_string(),
            operation: "grant".to_string(),
            scope: Some("department:tech".to_string()),
            reason: Some("职位调整".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("role123"));
        assert!(json.contains("user789"));
        assert!(json.contains("grant"));
        assert!(json.contains("department:tech"));
        assert!(json.contains("职位调整"));
    }

    #[test]
    fn test_role_member_complete() {
        let member = RoleMember {
            user_id: Some("user654".to_string()),
            user_name: Some("赵六".to_string()),
            role_id: Some("role456".to_string()),
            role_name: Some("部门管理员".to_string()),
            granted_time: Some("2024-01-01T09:00:00Z".to_string()),
            granted_by: Some("admin001".to_string()),
            scope: Some("department:sales".to_string()),
            status: Some("active".to_string()),
            expires_at: Some("2024-12-31T23:59:59Z".to_string()),
        };
        let json = serde_json::to_string(&member).unwrap();
        assert!(json.contains("user654"));
        assert!(json.contains("赵六"));
        assert!(json.contains("部门管理员"));
        assert!(json.contains("department:sales"));
        assert!(json.contains("2024-12-31"));
    }

    #[test]
    fn test_user_permission_query_request() {
        let request = UserPermissionQueryRequest {
            user_id: "user888".to_string(),
            resource_type: Some("document".to_string()),
            resource_id: Some("doc123".to_string()),
            permission_type: Some("read".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("user888"));
        assert!(json.contains("document"));
        assert!(json.contains("doc123"));
        assert!(json.contains("read"));
    }

    #[test]
    fn test_user_permission_granted() {
        let permission = UserPermission {
            user_id: Some("user999".to_string()),
            resource_type: Some("project".to_string()),
            resource_id: Some("proj456".to_string()),
            permission_type: Some("write".to_string()),
            granted: Some(true),
            granted_by_role: Some("project_admin".to_string()),
            granted_time: Some("2024-01-10T14:00:00Z".to_string()),
            expires_at: None,
        };
        let json = serde_json::to_string(&permission).unwrap();
        assert!(json.contains("user999"));
        assert!(json.contains("project"));
        assert!(json.contains("write"));
        assert!(json.contains("true"));
        assert!(json.contains("project_admin"));
    }

    #[test]
    fn test_access_token_create_request() {
        let request = AccessTokenCreateRequest {
            app_id: "app123".to_string(),
            user_id: Some("user456".to_string()),
            scope: vec!["read:user".to_string(), "write:document".to_string()],
            expires_in: Some(3600),
            description: Some("API访问令牌".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("app123"));
        assert!(json.contains("user456"));
        assert!(json.contains("read:user"));
        assert!(json.contains("write:document"));
        assert!(json.contains("3600"));
        assert!(json.contains("API访问令牌"));
    }

    #[test]
    fn test_access_token_active() {
        let token = AccessToken {
            token_id: Some("token789".to_string()),
            app_id: Some("app456".to_string()),
            user_id: Some("user123".to_string()),
            scope: Some(vec!["read:profile".to_string()]),
            created_at: Some("2024-01-01T10:00:00Z".to_string()),
            expires_at: Some("2024-01-01T11:00:00Z".to_string()),
            status: Some("active".to_string()),
            last_used_at: Some("2024-01-01T10:30:00Z".to_string()),
            usage_count: Some(5),
        };
        let json = serde_json::to_string(&token).unwrap();
        assert!(json.contains("token789"));
        assert!(json.contains("app456"));
        assert!(json.contains("read:profile"));
        assert!(json.contains("active"));
        assert!(json.contains("\"usage_count\":5"));
    }

    #[test]
    fn test_app_authorization_request() {
        let request = AppAuthorizationRequest {
            app_id: "app789".to_string(),
            user_id: "user321".to_string(),
            requested_scope: vec!["read:contact".to_string(), "write:calendar".to_string()],
            redirect_uri: Some("https://app.example.com/callback".to_string()),
            state: Some("random_state_123".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("app789"));
        assert!(json.contains("user321"));
        assert!(json.contains("read:contact"));
        assert!(json.contains("write:calendar"));
        assert!(json.contains("callback"));
        assert!(json.contains("random_state_123"));
    }

    #[test]
    fn test_app_authorization_approved() {
        let auth = AppAuthorization {
            authorization_id: Some("auth456".to_string()),
            app_id: Some("app654".to_string()),
            user_id: Some("user987".to_string()),
            granted_scope: Some(vec!["read:user".to_string(), "read:email".to_string()]),
            authorization_code: Some("code_abc123".to_string()),
            status: Some("approved".to_string()),
            created_at: Some("2024-01-15T12:00:00Z".to_string()),
            expires_at: Some("2024-01-15T12:10:00Z".to_string()),
        };
        let json = serde_json::to_string(&auth).unwrap();
        assert!(json.contains("auth456"));
        assert!(json.contains("app654"));
        assert!(json.contains("approved"));
        assert!(json.contains("code_abc123"));
        assert!(json.contains("read:email"));
    }

    #[test]
    fn test_page_response_generic() {
        let response: PageResponse<String> = PageResponse {
            items: Some(vec!["item1".to_string(), "item2".to_string()]),
            has_more: Some(true),
            page_token: Some("next_token".to_string()),
            total_count: Some(250),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("item1"));
        assert!(json.contains("true"));
        assert!(json.contains("next_token"));
        assert!(json.contains("250"));
    }
}
