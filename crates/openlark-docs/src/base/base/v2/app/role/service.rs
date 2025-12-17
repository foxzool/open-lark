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
    pub fn create(&self) -> super::create::Create {
        super::create::Create::new(self.config.clone())
    }

    /// 更新自定义角色
    pub fn update(&self) -> super::update::Update {
        super::update::Update::new(self.config.clone())
    }

    /// 列出自定义角色
    pub fn list(&self) -> super::list::List {
        super::list::List::new(self.config.clone())
    }
}
