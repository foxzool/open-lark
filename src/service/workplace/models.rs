use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 工作台访问数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkplaceAccessData {
    /// 数据ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 访问时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_time: Option<i64>,
    /// 访问类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    /// 访问次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i32>,
    /// 访问时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// 访问的应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 平台类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// 设备类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

/// 定制工作台访问数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomWorkplaceAccessData {
    /// 数据ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 定制工作台ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_workplace_id: Option<String>,
    /// 访问时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_time: Option<i64>,
    /// 访问次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i32>,
    /// 访问时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// 操作类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
}

/// 定制工作台小组件访问数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomWorkplaceWidgetAccessData {
    /// 数据ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 定制工作台ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_workplace_id: Option<String>,
    /// 小组件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_id: Option<String>,
    /// 小组件名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_name: Option<String>,
    /// 访问时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_time: Option<i64>,
    /// 访问次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i32>,
    /// 点击次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_count: Option<i32>,
}

/// 应用信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    /// 应用ID
    pub app_id: String,
    /// 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// 应用描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_description: Option<String>,
    /// 应用图标URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_icon_url: Option<String>,
    /// 应用类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
    /// 应用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 用户常用应用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavouriteApp {
    /// 应用ID
    pub app_id: String,
    /// 应用信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_info: Option<AppInfo>,
    /// 添加到常用的时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourited_at: Option<i64>,
    /// 使用频率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_frequency: Option<i32>,
    /// 最后使用时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<i64>,
}

/// 推荐应用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedApp {
    /// 应用ID
    pub app_id: String,
    /// 应用信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_info: Option<AppInfo>,
    /// 推荐原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommend_reason: Option<String>,
    /// 推荐分数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommend_score: Option<f64>,
    /// 推荐时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_at: Option<i64>,
    /// 推荐规则ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

/// 应用推荐规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRecommendRule {
    /// 规则ID
    pub rule_id: String,
    /// 规则名称
    pub rule_name: String,
    /// 规则描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_description: Option<String>,
    /// 规则类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// 规则状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 适用的应用ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<Vec<String>>,
    /// 适用的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 适用的部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 生效开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 生效结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_page_response() {
        let access_data = WorkplaceAccessData {
            data_id: Some("access_1".to_string()),
            user_id: Some("ou_user123".to_string()),
            department_id: Some("dept_456".to_string()),
            access_time: Some(1704067200000),
            access_type: Some("login".to_string()),
            access_count: Some(5),
            duration: Some(3600),
            app_id: Some("app_789".to_string()),
            platform: Some("web".to_string()),
            device_type: Some("desktop".to_string()),
        };

        let response: PageResponse<WorkplaceAccessData> = PageResponse {
            items: vec![access_data],
            page_token: Some("next_page_token".to_string()),
            has_more: Some(true),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("access_1"));
        assert!(json.contains("ou_user123"));
        assert!(json.contains("next_page_token"));
        assert!(json.contains("\"has_more\":true"));
    }

    #[test]
    fn test_workplace_access_data() {
        let access_data = WorkplaceAccessData {
            data_id: Some("workplace_access_123".to_string()),
            user_id: Some("ou_user456".to_string()),
            department_id: Some("dept_tech".to_string()),
            access_time: Some(1704067200000),
            access_type: Some("app_launch".to_string()),
            access_count: Some(10),
            duration: Some(7200),
            app_id: Some("app_productivity".to_string()),
            platform: Some("mobile".to_string()),
            device_type: Some("android".to_string()),
        };

        let json = serde_json::to_string(&access_data).unwrap();
        assert!(json.contains("workplace_access_123"));
        assert!(json.contains("ou_user456"));
        assert!(json.contains("dept_tech"));
        assert!(json.contains("1704067200000"));
        assert!(json.contains("app_launch"));
        assert!(json.contains("\"access_count\":10"));
        assert!(json.contains("\"duration\":7200"));
        assert!(json.contains("app_productivity"));
        assert!(json.contains("mobile"));
        assert!(json.contains("android"));
    }

    #[test]
    fn test_workplace_access_data_minimal() {
        let access_data = WorkplaceAccessData {
            data_id: Some("minimal_access".to_string()),
            user_id: None,
            department_id: None,
            access_time: None,
            access_type: None,
            access_count: None,
            duration: None,
            app_id: None,
            platform: None,
            device_type: None,
        };

        let json = serde_json::to_string(&access_data).unwrap();
        assert!(json.contains("minimal_access"));
        assert!(!json.contains("user_id"));
        assert!(!json.contains("access_time"));
        assert!(!json.contains("app_id"));
    }

    #[test]
    fn test_custom_workplace_access_data() {
        let custom_access = CustomWorkplaceAccessData {
            data_id: Some("custom_access_789".to_string()),
            user_id: Some("ou_customuser".to_string()),
            custom_workplace_id: Some("workplace_custom_001".to_string()),
            access_time: Some(1704153600000),
            access_count: Some(3),
            duration: Some(1800),
            action_type: Some("customize".to_string()),
        };

        let json = serde_json::to_string(&custom_access).unwrap();
        assert!(json.contains("custom_access_789"));
        assert!(json.contains("ou_customuser"));
        assert!(json.contains("workplace_custom_001"));
        assert!(json.contains("1704153600000"));
        assert!(json.contains("\"access_count\":3"));
        assert!(json.contains("\"duration\":1800"));
        assert!(json.contains("customize"));
    }

    #[test]
    fn test_custom_workplace_widget_access_data() {
        let widget_access = CustomWorkplaceWidgetAccessData {
            data_id: Some("widget_access_456".to_string()),
            user_id: Some("ou_widgetuser".to_string()),
            custom_workplace_id: Some("workplace_001".to_string()),
            widget_id: Some("widget_calendar".to_string()),
            widget_name: Some("日历组件".to_string()),
            access_time: Some(1704240000000),
            access_count: Some(15),
            click_count: Some(25),
        };

        let json = serde_json::to_string(&widget_access).unwrap();
        assert!(json.contains("widget_access_456"));
        assert!(json.contains("ou_widgetuser"));
        assert!(json.contains("workplace_001"));
        assert!(json.contains("widget_calendar"));
        assert!(json.contains("日历组件"));
        assert!(json.contains("1704240000000"));
        assert!(json.contains("\"access_count\":15"));
        assert!(json.contains("\"click_count\":25"));
    }

    #[test]
    fn test_app_info() {
        let app_info = AppInfo {
            app_id: "app_productivity_suite".to_string(),
            app_name: Some("生产力套件".to_string()),
            app_description: Some("全能办公生产力工具集合".to_string()),
            app_icon_url: Some("https://cdn.example.com/icons/productivity.png".to_string()),
            app_type: Some("productivity".to_string()),
            status: Some("active".to_string()),
            created_at: Some(1703980800000),
            updated_at: Some(1704067200000),
        };

        let json = serde_json::to_string(&app_info).unwrap();
        assert!(json.contains("app_productivity_suite"));
        assert!(json.contains("生产力套件"));
        assert!(json.contains("全能办公生产力工具集合"));
        assert!(json.contains("https://cdn.example.com/icons/productivity.png"));
        assert!(json.contains("productivity"));
        assert!(json.contains("active"));
        assert!(json.contains("1703980800000"));
        assert!(json.contains("1704067200000"));
    }

    #[test]
    fn test_favourite_app() {
        let app_info = AppInfo {
            app_id: "app_fav_123".to_string(),
            app_name: Some("常用应用".to_string()),
            app_description: Some("用户常用的应用".to_string()),
            app_icon_url: Some("https://cdn.example.com/fav-app.png".to_string()),
            app_type: Some("utility".to_string()),
            status: Some("active".to_string()),
            created_at: Some(1703980800000),
            updated_at: None,
        };

        let fav_app = FavouriteApp {
            app_id: "app_fav_123".to_string(),
            app_info: Some(app_info),
            favourited_at: Some(1704067200000),
            usage_frequency: Some(50),
            last_used_at: Some(1704326400000),
        };

        let json = serde_json::to_string(&fav_app).unwrap();
        assert!(json.contains("app_fav_123"));
        assert!(json.contains("常用应用"));
        assert!(json.contains("用户常用的应用"));
        assert!(json.contains("utility"));
        assert!(json.contains("1704067200000"));
        assert!(json.contains("\"usage_frequency\":50"));
        assert!(json.contains("1704326400000"));
    }

    #[test]
    fn test_recommended_app() {
        let app_info = AppInfo {
            app_id: "app_recommended_456".to_string(),
            app_name: Some("推荐应用".to_string()),
            app_description: Some("系统推荐的应用".to_string()),
            app_icon_url: Some("https://cdn.example.com/recommended.png".to_string()),
            app_type: Some("collaboration".to_string()),
            status: Some("active".to_string()),
            created_at: Some(1703980800000),
            updated_at: Some(1704067200000),
        };

        let recommended_app = RecommendedApp {
            app_id: "app_recommended_456".to_string(),
            app_info: Some(app_info),
            recommend_reason: Some("基于您的工作习惯推荐".to_string()),
            recommend_score: Some(8.5),
            recommended_at: Some(1704412800000),
            rule_id: Some("rule_001".to_string()),
        };

        let json = serde_json::to_string(&recommended_app).unwrap();
        assert!(json.contains("app_recommended_456"));
        assert!(json.contains("推荐应用"));
        assert!(json.contains("系统推荐的应用"));
        assert!(json.contains("collaboration"));
        assert!(json.contains("基于您的工作习惯推荐"));
        assert!(json.contains("8.5"));
        assert!(json.contains("1704412800000"));
        assert!(json.contains("rule_001"));
    }

    #[test]
    fn test_app_recommend_rule() {
        let rule = AppRecommendRule {
            rule_id: "rule_smart_recommend".to_string(),
            rule_name: "智能推荐规则".to_string(),
            rule_description: Some("基于用户行为和偏好的智能应用推荐规则".to_string()),
            rule_type: Some("ml_based".to_string()),
            status: Some("active".to_string()),
            app_ids: Some(vec![
                "app_001".to_string(),
                "app_002".to_string(),
                "app_003".to_string(),
            ]),
            user_ids: Some(vec!["ou_user_001".to_string(), "ou_user_002".to_string()]),
            department_ids: Some(vec!["dept_tech".to_string(), "dept_product".to_string()]),
            priority: Some(10),
            start_time: Some(1704067200000),
            end_time: Some(1735689600000), // 一年后
            creator: Some("ou_admin".to_string()),
            created_at: Some(1703980800000),
            updated_at: Some(1704067200000),
        };

        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("rule_smart_recommend"));
        assert!(json.contains("智能推荐规则"));
        assert!(json.contains("基于用户行为和偏好的智能应用推荐规则"));
        assert!(json.contains("ml_based"));
        assert!(json.contains("active"));
        assert!(json.contains("app_001"));
        assert!(json.contains("app_002"));
        assert!(json.contains("app_003"));
        assert!(json.contains("ou_user_001"));
        assert!(json.contains("ou_user_002"));
        assert!(json.contains("dept_tech"));
        assert!(json.contains("dept_product"));
        assert!(json.contains("\"priority\":10"));
        assert!(json.contains("1704067200000"));
        assert!(json.contains("1735689600000"));
        assert!(json.contains("ou_admin"));
        assert!(json.contains("1703980800000"));
    }

    #[test]
    fn test_app_recommend_rule_minimal() {
        let rule = AppRecommendRule {
            rule_id: "minimal_rule".to_string(),
            rule_name: "最小规则".to_string(),
            rule_description: None,
            rule_type: None,
            status: None,
            app_ids: None,
            user_ids: None,
            department_ids: None,
            priority: None,
            start_time: None,
            end_time: None,
            creator: None,
            created_at: None,
            updated_at: None,
        };

        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("minimal_rule"));
        assert!(json.contains("最小规则"));
        assert!(!json.contains("rule_description"));
        assert!(!json.contains("app_ids"));
        assert!(!json.contains("priority"));
    }

    #[test]
    fn test_nested_structures() {
        let app_info = AppInfo {
            app_id: "nested_app".to_string(),
            app_name: Some("嵌套测试应用".to_string()),
            app_description: None,
            app_icon_url: None,
            app_type: Some("test".to_string()),
            status: Some("active".to_string()),
            created_at: Some(1704067200000),
            updated_at: None,
        };

        let fav_app = FavouriteApp {
            app_id: "nested_app".to_string(),
            app_info: Some(app_info),
            favourited_at: Some(1704153600000),
            usage_frequency: Some(20),
            last_used_at: Some(1704240000000),
        };

        let json = serde_json::to_string(&fav_app).unwrap();
        assert!(json.contains("nested_app"));
        assert!(json.contains("嵌套测试应用"));
        assert!(json.contains("test"));
        assert!(json.contains("\"usage_frequency\":20"));
        assert!(!json.contains("app_description"));
        assert!(!json.contains("updated_at"));
    }

    #[test]
    fn test_minimal_structs() {
        let minimal_custom_access = CustomWorkplaceAccessData {
            data_id: Some("minimal_custom".to_string()),
            user_id: None,
            custom_workplace_id: None,
            access_time: None,
            access_count: None,
            duration: None,
            action_type: None,
        };

        let json = serde_json::to_string(&minimal_custom_access).unwrap();
        assert!(json.contains("minimal_custom"));
        assert!(!json.contains("user_id"));
        assert!(!json.contains("custom_workplace_id"));

        let minimal_widget_access = CustomWorkplaceWidgetAccessData {
            data_id: Some("minimal_widget".to_string()),
            user_id: None,
            custom_workplace_id: None,
            widget_id: None,
            widget_name: None,
            access_time: None,
            access_count: None,
            click_count: None,
        };

        let widget_json = serde_json::to_string(&minimal_widget_access).unwrap();
        assert!(widget_json.contains("minimal_widget"));
        assert!(!widget_json.contains("widget_id"));
        assert!(!widget_json.contains("click_count"));
    }
}
