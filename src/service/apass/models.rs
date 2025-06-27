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
