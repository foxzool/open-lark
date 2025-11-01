// docx - 文档服务
//,
// 提供文档相关的所有功能，包括：
// - 文档的创建、读取、更新、删除
// - 文档块操作（文本、图片、表格等）
// - 文档版本管理
// - 文档评论和回复
// - 文档导入导出
// - 文档搜索和统计
//,
// 覆盖18个API接口
use crate::prelude::*;
use crate::service::ccm::docx::v1::DocxV1Service;
use crate::service::ccm::docx::documents::DocumentsService;
/// 文档服务
#[derive(Debug, Clone)]
pub struct DocxService {
}

impl DocxService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v1版本API
pub mod v1;
/// 文档操作API
pub mod documents;
}