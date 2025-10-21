use serde::{Deserialize, Serialize};

/// 用户ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserIdType {
    /// 用户ID
    #[serde(rename = "user_id")]
    UserId,
    /// union_id
    #[serde(rename = "union_id")]
    UnionId,
    /// open_id
    #[serde(rename = "open_id")]
    OpenId,
}

impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
            UserIdType::OpenId => "open_id",
        }
    }
}

/// 部门ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DepartmentIdType {
    /// 部门ID
    #[serde(rename = "department_id")]
    DepartmentId,
    /// open_department_id
    #[serde(rename = "open_department_id")]
    OpenDepartmentId,
}

impl DepartmentIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DepartmentIdType::DepartmentId => "department_id",
            DepartmentIdType::OpenDepartmentId => "open_department_id",
        }
    }
}

/// 审批状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    /// 审批中
    #[serde(rename = "PENDING")]
    Pending,
    /// 通过
    #[serde(rename = "APPROVED")]
    Approved,
    /// 拒绝
    #[serde(rename = "REJECTED")]
    Rejected,
    /// 撤回
    #[serde(rename = "CANCELED")]
    Canceled,
    /// 已删除
    #[serde(rename = "DELETED")]
    Deleted,
}

/// 任务状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// 待处理
    #[serde(rename = "PENDING")]
    Pending,
    /// 已通过
    #[serde(rename = "APPROVED")]
    Approved,
    /// 已拒绝
    #[serde(rename = "REJECTED")]
    Rejected,
    /// 已转交
    #[serde(rename = "TRANSFERRED")]
    Transferred,
    /// 已完成
    #[serde(rename = "DONE")]
    Done,
}

/// 审批定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    /// 审批定义编码
    pub approval_code: String,
    /// 审批定义名称
    pub approval_name: String,
    /// 审批定义描述
    pub description: Option<String>,
    /// 审批定义状态
    pub status: Option<String>,
    /// 创建者
    pub creator: Option<UserInfo>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 表单字段
    pub form: Option<Vec<FormField>>,
    /// 审批流程
    pub process: Option<ApprovalProcess>,
}

/// 审批实例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalInstance {
    /// 审批实例编码
    pub instance_code: String,
    /// 审批定义编码
    pub approval_code: String,
    /// 审批定义名称
    pub approval_name: Option<String>,
    /// 发起人
    pub initiator: Option<UserInfo>,
    /// 审批状态
    pub status: ApprovalStatus,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 表单数据
    pub form: Option<Vec<FormData>>,
    /// 审批流程
    pub timeline: Option<Vec<ApprovalNode>>,
    /// 抄送人
    pub cc_users: Option<Vec<UserInfo>>,
}

/// 审批任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalTask {
    /// 任务ID
    pub task_id: String,
    /// 审批实例编码
    pub instance_code: String,
    /// 审批定义编码
    pub approval_code: String,
    /// 审批定义名称
    pub approval_name: Option<String>,
    /// 发起人
    pub initiator: Option<UserInfo>,
    /// 审批人
    pub approver: Option<UserInfo>,
    /// 任务状态
    pub status: TaskStatus,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 任务链接
    pub task_links: Option<Vec<TaskLink>>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: Option<String>,
    /// 用户头像
    pub avatar: Option<String>,
    /// 用户邮箱
    pub email: Option<String>,
}

/// 表单字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    /// 字段ID
    pub id: String,
    /// 字段名称
    pub name: String,
    /// 字段类型
    pub field_type: String,
    /// 是否必填
    pub required: Option<bool>,
    /// 字段属性
    pub properties: Option<serde_json::Value>,
}

/// 表单数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormData {
    /// 字段ID
    pub id: String,
    /// 字段名称
    pub name: Option<String>,
    /// 字段值
    pub value: Option<serde_json::Value>,
}

/// 审批流程
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalProcess {
    /// 流程节点
    pub nodes: Vec<ProcessNode>,
}

/// 流程节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessNode {
    /// 节点ID
    pub node_id: String,
    /// 节点名称
    pub node_name: Option<String>,
    /// 节点类型
    pub node_type: String,
    /// 审批人
    pub approvers: Option<Vec<UserInfo>>,
}

/// 审批节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalNode {
    /// 节点ID
    pub node_id: String,
    /// 节点名称
    pub node_name: Option<String>,
    /// 节点类型
    pub node_type: String,
    /// 审批人
    pub approver: Option<UserInfo>,
    /// 审批状态
    pub status: Option<String>,
    /// 审批时间
    pub approve_time: Option<String>,
    /// 审批意见
    pub comment: Option<String>,
}

/// 任务链接
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLink {
    /// 链接平台
    pub platform: String,
    /// 链接地址
    pub link: String,
}

/// 审批文件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalFile {
    /// 文件ID
    pub file_id: String,
    /// 文件名
    pub filename: String,
    /// 文件大小
    pub file_size: Option<i64>,
    /// 文件类型
    pub file_type: Option<String>,
    /// 上传时间
    pub upload_time: Option<String>,
}

/// 审批评论
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalComment {
    /// 评论ID
    pub comment_id: String,
    /// 评论内容
    pub content: String,
    /// 评论者
    pub commenter: Option<UserInfo>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 附件
    pub attachments: Option<Vec<CommentAttachment>>,
}

/// 评论附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentAttachment {
    /// 附件ID
    pub attachment_id: String,
    /// 附件名称
    pub name: String,
    /// 附件类型
    pub attachment_type: String,
    /// 附件链接
    pub link: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_type_serialization() {
        let user_id_type = UserIdType::UserId;
        let serialized = serde_json::to_string(&user_id_type).unwrap();
        assert_eq!(serialized, "\"user_id\"");

        let deserialized: UserIdType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(user_id_type, deserialized);
    }

    #[test]
    fn test_user_id_type_as_str() {
        assert_eq!(UserIdType::UserId.as_str(), "user_id");
        assert_eq!(UserIdType::UnionId.as_str(), "union_id");
        assert_eq!(UserIdType::OpenId.as_str(), "open_id");
    }

    #[test]
    fn test_department_id_type_serialization() {
        let dept_id_type = DepartmentIdType::DepartmentId;
        let serialized = serde_json::to_string(&dept_id_type).unwrap();
        assert_eq!(serialized, "\"department_id\"");

        let deserialized: DepartmentIdType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(dept_id_type, deserialized);
    }

    #[test]
    fn test_department_id_type_as_str() {
        assert_eq!(DepartmentIdType::DepartmentId.as_str(), "department_id");
        assert_eq!(
            DepartmentIdType::OpenDepartmentId.as_str(),
            "open_department_id"
        );
    }

    #[test]
    fn test_approval_status_serialization() {
        let status = ApprovalStatus::Pending;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"PENDING\"");

        let deserialized: ApprovalStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_task_status_serialization() {
        let status = TaskStatus::Approved;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"APPROVED\"");

        let deserialized: TaskStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_approval_serialization() {
        let approval = Approval {
            approval_code: "LEAVE_001".to_string(),
            approval_name: "请假申请".to_string(),
            description: Some("员工请假审批流程".to_string()),
            status: Some("ACTIVE".to_string()),
            creator: Some(UserInfo {
                user_id: "user_123".to_string(),
                name: Some("张三".to_string()),
                avatar: Some("https://example.com/avatar.jpg".to_string()),
                email: Some("zhangsan@example.com".to_string()),
            }),
            create_time: Some("2024-01-01T00:00:00Z".to_string()),
            update_time: Some("2024-01-02T00:00:00Z".to_string()),
            form: Some(vec![FormField {
                id: "field_001".to_string(),
                name: "请假类型".to_string(),
                field_type: "select".to_string(),
                required: Some(true),
                properties: Some(serde_json::json!({"options": ["年假", "病假"]})),
            }]),
            process: Some(ApprovalProcess {
                nodes: vec![ProcessNode {
                    node_id: "node_001".to_string(),
                    node_name: Some("部门经理审批".to_string()),
                    node_type: "approval".to_string(),
                    approvers: Some(vec![UserInfo {
                        user_id: "manager_001".to_string(),
                        name: Some("李四".to_string()),
                        avatar: None,
                        email: Some("lisi@example.com".to_string()),
                    }]),
                }],
            }),
        };

        let serialized = serde_json::to_string(&approval).unwrap();
        let deserialized: Approval = serde_json::from_str(&serialized).unwrap();

        assert_eq!(approval.approval_code, deserialized.approval_code);
        assert_eq!(approval.approval_name, deserialized.approval_name);
        assert_eq!(approval.description, deserialized.description);
        assert_eq!(
            approval.creator.as_ref().unwrap().user_id,
            deserialized.creator.as_ref().unwrap().user_id
        );
        assert_eq!(
            approval.form.as_ref().unwrap().len(),
            deserialized.form.as_ref().unwrap().len()
        );
    }

    #[test]
    fn test_approval_instance_serialization() {
        let instance = ApprovalInstance {
            instance_code: "INST_001".to_string(),
            approval_code: "LEAVE_001".to_string(),
            approval_name: Some("请假申请".to_string()),
            initiator: Some(UserInfo {
                user_id: "user_123".to_string(),
                name: Some("王五".to_string()),
                avatar: None,
                email: Some("wangwu@example.com".to_string()),
            }),
            status: ApprovalStatus::Pending,
            create_time: Some("2024-01-01T10:00:00Z".to_string()),
            update_time: Some("2024-01-01T10:30:00Z".to_string()),
            form: Some(vec![FormData {
                id: "field_001".to_string(),
                name: Some("请假类型".to_string()),
                value: Some(serde_json::json!("年假")),
            }]),
            timeline: Some(vec![ApprovalNode {
                node_id: "node_001".to_string(),
                node_name: Some("部门经理审批".to_string()),
                node_type: "approval".to_string(),
                approver: Some(UserInfo {
                    user_id: "manager_001".to_string(),
                    name: Some("李四".to_string()),
                    avatar: None,
                    email: Some("lisi@example.com".to_string()),
                }),
                status: Some("PENDING".to_string()),
                approve_time: None,
                comment: None,
            }]),
            cc_users: Some(vec![UserInfo {
                user_id: "hr_001".to_string(),
                name: Some("HR小王".to_string()),
                avatar: None,
                email: Some("hr@example.com".to_string()),
            }]),
        };

        let serialized = serde_json::to_string(&instance).unwrap();
        let deserialized: ApprovalInstance = serde_json::from_str(&serialized).unwrap();

        assert_eq!(instance.instance_code, deserialized.instance_code);
        assert_eq!(instance.approval_code, deserialized.approval_code);
        assert_eq!(instance.status, deserialized.status);
        assert_eq!(
            instance.initiator.as_ref().unwrap().user_id,
            deserialized.initiator.as_ref().unwrap().user_id
        );
        assert_eq!(
            instance.timeline.as_ref().unwrap().len(),
            deserialized.timeline.as_ref().unwrap().len()
        );
    }

    #[test]
    fn test_approval_task_serialization() {
        let task = ApprovalTask {
            task_id: "TASK_001".to_string(),
            instance_code: "INST_001".to_string(),
            approval_code: "LEAVE_001".to_string(),
            approval_name: Some("请假申请".to_string()),
            initiator: Some(UserInfo {
                user_id: "user_123".to_string(),
                name: Some("王五".to_string()),
                avatar: None,
                email: Some("wangwu@example.com".to_string()),
            }),
            approver: Some(UserInfo {
                user_id: "manager_001".to_string(),
                name: Some("李四".to_string()),
                avatar: None,
                email: Some("lisi@example.com".to_string()),
            }),
            status: TaskStatus::Pending,
            create_time: Some("2024-01-01T10:00:00Z".to_string()),
            update_time: Some("2024-01-01T10:30:00Z".to_string()),
            task_links: Some(vec![TaskLink {
                platform: "web".to_string(),
                link: "https://approval.example.com/task/TASK_001".to_string(),
            }]),
        };

        let serialized = serde_json::to_string(&task).unwrap();
        let deserialized: ApprovalTask = serde_json::from_str(&serialized).unwrap();

        assert_eq!(task.task_id, deserialized.task_id);
        assert_eq!(task.instance_code, deserialized.instance_code);
        assert_eq!(task.status, deserialized.status);
        assert_eq!(
            task.approver.as_ref().unwrap().user_id,
            deserialized.approver.as_ref().unwrap().user_id
        );
        assert_eq!(
            task.task_links.as_ref().unwrap().len(),
            deserialized.task_links.as_ref().unwrap().len()
        );
    }

    #[test]
    fn test_user_info_serialization() {
        let user = UserInfo {
            user_id: "user_123".to_string(),
            name: Some("测试用户".to_string()),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
            email: Some("test@example.com".to_string()),
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: UserInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(user.user_id, deserialized.user_id);
        assert_eq!(user.name, deserialized.name);
        assert_eq!(user.avatar, deserialized.avatar);
        assert_eq!(user.email, deserialized.email);
    }

    #[test]
    fn test_form_field_serialization() {
        let field = FormField {
            id: "field_001".to_string(),
            name: "员工姓名".to_string(),
            field_type: "text".to_string(),
            required: Some(true),
            properties: Some(serde_json::json!({"max_length": 50})),
        };

        let serialized = serde_json::to_string(&field).unwrap();
        let deserialized: FormField = serde_json::from_str(&serialized).unwrap();

        assert_eq!(field.id, deserialized.id);
        assert_eq!(field.name, deserialized.name);
        assert_eq!(field.field_type, deserialized.field_type);
        assert_eq!(field.required, deserialized.required);
    }

    #[test]
    fn test_form_data_serialization() {
        let data = FormData {
            id: "field_001".to_string(),
            name: Some("员工姓名".to_string()),
            value: Some(serde_json::json!("张三")),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: FormData = serde_json::from_str(&serialized).unwrap();

        assert_eq!(data.id, deserialized.id);
        assert_eq!(data.name, deserialized.name);
        assert_eq!(data.value, deserialized.value);
    }

    #[test]
    fn test_approval_file_serialization() {
        let file = ApprovalFile {
            file_id: "FILE_001".to_string(),
            filename: "请假申请.pdf".to_string(),
            file_size: Some(1024000),
            file_type: Some("application/pdf".to_string()),
            upload_time: Some("2024-01-01T10:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&file).unwrap();
        let deserialized: ApprovalFile = serde_json::from_str(&serialized).unwrap();

        assert_eq!(file.file_id, deserialized.file_id);
        assert_eq!(file.filename, deserialized.filename);
        assert_eq!(file.file_size, deserialized.file_size);
        assert_eq!(file.file_type, deserialized.file_type);
    }

    #[test]
    fn test_approval_comment_serialization() {
        let comment = ApprovalComment {
            comment_id: "COMMENT_001".to_string(),
            content: "请尽快审批，谢谢！".to_string(),
            commenter: Some(UserInfo {
                user_id: "user_123".to_string(),
                name: Some("王五".to_string()),
                avatar: None,
                email: Some("wangwu@example.com".to_string()),
            }),
            create_time: Some("2024-01-01T10:00:00Z".to_string()),
            update_time: Some("2024-01-01T10:00:00Z".to_string()),
            attachments: Some(vec![CommentAttachment {
                attachment_id: "ATTACH_001".to_string(),
                name: "病假证明.jpg".to_string(),
                attachment_type: "image".to_string(),
                link: Some("https://example.com/attach/001.jpg".to_string()),
            }]),
        };

        let serialized = serde_json::to_string(&comment).unwrap();
        let deserialized: ApprovalComment = serde_json::from_str(&serialized).unwrap();

        assert_eq!(comment.comment_id, deserialized.comment_id);
        assert_eq!(comment.content, deserialized.content);
        assert_eq!(
            comment.commenter.as_ref().unwrap().user_id,
            deserialized.commenter.as_ref().unwrap().user_id
        );
        assert_eq!(
            comment.attachments.as_ref().unwrap().len(),
            deserialized.attachments.as_ref().unwrap().len()
        );
    }

    #[test]
    fn test_comment_attachment_serialization() {
        let attachment = CommentAttachment {
            attachment_id: "ATTACH_001".to_string(),
            name: "附件文件.doc".to_string(),
            attachment_type: "document".to_string(),
            link: Some("https://example.com/files/attach_001.doc".to_string()),
        };

        let serialized = serde_json::to_string(&attachment).unwrap();
        let deserialized: CommentAttachment = serde_json::from_str(&serialized).unwrap();

        assert_eq!(attachment.attachment_id, deserialized.attachment_id);
        assert_eq!(attachment.name, deserialized.name);
        assert_eq!(attachment.attachment_type, deserialized.attachment_type);
        assert_eq!(attachment.link, deserialized.link);
    }

    #[test]
    fn test_models_with_none_values() {
        let user = UserInfo {
            user_id: "user_123".to_string(),
            name: None,
            avatar: None,
            email: None,
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: UserInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(user.user_id, deserialized.user_id);
        assert!(deserialized.name.is_none());
        assert!(deserialized.avatar.is_none());
        assert!(deserialized.email.is_none());
    }

    #[test]
    fn test_debug_trait_for_models() {
        let status = ApprovalStatus::Approved;
        let debug_string = format!("{:?}", status);
        assert!(debug_string.contains("Approved"));

        let user = UserInfo {
            user_id: "test_user".to_string(),
            name: Some("测试".to_string()),
            avatar: None,
            email: None,
        };
        let debug_string = format!("{:?}", user);
        assert!(debug_string.contains("UserInfo"));
        assert!(debug_string.contains("test_user"));
    }

    #[test]
    fn test_all_approval_statuses() {
        let statuses = vec![
            ApprovalStatus::Pending,
            ApprovalStatus::Approved,
            ApprovalStatus::Rejected,
            ApprovalStatus::Canceled,
            ApprovalStatus::Deleted,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: ApprovalStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_all_task_statuses() {
        let statuses = vec![
            TaskStatus::Pending,
            TaskStatus::Approved,
            TaskStatus::Rejected,
            TaskStatus::Transferred,
            TaskStatus::Done,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: TaskStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_complex_approval_process() {
        let process = ApprovalProcess {
            nodes: vec![
                ProcessNode {
                    node_id: "node_001".to_string(),
                    node_name: Some("部门经理审批".to_string()),
                    node_type: "approval".to_string(),
                    approvers: Some(vec![UserInfo {
                        user_id: "manager_001".to_string(),
                        name: Some("张经理".to_string()),
                        avatar: None,
                        email: Some("manager@example.com".to_string()),
                    }]),
                },
                ProcessNode {
                    node_id: "node_002".to_string(),
                    node_name: Some("HR审批".to_string()),
                    node_type: "approval".to_string(),
                    approvers: Some(vec![UserInfo {
                        user_id: "hr_001".to_string(),
                        name: Some("HR小王".to_string()),
                        avatar: None,
                        email: Some("hr@example.com".to_string()),
                    }]),
                },
            ],
        };

        let serialized = serde_json::to_string(&process).unwrap();
        let deserialized: ApprovalProcess = serde_json::from_str(&serialized).unwrap();

        assert_eq!(process.nodes.len(), deserialized.nodes.len());
        assert_eq!(process.nodes[0].node_id, deserialized.nodes[0].node_id);
        assert_eq!(process.nodes[1].node_id, deserialized.nodes[1].node_id);
    }
}
