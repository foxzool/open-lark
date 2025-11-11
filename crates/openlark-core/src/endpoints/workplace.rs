//! 工作台服务端点常量定义

// ==================== 工作台服务端点 ====================

/// 搜索工作台访问数据
pub const WORKPLACE_ACCESS_DATA_SEARCH: &str =
    "/open-apis/workplace/v1/workplace_access_data/search";

/// 搜索自定义工作台访问数据
pub const WORKPLACE_CUSTOM_ACCESS_DATA_SEARCH: &str =
    "/open-apis/workplace/v1/custom_workplace_access_data/search";

/// 搜索工作台小部件访问数据
pub const WORKPLACE_WIDGET_ACCESS_DATA_SEARCH: &str =
    "/open-apis/workplace/v1/custom_workplace_widget_access_data/search";

/// 收藏应用推荐规则
pub const WORKPLACE_APP_RECOMMEND_FAVOURITE: &str =
    "/open-apis/workplace/v1/app_recommend_rule/favourite";

/// 推荐应用推荐规则
pub const WORKPLACE_APP_RECOMMEND_RECOMMEND: &str =
    "/open-apis/workplace/v1/app_recommend_rule/recommend";

/// 获取应用推荐规则列表
pub const WORKPLACE_APP_RECOMMEND_LIST: &str = "/open-apis/workplace/v1/app_recommend_rule/list";

/// 工作台ID
pub const WORKPLACE_ID: &str = "/open-apis/workplace/v1/workplace_access_data/search";
