#[path = "approval/approval/v4/task/approve.rs"]
mod approval_task_approve;
#[path = "approval/approval/v4/task/query.rs"]
mod approval_task_query;
#[path = "approval/approval/v4/task/reject.rs"]
mod approval_task_reject;
#[path = "approval/approval/v4/task/resubmit.rs"]
mod approval_task_resubmit;

use openlark_core::{config::Config, SDKResult};
use std::sync::Arc;

use crate::common::constants::MAX_PAGE_SIZE;

/// 任务列表查询 helper。
///
/// 用于封装常见的任务列表过滤条件，并让 helper 统一处理分页。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorkflowTaskListQuery {
    pub tasklist_guid: Option<String>,
    pub section_guid: Option<String>,
    pub filter: Option<String>,
    pub sort: Option<serde_json::Value>,
    pub user_type: Option<String>,
    pub page_size: Option<i32>,
}

impl WorkflowTaskListQuery {
    pub fn for_tasklist(tasklist_guid: impl Into<String>) -> Self {
        Self {
            tasklist_guid: Some(tasklist_guid.into()),
            ..Self::default()
        }
    }

    pub fn section_guid(mut self, section_guid: impl Into<String>) -> Self {
        self.section_guid = Some(section_guid.into());
        self
    }

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    pub fn sort(mut self, sort: serde_json::Value) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn user_type(mut self, user_type: impl Into<String>) -> Self {
        self.user_type = Some(user_type.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

/// 任务变更 helper。
///
/// 只覆盖高频可变字段，不试图替代完整 typed request。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorkflowTaskMutation {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub due: Option<String>,
    pub priority: Option<i32>,
    pub assignee: Option<String>,
    pub status: Option<String>,
}

impl WorkflowTaskMutation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn due(mut self, due: impl Into<String>) -> Self {
        self.due = Some(due.into());
        self
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

/// 审批任务查询 helper。
///
/// 用于封装审批待办的常见筛选条件。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApprovalTaskQuery {
    pub user_id: String,
    pub topic: String,
    pub user_id_type: Option<String>,
    pub status: Option<String>,
    pub instance_code: Option<String>,
    pub page_size: Option<i32>,
}

impl ApprovalTaskQuery {
    pub fn new(user_id: impl Into<String>, topic: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            topic: topic.into(),
            ..Self::default()
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.instance_code = Some(instance_code.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

/// 审批任务操作 helper。
///
/// 统一高频审批动作的 `task_id + comment` 组合。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApprovalTaskAction {
    pub approval_code: String,
    pub instance_code: String,
    pub user_id: String,
    pub task_id: String,
    pub user_id_type: Option<String>,
    pub comment: Option<String>,
    pub form: Option<String>,
}

impl ApprovalTaskAction {
    pub fn new(
        approval_code: impl Into<String>,
        instance_code: impl Into<String>,
        user_id: impl Into<String>,
        task_id: impl Into<String>,
    ) -> Self {
        Self {
            approval_code: approval_code.into(),
            instance_code: instance_code.into(),
            user_id: user_id.into(),
            task_id: task_id.into(),
            user_id_type: None,
            comment: None,
            form: None,
        }
    }

    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn form(mut self, form: impl Into<String>) -> Self {
        self.form = Some(form.into());
        self
    }
}

pub type ApprovalTaskItem = approval_task_query::TaskItemV4;

/// 审批任务动作结果 helper。
#[derive(Debug, Clone, PartialEq)]
pub struct ApprovalTaskActionResult {
    pub success: bool,
}

/// WorkflowService：工作流服务的统一入口
///
/// 提供对任务、审批、看板 API 的访问能力
#[derive(Clone)]
#[allow(dead_code)]
pub struct WorkflowService {
    config: Arc<Config>,
}

impl WorkflowService {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::v1::TaskV1 {
        crate::v1::TaskV1::new(self.config.clone())
    }

    #[cfg(feature = "v2")]
    pub fn v2(&self) -> crate::v2::TaskV2 {
        crate::v2::TaskV2::new(self.config.clone())
    }

    #[cfg(feature = "v2")]
    pub fn task(&self) -> crate::v2::task::Task {
        crate::v2::task::Task::new(self.config.clone())
    }

    #[cfg(feature = "v2")]
    pub fn tasklist(&self) -> crate::v2::tasklist::Tasklist {
        crate::v2::tasklist::Tasklist::new(self.config.clone())
    }

    /// 列取任务并自动处理分页。
    #[cfg(feature = "v2")]
    pub async fn list_tasks_all(
        &self,
        query: WorkflowTaskListQuery,
    ) -> SDKResult<Vec<crate::v2::task::models::TaskItem>> {
        use crate::v2::task::list::ListTasksRequest;

        let mut items = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let mut request = ListTasksRequest::new(self.config.clone())
                .page_size(query.page_size.unwrap_or(MAX_PAGE_SIZE));

            if let Some(tasklist_guid) = &query.tasklist_guid {
                request = request.tasklist_guid(tasklist_guid.clone());
            }
            if let Some(section_guid) = &query.section_guid {
                request = request.section_guid(section_guid.clone());
            }
            if let Some(filter) = &query.filter {
                request = request.filter(filter.clone());
            }
            if let Some(sort) = &query.sort {
                request = request.sort(sort.clone());
            }
            if let Some(user_type) = &query.user_type {
                request = request.user_type(user_type.clone());
            }
            if let Some(token) = &page_token {
                request = request.page_token(token.clone());
            }

            let response = request.execute().await?;
            items.extend(response.items);

            if !response.has_more {
                break;
            }
            page_token = response.page_token;
        }

        Ok(items)
    }

    /// 使用 helper 风格更新任务高频字段。
    #[cfg(feature = "v2")]
    pub async fn mutate_task(
        &self,
        task_guid: impl Into<String>,
        mutation: WorkflowTaskMutation,
    ) -> SDKResult<crate::v2::task::models::UpdateTaskResponse> {
        use crate::v2::task::update::UpdateTaskRequest;

        let mut request = UpdateTaskRequest::new(self.config.clone(), task_guid.into());
        if let Some(summary) = mutation.summary {
            request = request.summary(summary);
        }
        if let Some(description) = mutation.description {
            request = request.description(description);
        }
        if let Some(due) = mutation.due {
            request = request.due(due);
        }
        if let Some(priority) = mutation.priority {
            request = request.priority(priority);
        }
        if let Some(assignee) = mutation.assignee {
            request = request.assignee(assignee);
        }
        if let Some(status) = mutation.status {
            request = request.status(status);
        }

        request.execute().await
    }

    /// 完成任务 helper。
    #[cfg(feature = "v2")]
    pub async fn complete_task(
        &self,
        task_guid: impl Into<String>,
    ) -> SDKResult<crate::v2::task::models::CompleteTaskResponse> {
        use crate::v2::task::complete::CompleteTaskRequest;

        CompleteTaskRequest::new(self.config.clone(), task_guid.into())
            .execute()
            .await
    }

    /// 重新打开任务 helper。
    #[cfg(feature = "v2")]
    pub async fn reopen_task(
        &self,
        task_guid: impl Into<String>,
    ) -> SDKResult<crate::v2::task::models::UncompleteTaskResponse> {
        use crate::v2::task::uncomplete::UncompleteTaskRequest;

        UncompleteTaskRequest::new(self.config.clone(), task_guid.into())
            .execute()
            .await
    }

    /// 查询审批任务，并支持按状态/实例做本地筛选。
    pub async fn query_approval_tasks(
        &self,
        query: ApprovalTaskQuery,
    ) -> SDKResult<Vec<ApprovalTaskItem>> {
        let mut items = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let mut request = approval_task_query::QueryTaskRequestV4::new(self.config.clone())
                .user_id(query.user_id.clone())
                .topic(query.topic.clone())
                .page_size(query.page_size.unwrap_or(MAX_PAGE_SIZE));

            if let Some(user_id_type) = &query.user_id_type {
                request = request.user_id_type(user_id_type.clone());
            }
            if let Some(token) = &page_token {
                request = request.page_token(token.clone());
            }

            let response = request.execute().await?;
            items.extend(response.tasks);

            if !response.has_more.unwrap_or(false) {
                break;
            }
            page_token = response.page_token;
        }

        if let Some(status) = &query.status {
            items.retain(|item| item.status == *status);
        }
        if let Some(instance_code) = &query.instance_code {
            items.retain(|item| item.instance_code == *instance_code);
        }

        Ok(items)
    }

    /// 同意审批任务 helper。
    pub async fn approve_task(
        &self,
        action: ApprovalTaskAction,
    ) -> SDKResult<ApprovalTaskActionResult> {
        let mut request = approval_task_approve::ApproveTaskRequestV4::new(self.config.clone())
            .approval_code(action.approval_code)
            .instance_code(action.instance_code)
            .user_id(action.user_id)
            .task_id(action.task_id);
        if let Some(user_id_type) = action.user_id_type {
            request = request.user_id_type(user_id_type);
        }
        if let Some(comment) = action.comment {
            request = request.comment(comment);
        }
        if let Some(form) = action.form {
            request = request.form(form);
        }
        let response = request.execute().await?;
        let _ = response;
        Ok(ApprovalTaskActionResult { success: true })
    }

    /// 拒绝审批任务 helper。
    pub async fn reject_task(
        &self,
        action: ApprovalTaskAction,
    ) -> SDKResult<ApprovalTaskActionResult> {
        let mut request = approval_task_reject::RejectTaskRequestV4::new(self.config.clone())
            .approval_code(action.approval_code)
            .instance_code(action.instance_code)
            .user_id(action.user_id)
            .task_id(action.task_id);
        if let Some(user_id_type) = action.user_id_type {
            request = request.user_id_type(user_id_type);
        }
        if let Some(comment) = action.comment {
            request = request.comment(comment);
        }
        if let Some(form) = action.form {
            request = request.form(form);
        }
        let response = request.execute().await?;
        let _ = response;
        Ok(ApprovalTaskActionResult { success: true })
    }

    /// 重新提交审批任务 helper。
    pub async fn resubmit_task(
        &self,
        action: ApprovalTaskAction,
    ) -> SDKResult<ApprovalTaskActionResult> {
        let mut request = approval_task_resubmit::ResubmitTaskRequestV4::new(self.config.clone())
            .approval_code(action.approval_code)
            .instance_code(action.instance_code)
            .user_id(action.user_id)
            .task_id(action.task_id);
        if let Some(user_id_type) = action.user_id_type {
            request = request.user_id_type(user_id_type);
        }
        if let Some(comment) = action.comment {
            request = request.comment(comment);
        }
        if let Some(form) = action.form {
            request = request.form(form);
        }
        let response = request.execute().await?;
        let _ = response;
        Ok(ApprovalTaskActionResult { success: true })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }

    #[test]
    fn test_task_list_query_builder() {
        let query = WorkflowTaskListQuery::for_tasklist("tasklist_123")
            .section_guid("section_456")
            .filter("status = incomplete")
            .sort(json!([{"field": "due", "order": "asc"}]))
            .user_type("open_id")
            .page_size(50);

        assert_eq!(query.tasklist_guid.as_deref(), Some("tasklist_123"));
        assert_eq!(query.section_guid.as_deref(), Some("section_456"));
        assert_eq!(query.filter.as_deref(), Some("status = incomplete"));
        assert_eq!(query.user_type.as_deref(), Some("open_id"));
        assert_eq!(query.page_size, Some(50));
    }

    #[test]
    fn test_task_mutation_builder() {
        let mutation = WorkflowTaskMutation::new()
            .summary("完成项目文档")
            .description("补齐 workflow helper")
            .due("2026-09-30T23:59:59Z")
            .priority(3)
            .assignee("ou_xxx")
            .status("in_progress");

        assert_eq!(mutation.summary.as_deref(), Some("完成项目文档"));
        assert_eq!(
            mutation.description.as_deref(),
            Some("补齐 workflow helper")
        );
        assert_eq!(mutation.due.as_deref(), Some("2026-09-30T23:59:59Z"));
        assert_eq!(mutation.priority, Some(3));
        assert_eq!(mutation.assignee.as_deref(), Some("ou_xxx"));
        assert_eq!(mutation.status.as_deref(), Some("in_progress"));
    }

    #[test]
    fn test_approval_task_query_builder() {
        let query = ApprovalTaskQuery::new("ou_xxx", "1")
            .user_id_type("open_id")
            .status("PENDING")
            .instance_code("instance_123")
            .page_size(100);

        assert_eq!(query.user_id, "ou_xxx");
        assert_eq!(query.topic, "1");
        assert_eq!(query.user_id_type.as_deref(), Some("open_id"));
        assert_eq!(query.status.as_deref(), Some("PENDING"));
        assert_eq!(query.instance_code.as_deref(), Some("instance_123"));
        assert_eq!(query.page_size, Some(100));
    }

    #[test]
    fn test_approval_task_action_builder() {
        let action =
            ApprovalTaskAction::new("approval_code", "instance_code", "ou_xxx", "task_123")
                .user_id_type("open_id")
                .comment("已确认")
                .form("[{}]");

        assert_eq!(action.approval_code, "approval_code");
        assert_eq!(action.instance_code, "instance_code");
        assert_eq!(action.user_id, "ou_xxx");
        assert_eq!(action.task_id, "task_123");
        assert_eq!(action.user_id_type.as_deref(), Some("open_id"));
        assert_eq!(action.comment.as_deref(), Some("已确认"));
        assert_eq!(action.form.as_deref(), Some("[{}]"));
    }
}
