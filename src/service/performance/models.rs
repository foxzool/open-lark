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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagQuestionConfig {
    /// 配置ID
    pub config_id: String,
    /// 评估项ID
    pub item_id: String,
    /// 标签选项列表
    pub tag_options: Vec<String>,
    /// 最小选择数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_selection: Option<i32>,
    /// 最大选择数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selection: Option<i32>,
    /// 是否允许自定义标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_custom: Option<bool>,
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
