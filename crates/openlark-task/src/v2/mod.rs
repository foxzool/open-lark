pub mod task;
pub mod tasklist;
pub mod custom_field;
pub mod comment;
pub mod section;
pub mod attachment;

use std::sync::Arc;
use openlark_core::config::Config;

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
