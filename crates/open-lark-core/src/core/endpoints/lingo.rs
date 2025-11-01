//! lingo 服务端点常量定义

/// 分类列表
pub const LINGO_CLASSIFICATION_LIST: &str = "/open-apis/lingo/v1/classifications";

/// 草稿创建
pub const LINGO_DRAFT_CREATE: &str = "/open-apis/lingo/v1/drafts";

/// 草稿更新
pub const LINGO_DRAFT_UPDATE: &str = "/open-apis/lingo/v1/drafts/{draft_id}";

/// 词条创建
pub const LINGO_ENTITY_CREATE: &str = "/open-apis/lingo/v1/entities";

/// 词条获取
pub const LINGO_ENTITY_GET: &str = "/open-apis/lingo/v1/entities/{entity_id}";

/// 词条更新
pub const LINGO_ENTITY_UPDATE: &str = "/open-apis/lingo/v1/entities/{entity_id}";

/// 词条搜索
pub const LINGO_ENTITY_SEARCH: &str = "/open-apis/lingo/v1/entities/search";

/// 词条匹配
pub const LINGO_ENTITY_MATCH: &str = "/open-apis/lingo/v1/entities/match";

/// 词条高亮
pub const LINGO_ENTITY_HIGHLIGHT: &str = "/open-apis/lingo/v1/entities/highlight";

/// 文件上传
pub const LINGO_FILE_UPLOAD: &str = "/open-apis/lingo/v1/file/upload";

/// 文件下载
pub const LINGO_FILE_DOWNLOAD: &str = "/open-apis/lingo/v1/file/download/{file_token}";

/// 知识库列表
pub const LINGO_REPO_LIST: &str = "/open-apis/lingo/v1/repos";
