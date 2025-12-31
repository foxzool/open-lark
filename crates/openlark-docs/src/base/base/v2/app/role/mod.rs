pub mod create;
pub mod list;
pub mod update;

use openlark_core::config::Config;

#[derive(Debug, Clone)]
pub struct RoleService {
    config: Config,
}

impl RoleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 新增自定义角色
    pub fn create(&self) -> create::Create {
        create::Create::new(self.config.clone())
    }

    /// 更新自定义角色
    pub fn update(&self) -> update::Update {
        update::Update::new(self.config.clone())
    }

    /// 列出自定义角色
    pub fn list(&self) -> list::List {
        list::List::new(self.config.clone())
    }
}
