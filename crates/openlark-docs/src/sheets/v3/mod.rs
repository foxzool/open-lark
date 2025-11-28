pub mod spreadsheet;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct SheetsV3 {
    service: Arc<DocsService>,
}

impl SheetsV3 {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn spreadsheet(&self) -> spreadsheet::Spreadsheet {
        spreadsheet::Spreadsheet::new(self.service.clone())
    }

    pub fn spreadsheet_sheet(&self) -> spreadsheet::sheet::SpreadsheetSheet {
        spreadsheet::sheet::SpreadsheetSheet::new(self.service.clone())
    }

    pub fn spreadsheet_sheet_filter(&self) -> spreadsheet::sheet::filter::SpreadsheetSheetFilter {
        spreadsheet::sheet::filter::SpreadsheetSheetFilter::new(self.service.clone())
    }

    pub fn spreadsheet_sheet_filter_view(&self) -> spreadsheet::sheet::filter_view::SpreadsheetSheetFilterView {
        spreadsheet::sheet::filter_view::SpreadsheetSheetFilterView::new(self.service.clone())
    }

    pub fn spreadsheet_sheet_filter_view_condition(&self) -> spreadsheet::sheet::filter_view::condition::SpreadsheetSheetFilterViewCondition {
        spreadsheet::sheet::filter_view::condition::SpreadsheetSheetFilterViewCondition::new(self.service.clone())
    }

    pub fn spreadsheet_sheet_float_image(&self) -> spreadsheet::sheet::float_image::SpreadsheetSheetFloatImage {
        spreadsheet::sheet::float_image::SpreadsheetSheetFloatImage::new(self.service.clone())
    }
}