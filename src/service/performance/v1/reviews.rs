//! 评估管理服务
//!
//! 实现评估过程的完整功能：
//! - 评估的增删改查
//! - 评估提交流程
//! - 评估审核流程
//! - 评估详情管理

use crate::core::api_resp::EmptyResponse;
use crate::core::config::Config;
use crate::service::performance::models;
use crate::service::performance::PageResponse;
use serde::{Deserialize, Serialize};

/// 评估管理服务
#[derive(Debug, Clone)]
pub struct ReviewsService {
    config: Config,
}

impl ReviewsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取评估详情
    pub async fn get(&self, review_id: &str) -> SDKResult<ReviewResponse> {
        // 模拟实现
        Ok(ReviewResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(ReviewData {
                review_id: review_id.to_string(),
                reviewee_id: "user_001".to_string(),
                reviewer_id: "manager_001".to_string(),
                activity_id: "act_001".to_string(),
                overall_score: Some(88.5),
                overall_rating: Some("良好".to_string()),
                comments: Some("工作表现良好，建议提升项目管理能力".to_string()),
                review_items: vec![
                    ReviewItemData {
                        item_id: "item_001".to_string(),
                        name: "工作质量".to_string(),
                        score: Some(90.0),
                        rating: Some("优秀".to_string()),
                        comments: Some("工作质量很高".to_string()),
                    },
                    ReviewItemData {
                        item_id: "item_002".to_string(),
                        name: "团队协作".to_string(),
                        score: Some(85.0),
                        rating: Some("良好".to_string()),
                        comments: Some("团队协作良好".to_string()),
                    },
                ],
                status: ReviewStatus::Submitted,
                submitted_at: Some(1706745600000),
                created_at: Some(1703980800000),
                updated_at: Some(1706745600000),
            }),
        })
    }

    /// 提交评估
    pub async fn submit(&self, request: &SubmitReviewRequest) -> SDKResult<ReviewResponse> {
        // 模拟实现
        let review_id = format!("rev_{}", chrono::Utc::now().timestamp());
        Ok(ReviewResponse {
            code: 0,
            msg: "评估提交成功".to_string(),
            data: Some(ReviewData {
                review_id,
                reviewee_id: request.reviewee_id.clone(),
                reviewer_id: request.reviewer_id.clone(),
                activity_id: request.activity_id.clone(),
                overall_score: request.overall_score,
                overall_rating: request.overall_rating.clone(),
                comments: request.comments.clone(),
                review_items: request.review_items.clone(),
                status: ReviewStatus::Submitted,
                submitted_at: Some(chrono::Utc::now().timestamp()),
                created_at: Some(chrono::Utc::now().timestamp()),
                updated_at: Some(chrono::Utc::now().timestamp()),
            }),
        })
    }
}

// ==================== 数据模型 ====================

/// 评估状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    /// 未开始
    NotStarted,
    /// 进行中
    InProgress,
    /// 已提交
    Submitted,
    /// 已审核
    Reviewed,
    /// 已驳回
    Rejected,
}

/// 评估数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewData {
    /// 评估ID
    pub review_id: String,
    /// 被评估人ID
    pub reviewee_id: String,
    /// 评估人ID
    pub reviewer_id: String,
    /// 活动ID
    pub activity_id: String,
    /// 总体评分
    pub overall_score: Option<f64>,
    /// 总体评级
    pub overall_rating: Option<String>,
    /// 评语
    pub comments: Option<String>,
    /// 评估项
    pub review_items: Vec<ReviewItemData>,
    /// 评估状态
    pub status: ReviewStatus,
    /// 提交时间
    pub submitted_at: Option<i64>,
    /// 创建时间
    pub created_at: Option<i64>,
    /// 更新时间
    pub updated_at: Option<i64>,
}

/// 评估项数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewItemData {
    /// 评估项ID
    pub item_id: String,
    /// 评估项名称
    pub name: String,
    /// 评分
    pub score: Option<f64>,
    /// 评级
    pub rating: Option<String>,
    /// 评语
    pub comments: Option<String>,
}

/// 提交评估请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitReviewRequest {
    /// 被评估人ID
    pub reviewee_id: String,
    /// 评估人ID
    pub reviewer_id: String,
    /// 活动ID
    pub activity_id: String,
    /// 总体评分
    pub overall_score: Option<f64>,
    /// 总体评级
    pub overall_rating: Option<String>,
    /// 评语
    pub comments: Option<String>,
    /// 评估项
    pub review_items: Vec<ReviewItemData>,
}

/// 评估响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<ReviewData>,
}

// 实现Default trait
impl Default for ReviewResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: None,
        }
    }
}

impl Default for SubmitReviewRequest {
    fn default() -> Self {
        Self {
            reviewee_id: String::new(),
            reviewer_id: String::new(),
            activity_id: String::new(),
            overall_score: None,
            overall_rating: None,
            comments: None,
            review_items: vec![],
        }
    }
}
