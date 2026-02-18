pub mod attachment;
pub mod comment;
pub mod custom_field;
pub mod section;
pub mod task;
pub mod tasklist;

use openlark_core::config::Config;
use std::sync::Arc;

/// TaskV2：任务 API v2 访问入口
#[derive(Clone)]
pub struct TaskV2 {
    config: Arc<Config>,
}

impl TaskV2 {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问任务资源
    pub fn task(&self) -> task::Task {
        task::Task::new(self.config.clone())
    }

    /// 访问任务清单资源
    pub fn tasklist(&self) -> tasklist::Tasklist {
        tasklist::Tasklist::new(self.config.clone())
    }

    /// 访问自定义字段资源
    pub fn custom_field(&self) -> custom_field::CustomField {
        custom_field::CustomField::new(self.config.clone())
    }

    /// 访问评论资源
    pub fn comment(&self) -> comment::Comment {
        comment::Comment::new(self.config.clone())
    }

    /// 访问分组资源
    pub fn section(&self) -> section::Section {
        section::Section::new(self.config.clone())
    }

    /// 访问附件资源
    pub fn attachment(&self) -> attachment::Attachment {
        attachment::Attachment::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_config() -> Arc<Config> {
        Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        )
    }

    #[test]
    fn test_task_v2_new() {
        let config = create_test_config();
        let _task_v2 = TaskV2::new(config);
    }

    #[test]
    fn test_task_v2_task() {
        let config = create_test_config();
        let task_v2 = TaskV2::new(config);
        let _task = task_v2.task();
    }

    #[test]
    fn test_task_v2_tasklist() {
        let config = create_test_config();
        let task_v2 = TaskV2::new(config);
        let _tasklist = task_v2.tasklist();
    }

    #[test]
    fn test_task_v2_custom_field() {
        let config = create_test_config();
        let task_v2 = TaskV2::new(config);
        let _custom_field = task_v2.custom_field();
    }

    #[test]
    fn test_task_v2_comment() {
        let config = create_test_config();
        let task_v2 = TaskV2::new(config);
        let _comment = task_v2.comment();
    }

    #[test]
    fn test_task_v2_section() {
        let config = create_test_config();
        let task_v2 = TaskV2::new(config);
        let _section = task_v2.section();
    }

    #[test]
    fn test_task_v2_attachment() {
        let config = create_test_config();
        let task_v2 = TaskV2::new(config);
        let _attachment = task_v2.attachment();
    }
}
