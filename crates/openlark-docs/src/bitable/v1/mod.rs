pub mod app;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct BitableV1 {
    service: Arc<DocsService>,
}

impl BitableV1 {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn app(&self) -> app::App {
        app::App::new(self.service.clone())
    }

    pub fn app_dashboard(&self) -> app::dashboard::AppDashboard {
        app::dashboard::AppDashboard::new(self.service.clone())
    }

    pub fn app_role(&self) -> app::role::AppRole {
        app::role::AppRole::new(self.service.clone())
    }

    pub fn app_role_member(&self) -> app::role::member::AppRoleMember {
        app::role::member::AppRoleMember::new(self.service.clone())
    }

    pub fn app_table(&self) -> app::table::AppTable {
        app::table::AppTable::new(self.service.clone())
    }

    pub fn app_table_field(&self) -> app::table::field::AppTableField {
        app::table::field::AppTableField::new(self.service.clone())
    }

    pub fn app_table_form(&self) -> app::table::form::AppTableForm {
        app::table::form::AppTableForm::new(self.service.clone())
    }

    pub fn app_table_form_field(&self) -> app::table::form::field::AppTableFormField {
        app::table::form::field::AppTableFormField::new(self.service.clone())
    }

    pub fn app_table_record(&self) -> app::table::record::AppTableRecord {
        app::table::record::AppTableRecord::new(self.service.clone())
    }

    pub fn app_table_view(&self) -> app::table::view::AppTableView {
        app::table::view::AppTableView::new(self.service.clone())
    }

    pub fn app_workflow(&self) -> app::workflow::AppWorkflow {
        app::workflow::AppWorkflow::new(self.service.clone())
    }
}