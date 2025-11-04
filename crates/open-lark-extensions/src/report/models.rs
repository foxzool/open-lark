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

/// 汇报规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRule {
    /// 规则ID
    pub rule_id: String,
    /// 规则名称
    pub name: String,
    /// 规则描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 规则类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// 规则状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 汇报频率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// 汇报时间设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ReportSchedule>,
    /// 适用范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ReportScope>,
    /// 汇报模板
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<ReportTemplate>,
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

/// 汇报时间设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSchedule {
    /// 汇报时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_time: Option<String>,
    /// 提醒时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_time: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// 工作日设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekdays: Option<Vec<i32>>,
}

/// 汇报适用范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportScope {
    /// 适用的部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 适用的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 适用的角色列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}

/// 汇报模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    /// 模板ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 模板名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// 模板内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ReportField>>,
}

/// 汇报字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportField {
    /// 字段ID
    pub field_id: String,
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 字段描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 规则看板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleView {
    /// 看板ID
    pub view_id: String,
    /// 规则ID
    pub rule_id: String,
    /// 看板名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 看板类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
    /// 看板配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

/// 汇报任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTask {
    /// 任务ID
    pub task_id: String,
    /// 规则ID
    pub rule_id: String,
    /// 任务名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 任务状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 任务类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 汇报者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporter_id: Option<String>,
    /// 汇报者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporter_info: Option<UserInfo>,
    /// 预期汇报时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_report_time: Option<i64>,
    /// 实际汇报时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_report_time: Option<i64>,
    /// 汇报内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_page_response() {
        let response = PageResponse {
            items: vec!["item1".to_string(), "item2".to_string()],
            page_token: Some("next_page_123".to_string()),
            has_more: Some(true),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("item1"));
        assert!(json.contains("item2"));
        assert!(json.contains("next_page_123"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_report_rule() {
        let rule = ReportRule {
            rule_id: "rule_001".to_string(),
            name: "日报规则".to_string(),
            description: Some("每日工作汇报规则".to_string()),
            rule_type: Some("daily".to_string()),
            status: Some("active".to_string()),
            frequency: Some("daily".to_string()),
            schedule: Some(ReportSchedule {
                report_time: Some("18:00".to_string()),
                reminder_time: Some("17:30".to_string()),
                timezone: Some("Asia/Shanghai".to_string()),
                weekdays: Some(vec![1, 2, 3, 4, 5]),
            }),
            scope: Some(ReportScope {
                department_ids: Some(vec!["dept_001".to_string(), "dept_002".to_string()]),
                user_ids: Some(vec!["user_001".to_string()]),
                roles: Some(vec!["manager".to_string(), "employee".to_string()]),
            }),
            template: Some(ReportTemplate {
                template_id: Some("tpl_001".to_string()),
                template_name: Some("日报模板".to_string()),
                content: Some("今日工作内容...".to_string()),
                fields: Some(vec![ReportField {
                    field_id: "field_001".to_string(),
                    field_name: "工作内容".to_string(),
                    field_type: Some("text".to_string()),
                    required: Some(true),
                    description: Some("请填写今日完成的工作内容".to_string()),
                }]),
            }),
            creator: Some("creator_001".to_string()),
            created_at: Some(1640995200),
            updated_at: Some(1640995200),
        };
        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("rule_001"));
        assert!(json.contains("日报规则"));
        assert!(json.contains("daily"));
        assert!(json.contains("18:00"));
        assert!(json.contains("Asia/Shanghai"));
        assert!(json.contains("dept_001"));
    }

    #[test]
    fn test_report_schedule() {
        let schedule = ReportSchedule {
            report_time: Some("09:00".to_string()),
            reminder_time: Some("08:30".to_string()),
            timezone: Some("UTC".to_string()),
            weekdays: Some(vec![1, 2, 3, 4, 5, 6, 7]),
        };
        let json = serde_json::to_string(&schedule).unwrap();
        assert!(json.contains("09:00"));
        assert!(json.contains("08:30"));
        assert!(json.contains("UTC"));
        assert!(json.contains("[1,2,3,4,5,6,7]"));
    }

    #[test]
    fn test_report_scope() {
        let scope = ReportScope {
            department_ids: Some(vec!["engineering".to_string(), "marketing".to_string()]),
            user_ids: Some(vec!["alice".to_string(), "bob".to_string()]),
            roles: Some(vec!["developer".to_string(), "designer".to_string()]),
        };
        let json = serde_json::to_string(&scope).unwrap();
        assert!(json.contains("engineering"));
        assert!(json.contains("marketing"));
        assert!(json.contains("alice"));
        assert!(json.contains("developer"));
    }

    #[test]
    fn test_report_template() {
        let template = ReportTemplate {
            template_id: Some("template_daily".to_string()),
            template_name: Some("Daily Report Template".to_string()),
            content: Some("Please fill out your daily progress report".to_string()),
            fields: Some(vec![
                ReportField {
                    field_id: "progress".to_string(),
                    field_name: "Progress".to_string(),
                    field_type: Some("textarea".to_string()),
                    required: Some(true),
                    description: Some("Describe your progress today".to_string()),
                },
                ReportField {
                    field_id: "challenges".to_string(),
                    field_name: "Challenges".to_string(),
                    field_type: Some("text".to_string()),
                    required: Some(false),
                    description: Some("Any challenges faced".to_string()),
                },
            ]),
        };
        let json = serde_json::to_string(&template).unwrap();
        assert!(json.contains("template_daily"));
        assert!(json.contains("Daily Report Template"));
        assert!(json.contains("progress"));
        assert!(json.contains("challenges"));
        assert!(json.contains("textarea"));
    }

    #[test]
    fn test_report_field() {
        let field = ReportField {
            field_id: "accomplishments".to_string(),
            field_name: "Accomplishments".to_string(),
            field_type: Some("text".to_string()),
            required: Some(true),
            description: Some("List your accomplishments".to_string()),
        };
        let json = serde_json::to_string(&field).unwrap();
        assert!(json.contains("accomplishments"));
        assert!(json.contains("Accomplishments"));
        assert!(json.contains("\"required\":true"));
        assert!(json.contains("List your accomplishments"));
    }

    #[test]
    fn test_rule_view() {
        let view = RuleView {
            view_id: "view_dashboard_001".to_string(),
            rule_id: "rule_weekly".to_string(),
            name: Some("Weekly Report Dashboard".to_string()),
            view_type: Some("dashboard".to_string()),
            config: Some("{\"layout\":\"grid\",\"refresh\":300}".to_string()),
            creator: Some("admin_001".to_string()),
            created_at: Some(1640995200),
        };
        let json = serde_json::to_string(&view).unwrap();
        assert!(json.contains("view_dashboard_001"));
        assert!(json.contains("rule_weekly"));
        assert!(json.contains("Weekly Report Dashboard"));
        assert!(json.contains("dashboard"));
        assert!(json.contains("grid"));
    }

    #[test]
    fn test_report_task() {
        let user_info = UserInfo {
            user_id: "user_123".to_string(),
            name: Some("John Doe".to_string()),
            email: Some("john.doe@company.com".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        };

        let task = ReportTask {
            task_id: "task_20240101_001".to_string(),
            rule_id: "rule_daily".to_string(),
            name: Some("Daily Report - 2024-01-01".to_string()),
            status: Some("pending".to_string()),
            task_type: Some("daily_report".to_string()),
            reporter_id: Some("user_123".to_string()),
            reporter_info: Some(user_info),
            expected_report_time: Some(1704067200),
            actual_report_time: Some(1704070800),
            content: Some("Today I completed the API integration and fixed 3 bugs.".to_string()),
            created_at: Some(1704000000),
            updated_at: Some(1704070800),
        };
        let json = serde_json::to_string(&task).unwrap();
        assert!(json.contains("task_20240101_001"));
        assert!(json.contains("rule_daily"));
        assert!(json.contains("Daily Report - 2024-01-01"));
        assert!(json.contains("pending"));
        assert!(json.contains("John Doe"));
        assert!(json.contains("john.doe@company.com"));
        assert!(json.contains("API integration"));
    }

    #[test]
    fn test_user_info() {
        let user = UserInfo {
            user_id: "user_456".to_string(),
            name: Some("Alice Smith".to_string()),
            email: Some("alice.smith@example.com".to_string()),
            avatar_url: Some("https://cdn.example.com/avatars/alice.png".to_string()),
        };
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("user_456"));
        assert!(json.contains("Alice Smith"));
        assert!(json.contains("alice.smith@example.com"));
        assert!(json.contains("avatars/alice.png"));
    }

    #[test]
    fn test_minimal_structs() {
        let minimal_rule = ReportRule {
            rule_id: "minimal_rule".to_string(),
            name: "Minimal Rule".to_string(),
            description: None,
            rule_type: None,
            status: None,
            frequency: None,
            schedule: None,
            scope: None,
            template: None,
            creator: None,
            created_at: None,
            updated_at: None,
        };
        let json = serde_json::to_string(&minimal_rule).unwrap();
        assert!(json.contains("minimal_rule"));
        assert!(json.contains("Minimal Rule"));
        assert!(!json.contains("description"));
        assert!(!json.contains("rule_type"));

        let minimal_user = UserInfo {
            user_id: "user_minimal".to_string(),
            name: None,
            email: None,
            avatar_url: None,
        };
        let json = serde_json::to_string(&minimal_user).unwrap();
        assert!(json.contains("user_minimal"));
        assert!(!json.contains("name"));
        assert!(!json.contains("email"));
    }

    #[test]
    fn test_nested_structures() {
        let page_response = PageResponse {
            items: vec![
                ReportRule {
                    rule_id: "rule_1".to_string(),
                    name: "Rule 1".to_string(),
                    description: None,
                    rule_type: None,
                    status: None,
                    frequency: None,
                    schedule: None,
                    scope: None,
                    template: None,
                    creator: None,
                    created_at: None,
                    updated_at: None,
                },
                ReportRule {
                    rule_id: "rule_2".to_string(),
                    name: "Rule 2".to_string(),
                    description: Some("Second rule".to_string()),
                    rule_type: Some("weekly".to_string()),
                    status: Some("active".to_string()),
                    frequency: None,
                    schedule: None,
                    scope: None,
                    template: None,
                    creator: None,
                    created_at: None,
                    updated_at: None,
                },
            ],
            page_token: Some("page_2".to_string()),
            has_more: Some(false),
        };
        let json = serde_json::to_string(&page_response).unwrap();
        assert!(json.contains("rule_1"));
        assert!(json.contains("rule_2"));
        assert!(json.contains("Second rule"));
        assert!(json.contains("weekly"));
        assert!(json.contains("page_2"));
        assert!(json.contains("false"));
    }

    #[test]
    fn test_complex_report_workflow() {
        let schedule = ReportSchedule {
            report_time: Some("17:00".to_string()),
            reminder_time: Some("16:30".to_string()),
            timezone: Some("Asia/Shanghai".to_string()),
            weekdays: Some(vec![1, 2, 3, 4, 5]),
        };

        let scope = ReportScope {
            department_ids: Some(vec!["engineering".to_string()]),
            user_ids: Some(vec!["dev_001".to_string(), "dev_002".to_string()]),
            roles: Some(vec!["senior_developer".to_string()]),
        };

        let template = ReportTemplate {
            template_id: Some("engineering_daily".to_string()),
            template_name: Some("Engineering Daily Report".to_string()),
            content: Some("Please provide your daily engineering update".to_string()),
            fields: Some(vec![
                ReportField {
                    field_id: "code_commits".to_string(),
                    field_name: "Code Commits".to_string(),
                    field_type: Some("number".to_string()),
                    required: Some(true),
                    description: Some("Number of commits made today".to_string()),
                },
                ReportField {
                    field_id: "bugs_fixed".to_string(),
                    field_name: "Bugs Fixed".to_string(),
                    field_type: Some("number".to_string()),
                    required: Some(false),
                    description: Some("Number of bugs resolved".to_string()),
                },
            ]),
        };

        let rule = ReportRule {
            rule_id: "eng_daily_report".to_string(),
            name: "Engineering Daily Report".to_string(),
            description: Some("Daily report for engineering team productivity".to_string()),
            rule_type: Some("daily".to_string()),
            status: Some("active".to_string()),
            frequency: Some("daily".to_string()),
            schedule: Some(schedule),
            scope: Some(scope),
            template: Some(template),
            creator: Some("engineering_manager".to_string()),
            created_at: Some(1704067200),
            updated_at: Some(1704067200),
        };

        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("eng_daily_report"));
        assert!(json.contains("Engineering Daily Report"));
        assert!(json.contains("Asia/Shanghai"));
        assert!(json.contains("engineering"));
        assert!(json.contains("code_commits"));
        assert!(json.contains("bugs_fixed"));
        assert!(json.contains("engineering_manager"));
    }
}
