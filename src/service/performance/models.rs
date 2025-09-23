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

/// 周期状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemesterStatus {
    /// 未开始
    NotStarted,
    /// 进行中
    InProgress,
    /// 已结束
    Finished,
    /// 已暂停
    Paused,
}

/// 周期信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Semester {
    /// 周期ID
    pub semester_id: String,
    /// 周期名称
    pub name: String,
    /// 周期描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 周期状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SemesterStatus>,
    /// 开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 项目状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityStatus {
    /// 未开始
    NotStarted,
    /// 进行中
    InProgress,
    /// 已结束
    Finished,
    /// 已暂停
    Paused,
    /// 已取消
    Cancelled,
}

/// 项目类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    /// 绩效评估
    Performance,
    /// 360度评估
    Full360,
    /// 自评
    SelfReview,
    /// 上级评估
    ManagerReview,
    /// 同事评估
    PeerReview,
}

/// 项目信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    /// 项目ID
    pub activity_id: String,
    /// 项目名称
    pub name: String,
    /// 项目描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 项目类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ActivityType>,
    /// 项目状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ActivityStatus>,
    /// 所属周期ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    /// 开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 补充信息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdditionalInfoType {
    /// 文本
    Text,
    /// 数字
    Number,
    /// 日期
    Date,
    /// 选择项
    Selection,
    /// 多选项
    MultiSelection,
}

/// 补充信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalInformation {
    /// 信息ID
    pub info_id: String,
    /// 用户ID
    pub user_id: String,
    /// 项目ID
    pub activity_id: String,
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    pub field_type: AdditionalInfoType,
    /// 字段值
    pub field_value: String,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 用户组信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroup {
    /// 用户组ID
    pub group_id: String,
    /// 用户组名称
    pub name: String,
    /// 用户组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 成员用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_user_ids: Option<Vec<String>>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 被评估人信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reviewee {
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub name: String,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// 职位信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 上级ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<String>,
    /// 项目ID
    pub activity_id: String,
    /// 评估状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<String>,
}

/// 评估模板类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateType {
    /// 自评
    SelfReview,
    /// 上级评估
    ManagerReview,
    /// 同事评估
    PeerReview,
    /// 下级评估
    SubordinateReview,
}

/// 评估模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewTemplate {
    /// 模板ID
    pub template_id: String,
    /// 模板名称
    pub name: String,
    /// 模板描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 模板类型
    pub template_type: TemplateType,
    /// 项目ID
    pub activity_id: String,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 评估项类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewItemType {
    /// 评分题
    Rating,
    /// 文本题
    Text,
    /// 单选题
    SingleChoice,
    /// 多选题
    MultipleChoice,
    /// 标签题
    Tag,
}

/// 评估项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewItem {
    /// 评估项ID
    pub item_id: String,
    /// 评估项名称
    pub name: String,
    /// 评估项描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 评估项类型
    pub item_type: ReviewItemType,
    /// 所属模板ID
    pub template_id: String,
    /// 排序权重
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 选项配置（JSON格式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}

/// 标签填写题配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TagQuestionConfig {
    /// 配置ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    /// 可选标签列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// 指标类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricType {
    /// 数值型
    Number,
    /// 百分比
    Percentage,
    /// 文本型
    Text,
    /// 布尔型
    Boolean,
}

/// 指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// 指标ID
    pub metric_id: String,
    /// 指标名称
    pub name: String,
    /// 指标描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 指标类型
    pub metric_type: MetricType,
    /// 指标单位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// 是否为关键指标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_key: Option<bool>,
    /// 权重
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

/// 指标模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricTemplate {
    /// 模板ID
    pub template_id: String,
    /// 模板名称
    pub name: String,
    /// 模板描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 包含的指标ID列表
    pub metric_ids: Vec<String>,
    /// 项目ID
    pub activity_id: String,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

/// 指标字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricField {
    /// 字段ID
    pub field_id: String,
    /// 字段名称
    pub name: String,
    /// 字段类型
    pub field_type: MetricType,
    /// 所属指标ID
    pub metric_id: String,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 默认值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

/// 指标标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricTag {
    /// 标签ID
    pub tag_id: String,
    /// 标签名称
    pub name: String,
    /// 标签颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// 标签描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// 未开始
    NotStarted,
    /// 进行中
    InProgress,
    /// 已完成
    Completed,
    /// 已过期
    Expired,
    /// 已暂停
    Paused,
}

/// 周期任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageTask {
    /// 任务ID
    pub task_id: String,
    /// 任务名称
    pub name: String,
    /// 任务类型
    pub task_type: String,
    /// 任务状态
    pub status: TaskStatus,
    /// 被评估人ID
    pub reviewee_id: String,
    /// 评估人ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer_id: Option<String>,
    /// 项目ID
    pub activity_id: String,
    /// 周期ID
    pub semester_id: String,
    /// 任务开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 任务结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 完成时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

/// 指标详情数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDetail {
    /// 数据ID
    pub detail_id: String,
    /// 指标ID
    pub metric_id: String,
    /// 被评估人ID
    pub reviewee_id: String,
    /// 项目ID
    pub activity_id: String,
    /// 指标值
    pub value: String,
    /// 指标单位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 录入时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_time: Option<i64>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

/// 绩效结果等级
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerformanceLevel {
    /// 优秀
    Excellent,
    /// 良好
    Good,
    /// 一般
    Average,
    /// 需改进
    NeedsImprovement,
    /// 不合格
    Unsatisfactory,
}

/// 绩效结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceResult {
    /// 结果ID
    pub result_id: String,
    /// 被评估人ID
    pub reviewee_id: String,
    /// 项目ID
    pub activity_id: String,
    /// 周期ID
    pub semester_id: String,
    /// 绩效等级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<PerformanceLevel>,
    /// 绩效分数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// 排名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    /// 总体评价
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_comment: Option<String>,
    /// 是否已开通结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_opened: Option<bool>,
    /// 开通时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_at: Option<i64>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 绩效详情数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewDetail {
    /// 详情ID
    pub detail_id: String,
    /// 被评估人ID
    pub reviewee_id: String,
    /// 评估人ID
    pub reviewer_id: String,
    /// 项目ID
    pub activity_id: String,
    /// 评估项ID
    pub item_id: String,
    /// 评估内容/回答
    pub content: String,
    /// 评分（如果是评分题）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// 提交时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<i64>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_page_response_serialization() {
        let page = PageResponse {
            items: vec!["item1".to_string(), "item2".to_string()],
            page_token: Some("token123".to_string()),
            has_more: Some(true),
        };
        let json = serde_json::to_string(&page).unwrap();
        assert!(json.contains("token123"));
        assert!(json.contains("item1"));
    }

    #[test]
    fn test_semester_status_enum() {
        assert_eq!(
            serde_json::to_string(&SemesterStatus::NotStarted).unwrap(),
            "\"not_started\""
        );
        assert_eq!(
            serde_json::to_string(&SemesterStatus::InProgress).unwrap(),
            "\"in_progress\""
        );
        assert_eq!(
            serde_json::to_string(&SemesterStatus::Finished).unwrap(),
            "\"finished\""
        );
        assert_eq!(
            serde_json::to_string(&SemesterStatus::Paused).unwrap(),
            "\"paused\""
        );
    }

    #[test]
    fn test_semester_full() {
        let semester = Semester {
            semester_id: "sem123".to_string(),
            name: "2024年度绩效评估".to_string(),
            description: Some("年度绩效考核周期".to_string()),
            status: Some(SemesterStatus::InProgress),
            start_time: Some(1704067200000),
            end_time: Some(1735689600000),
            created_at: Some(1703980800000),
            updated_at: Some(1703980800000),
        };
        let json = serde_json::to_string(&semester).unwrap();
        assert!(json.contains("sem123"));
        assert!(json.contains("2024年度绩效评估"));
        assert!(json.contains("in_progress"));
    }

    #[test]
    fn test_activity_status_enum() {
        assert_eq!(
            serde_json::to_string(&ActivityStatus::NotStarted).unwrap(),
            "\"not_started\""
        );
        assert_eq!(
            serde_json::to_string(&ActivityStatus::InProgress).unwrap(),
            "\"in_progress\""
        );
        assert_eq!(
            serde_json::to_string(&ActivityStatus::Finished).unwrap(),
            "\"finished\""
        );
        assert_eq!(
            serde_json::to_string(&ActivityStatus::Paused).unwrap(),
            "\"paused\""
        );
        assert_eq!(
            serde_json::to_string(&ActivityStatus::Cancelled).unwrap(),
            "\"cancelled\""
        );
    }

    #[test]
    fn test_activity_type_enum() {
        assert_eq!(
            serde_json::to_string(&ActivityType::Performance).unwrap(),
            "\"performance\""
        );
        assert_eq!(
            serde_json::to_string(&ActivityType::Full360).unwrap(),
            "\"full360\""
        );
        assert_eq!(
            serde_json::to_string(&ActivityType::SelfReview).unwrap(),
            "\"self_review\""
        );
        assert_eq!(
            serde_json::to_string(&ActivityType::ManagerReview).unwrap(),
            "\"manager_review\""
        );
        assert_eq!(
            serde_json::to_string(&ActivityType::PeerReview).unwrap(),
            "\"peer_review\""
        );
    }

    #[test]
    fn test_activity_performance_review() {
        let activity = Activity {
            activity_id: "act456".to_string(),
            name: "年度绩效评估".to_string(),
            description: Some("2024年度绩效考核".to_string()),
            activity_type: Some(ActivityType::Performance),
            status: Some(ActivityStatus::InProgress),
            semester_id: Some("sem123".to_string()),
            start_time: Some(1704067200000),
            end_time: Some(1706659200000),
            created_at: Some(1703980800000),
            updated_at: Some(1703980800000),
        };
        let json = serde_json::to_string(&activity).unwrap();
        assert!(json.contains("act456"));
        assert!(json.contains("performance"));
        assert!(json.contains("in_progress"));
    }

    #[test]
    fn test_additional_info_type_enum() {
        assert_eq!(
            serde_json::to_string(&AdditionalInfoType::Text).unwrap(),
            "\"text\""
        );
        assert_eq!(
            serde_json::to_string(&AdditionalInfoType::Number).unwrap(),
            "\"number\""
        );
        assert_eq!(
            serde_json::to_string(&AdditionalInfoType::Date).unwrap(),
            "\"date\""
        );
        assert_eq!(
            serde_json::to_string(&AdditionalInfoType::Selection).unwrap(),
            "\"selection\""
        );
        assert_eq!(
            serde_json::to_string(&AdditionalInfoType::MultiSelection).unwrap(),
            "\"multi_selection\""
        );
    }

    #[test]
    fn test_additional_information() {
        let info = AdditionalInformation {
            info_id: "info789".to_string(),
            user_id: "user123".to_string(),
            activity_id: "act456".to_string(),
            field_name: "工作年限".to_string(),
            field_type: AdditionalInfoType::Number,
            field_value: "5".to_string(),
            created_at: Some(1703980800000),
            updated_at: Some(1703980800000),
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("info789"));
        assert!(json.contains("工作年限"));
        assert!(json.contains("number"));
    }

    #[test]
    fn test_user_group() {
        let group = UserGroup {
            group_id: "grp123".to_string(),
            name: "技术团队".to_string(),
            description: Some("研发部技术人员".to_string()),
            member_user_ids: Some(vec!["user1".to_string(), "user2".to_string()]),
            created_at: Some(1703980800000),
            updated_at: Some(1703980800000),
        };
        let json = serde_json::to_string(&group).unwrap();
        assert!(json.contains("grp123"));
        assert!(json.contains("技术团队"));
        assert!(json.contains("user1"));
    }

    #[test]
    fn test_reviewee() {
        let reviewee = Reviewee {
            user_id: "user456".to_string(),
            name: "张三".to_string(),
            email: Some("zhangsan@company.com".to_string()),
            department: Some("研发部".to_string()),
            position: Some("高级工程师".to_string()),
            manager_id: Some("manager123".to_string()),
            activity_id: "act456".to_string(),
            review_status: Some("in_progress".to_string()),
        };
        let json = serde_json::to_string(&reviewee).unwrap();
        assert!(json.contains("user456"));
        assert!(json.contains("张三"));
        assert!(json.contains("研发部"));
    }

    #[test]
    fn test_template_type_enum() {
        assert_eq!(
            serde_json::to_string(&TemplateType::SelfReview).unwrap(),
            "\"self_review\""
        );
        assert_eq!(
            serde_json::to_string(&TemplateType::ManagerReview).unwrap(),
            "\"manager_review\""
        );
        assert_eq!(
            serde_json::to_string(&TemplateType::PeerReview).unwrap(),
            "\"peer_review\""
        );
        assert_eq!(
            serde_json::to_string(&TemplateType::SubordinateReview).unwrap(),
            "\"subordinate_review\""
        );
    }

    #[test]
    fn test_review_template() {
        let template = ReviewTemplate {
            template_id: "tpl123".to_string(),
            name: "自评模板".to_string(),
            description: Some("员工自我评估模板".to_string()),
            template_type: TemplateType::SelfReview,
            activity_id: "act456".to_string(),
            enabled: Some(true),
            created_at: Some(1703980800000),
            updated_at: Some(1703980800000),
        };
        let json = serde_json::to_string(&template).unwrap();
        assert!(json.contains("tpl123"));
        assert!(json.contains("self_review"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_review_item_type_enum() {
        assert_eq!(
            serde_json::to_string(&ReviewItemType::Rating).unwrap(),
            "\"rating\""
        );
        assert_eq!(
            serde_json::to_string(&ReviewItemType::Text).unwrap(),
            "\"text\""
        );
        assert_eq!(
            serde_json::to_string(&ReviewItemType::SingleChoice).unwrap(),
            "\"single_choice\""
        );
        assert_eq!(
            serde_json::to_string(&ReviewItemType::MultipleChoice).unwrap(),
            "\"multiple_choice\""
        );
        assert_eq!(
            serde_json::to_string(&ReviewItemType::Tag).unwrap(),
            "\"tag\""
        );
    }

    #[test]
    fn test_review_item_rating() {
        let item = ReviewItem {
            item_id: "item123".to_string(),
            name: "工作质量评价".to_string(),
            description: Some("请对工作质量进行评分".to_string()),
            item_type: ReviewItemType::Rating,
            template_id: "tpl123".to_string(),
            weight: Some(20),
            required: Some(true),
            options: Some("{\"min\":1,\"max\":5}".to_string()),
        };
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("item123"));
        assert!(json.contains("rating"));
        assert!(json.contains("工作质量评价"));
    }

    #[test]
    fn test_tag_question_config() {
        let config = TagQuestionConfig {
            config_id: Some("cfg123".to_string()),
            tags: Some(vec!["团队合作".to_string(), "创新能力".to_string()]),
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("cfg123"));
        assert!(json.contains("团队合作"));
        assert!(json.contains("创新能力"));
    }

    #[test]
    fn test_metric_type_enum() {
        assert_eq!(
            serde_json::to_string(&MetricType::Number).unwrap(),
            "\"number\""
        );
        assert_eq!(
            serde_json::to_string(&MetricType::Percentage).unwrap(),
            "\"percentage\""
        );
        assert_eq!(
            serde_json::to_string(&MetricType::Text).unwrap(),
            "\"text\""
        );
        assert_eq!(
            serde_json::to_string(&MetricType::Boolean).unwrap(),
            "\"boolean\""
        );
    }

    #[test]
    fn test_metric() {
        let metric = Metric {
            metric_id: "met123".to_string(),
            name: "销售业绩".to_string(),
            description: Some("月度销售完成情况".to_string()),
            metric_type: MetricType::Number,
            unit: Some("万元".to_string()),
            is_key: Some(true),
            weight: Some(0.3),
            created_at: Some(1703980800000),
        };
        let json = serde_json::to_string(&metric).unwrap();
        assert!(json.contains("met123"));
        assert!(json.contains("销售业绩"));
        assert!(json.contains("number"));
    }

    #[test]
    fn test_task_status_enum() {
        assert_eq!(
            serde_json::to_string(&TaskStatus::NotStarted).unwrap(),
            "\"not_started\""
        );
        assert_eq!(
            serde_json::to_string(&TaskStatus::InProgress).unwrap(),
            "\"in_progress\""
        );
        assert_eq!(
            serde_json::to_string(&TaskStatus::Completed).unwrap(),
            "\"completed\""
        );
        assert_eq!(
            serde_json::to_string(&TaskStatus::Expired).unwrap(),
            "\"expired\""
        );
        assert_eq!(
            serde_json::to_string(&TaskStatus::Paused).unwrap(),
            "\"paused\""
        );
    }

    #[test]
    fn test_stage_task() {
        let task = StageTask {
            task_id: "task123".to_string(),
            name: "自评任务".to_string(),
            task_type: "self_review".to_string(),
            status: TaskStatus::InProgress,
            reviewee_id: "user456".to_string(),
            reviewer_id: None,
            activity_id: "act456".to_string(),
            semester_id: "sem123".to_string(),
            start_time: Some(1704067200000),
            end_time: Some(1706659200000),
            completed_at: None,
            created_at: Some(1703980800000),
        };
        let json = serde_json::to_string(&task).unwrap();
        assert!(json.contains("task123"));
        assert!(json.contains("in_progress"));
        assert!(json.contains("自评任务"));
    }

    #[test]
    fn test_performance_level_enum() {
        assert_eq!(
            serde_json::to_string(&PerformanceLevel::Excellent).unwrap(),
            "\"excellent\""
        );
        assert_eq!(
            serde_json::to_string(&PerformanceLevel::Good).unwrap(),
            "\"good\""
        );
        assert_eq!(
            serde_json::to_string(&PerformanceLevel::Average).unwrap(),
            "\"average\""
        );
        assert_eq!(
            serde_json::to_string(&PerformanceLevel::NeedsImprovement).unwrap(),
            "\"needs_improvement\""
        );
        assert_eq!(
            serde_json::to_string(&PerformanceLevel::Unsatisfactory).unwrap(),
            "\"unsatisfactory\""
        );
    }

    #[test]
    fn test_performance_result() {
        let result = PerformanceResult {
            result_id: "res123".to_string(),
            reviewee_id: "user456".to_string(),
            activity_id: "act456".to_string(),
            semester_id: "sem123".to_string(),
            level: Some(PerformanceLevel::Excellent),
            score: Some(4.5),
            rank: Some(3),
            overall_comment: Some("表现优秀，继续保持".to_string()),
            result_opened: Some(true),
            opened_at: Some(1706745600000),
            created_at: Some(1703980800000),
            updated_at: Some(1706745600000),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("res123"));
        assert!(json.contains("excellent"));
        assert!(json.contains("4.5"));
    }

    #[test]
    fn test_review_detail() {
        let detail = ReviewDetail {
            detail_id: "det123".to_string(),
            reviewee_id: "user456".to_string(),
            reviewer_id: "reviewer789".to_string(),
            activity_id: "act456".to_string(),
            item_id: "item123".to_string(),
            content: "工作完成质量高，团队协作能力强".to_string(),
            score: Some(4.0),
            submitted_at: Some(1706659200000),
            created_at: Some(1703980800000),
        };
        let json = serde_json::to_string(&detail).unwrap();
        assert!(json.contains("det123"));
        assert!(json.contains("工作完成质量高"));
        assert!(json.contains("4.0"));
    }

    #[test]
    fn test_minimal_structs() {
        let minimal_semester = Semester {
            semester_id: "sem_min".to_string(),
            name: "最小周期".to_string(),
            description: None,
            status: None,
            start_time: None,
            end_time: None,
            created_at: None,
            updated_at: None,
        };
        let json = serde_json::to_string(&minimal_semester).unwrap();
        assert!(json.contains("sem_min"));
        assert!(!json.contains("description"));
    }
}
