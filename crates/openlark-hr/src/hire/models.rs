use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 分页响应通用结构
#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页的分页标记
    pub page_token: Option<String>,
}

/// I18n 多语言文本
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct I18nText {
    /// 中文
    pub zh_cn: Option<String>,
    /// 英文
    pub en_us: Option<String>,
    /// 日文
    pub ja_jp: Option<String>,
}

/// 用户ID对象
#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    /// 用户ID
    pub id: String,
    /// 用户ID类型
    pub id_type: String,
}

/// 部门ID对象
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentId {
    /// 部门ID
    pub id: String,
    /// 部门ID类型
    pub id_type: String,
}

/// 附件信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    /// 附件ID
    pub id: String,
    /// 附件名称
    pub name: String,
    /// 附件类型
    pub file_type: Option<String>,
    /// 附件大小
    pub size: Option<u64>,
    /// 创建时间
    pub created_time: Option<String>,
}

/// 地址信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    /// 地址ID
    pub id: String,
    /// 地址名称
    pub name: I18nText,
    /// 地址类型
    pub location_type: String,
    /// 父级地址ID
    pub parent_id: Option<String>,
    /// 地址代码
    pub code: Option<String>,
    /// 活跃状态
    pub active_status: bool,
}

/// 地址查询请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationQueryRequest {
    /// 地址类型
    pub location_type: Option<String>,
    /// 父级地址ID
    pub parent_id: Option<String>,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 角色信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    /// 角色ID
    pub id: String,
    /// 角色名称
    pub name: I18nText,
    /// 角色描述
    pub description: Option<I18nText>,
    /// 角色权限列表
    pub permissions: Vec<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 角色列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RoleListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 用户角色信息
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    /// 用户ID
    pub user_id: String,
    /// 角色ID列表
    pub role_ids: Vec<String>,
    /// 角色详情列表
    pub roles: Option<Vec<Role>>,
}

/// 职位基本信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    /// 职位ID
    pub id: String,
    /// 职位名称
    pub title: String,
    /// 职位描述
    pub description: Option<String>,
    /// 职位要求
    pub requirement: Option<String>,
    /// 职位状态
    pub status: String,
    /// 职位类型
    pub job_type: Option<String>,
    /// 职能分类ID
    pub job_function_id: Option<String>,
    /// 职位类别ID
    pub job_category_id: Option<String>,
    /// 工作地点
    pub locations: Vec<Location>,
    /// 部门ID
    pub department_id: Option<String>,
    /// 招聘人员
    pub recruiters: Vec<UserId>,
    /// 面试官
    pub interviewers: Vec<UserId>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
    /// 自定义字段
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
}

/// 职位创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JobCreateRequest {
    /// 职位名称
    pub title: String,
    /// 职位描述
    pub description: Option<String>,
    /// 职位要求
    pub requirement: Option<String>,
    /// 职位类型
    pub job_type: Option<String>,
    /// 职能分类ID
    pub job_function_id: Option<String>,
    /// 职位类别ID
    pub job_category_id: Option<String>,
    /// 工作地点ID列表
    pub location_ids: Vec<String>,
    /// 部门ID
    pub department_id: Option<String>,
    /// 招聘人员ID列表
    pub recruiter_ids: Vec<String>,
    /// 面试官ID列表
    pub interviewer_ids: Vec<String>,
    /// 自定义字段
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
}

/// 职位更新请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JobUpdateRequest {
    /// 职位ID
    pub job_id: String,
    /// 职位名称
    pub title: Option<String>,
    /// 职位描述
    pub description: Option<String>,
    /// 职位要求
    pub requirement: Option<String>,
    /// 职位类型
    pub job_type: Option<String>,
    /// 职能分类ID
    pub job_function_id: Option<String>,
    /// 职位类别ID
    pub job_category_id: Option<String>,
    /// 工作地点ID列表
    pub location_ids: Option<Vec<String>>,
    /// 部门ID
    pub department_id: Option<String>,
    /// 招聘人员ID列表
    pub recruiter_ids: Option<Vec<String>>,
    /// 面试官ID列表
    pub interviewer_ids: Option<Vec<String>>,
    /// 自定义字段
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
}

/// 职位列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JobListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 职位状态
    pub status: Option<String>,
    /// 部门ID
    pub department_id: Option<String>,
    /// 职位类型
    pub job_type: Option<String>,
    /// 创建时间开始
    pub created_start_time: Option<String>,
    /// 创建时间结束
    pub created_end_time: Option<String>,
}

/// 招聘需求信息
#[derive(Debug, Serialize, Deserialize)]
pub struct JobRequirement {
    /// 需求ID
    pub id: String,
    /// 需求名称
    pub name: String,
    /// 需求描述
    pub description: Option<String>,
    /// 关联职位ID
    pub job_id: String,
    /// 需求人数
    pub headcount: u32,
    /// 需求状态
    pub status: String,
    /// 期望入职时间
    pub expected_entry_time: Option<String>,
    /// 创建人
    pub creator: Option<UserId>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 招聘需求创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JobRequirementCreateRequest {
    /// 需求名称
    pub name: String,
    /// 需求描述
    pub description: Option<String>,
    /// 关联职位ID
    pub job_id: String,
    /// 需求人数
    pub headcount: u32,
    /// 期望入职时间
    pub expected_entry_time: Option<String>,
}

/// 人才基本信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Talent {
    /// 人才ID
    pub id: String,
    /// 姓名
    pub name: String,
    /// 邮箱
    pub email: Option<String>,
    /// 电话
    pub phone: Option<String>,
    /// 性别
    pub gender: Option<String>,
    /// 生日
    pub birthday: Option<String>,
    /// 工作年限
    pub work_experience: Option<u32>,
    /// 学历
    pub education: Option<String>,
    /// 当前公司
    pub current_company: Option<String>,
    /// 当前职位
    pub current_position: Option<String>,
    /// 期望薪资
    pub expected_salary: Option<String>,
    /// 简历附件
    pub resume_attachments: Vec<Attachment>,
    /// 人才标签
    pub tags: Vec<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
    /// 自定义字段
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
}

/// 人才创建请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TalentCreateRequest {
    /// 姓名
    pub name: String,
    /// 邮箱
    pub email: Option<String>,
    /// 电话
    pub phone: Option<String>,
    /// 性别
    pub gender: Option<String>,
    /// 生日
    pub birthday: Option<String>,
    /// 工作年限
    pub work_experience: Option<u32>,
    /// 学历
    pub education: Option<String>,
    /// 当前公司
    pub current_company: Option<String>,
    /// 当前职位
    pub current_position: Option<String>,
    /// 期望薪资
    pub expected_salary: Option<String>,
    /// 简历附件ID列表
    pub resume_attachment_ids: Vec<String>,
    /// 人才标签
    pub tags: Vec<String>,
    /// 自定义字段
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
}

/// 投递信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    /// 投递ID
    pub id: String,
    /// 人才ID
    pub talent_id: String,
    /// 职位ID
    pub job_id: String,
    /// 当前阶段ID
    pub stage_id: String,
    /// 投递状态
    pub status: String,
    /// 投递渠道
    pub source: Option<String>,
    /// 投递时间
    pub apply_time: Option<String>,
    /// 人才信息
    pub talent: Option<Talent>,
    /// 职位信息
    pub job: Option<Job>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 投递创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApplicationCreateRequest {
    /// 人才ID
    pub talent_id: String,
    /// 职位ID
    pub job_id: String,
    /// 阶段ID
    pub stage_id: String,
    /// 投递渠道
    pub source: Option<String>,
    /// 投递时间
    pub apply_time: Option<String>,
}

/// 投递列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApplicationListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 职位ID
    pub job_id: Option<String>,
    /// 投递状态
    pub status: Option<String>,
    /// 阶段ID
    pub stage_id: Option<String>,
    /// 投递渠道
    pub source: Option<String>,
    /// 创建时间开始
    pub created_start_time: Option<String>,
    /// 创建时间结束
    pub created_end_time: Option<String>,
}

/// 面试信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Interview {
    /// 面试ID
    pub id: String,
    /// 投递ID
    pub application_id: String,
    /// 面试轮次
    pub round: u32,
    /// 面试类型
    pub interview_type: String,
    /// 面试状态
    pub status: String,
    /// 面试开始时间
    pub start_time: Option<String>,
    /// 面试结束时间
    pub end_time: Option<String>,
    /// 面试官列表
    pub interviewers: Vec<UserId>,
    /// 面试地点
    pub location: Option<String>,
    /// 面试备注
    pub remark: Option<String>,
    /// 面试结果
    pub result: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// Offer信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
    /// Offer ID
    pub id: String,
    /// 投递ID
    pub application_id: String,
    /// Offer状态
    pub status: String,
    /// 职位级别
    pub job_level: Option<String>,
    /// 基本薪资
    pub basic_salary: Option<String>,
    /// 绩效奖金
    pub performance_bonus: Option<String>,
    /// 股票期权
    pub stock_option: Option<String>,
    /// 入职时间
    pub onboard_date: Option<String>,
    /// 过期时间
    pub expire_time: Option<String>,
    /// Offer备注
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
    /// 自定义字段
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
}

/// Offer创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OfferCreateRequest {
    /// 投递ID
    pub application_id: String,
    /// 职位级别
    pub job_level: Option<String>,
    /// 基本薪资
    pub basic_salary: Option<String>,
    /// 绩效奖金
    pub performance_bonus: Option<String>,
    /// 股票期权
    pub stock_option: Option<String>,
    /// 入职时间
    pub onboard_date: Option<String>,
    /// 过期时间
    pub expire_time: Option<String>,
    /// Offer备注
    pub remark: Option<String>,
    /// 自定义字段
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
}

/// 内推账户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralAccount {
    /// 账户ID
    pub id: String,
    /// 用户ID
    pub user_id: String,
    /// 账户状态
    pub status: String,
    /// 账户余额
    pub balance: String,
    /// 总收入
    pub total_income: String,
    /// 已提现金额
    pub withdrawn_amount: String,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 内推账户注册请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReferralAccountCreateRequest {
    /// 用户ID
    pub user_id: String,
    /// 身份证号
    pub id_card: Option<String>,
    /// 银行卡号
    pub bank_card: Option<String>,
    /// 开户行
    pub bank_name: Option<String>,
}

/// 附件创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AttachmentCreateRequest {
    /// 文件名
    pub name: String,
    /// 文件内容(base64编码)
    pub content: String,
    /// 文件类型
    pub file_type: Option<String>,
}

/// 通用响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonResponse {
    /// 操作是否成功
    pub success: bool,
    /// 操作消息
    pub message: Option<String>,
    /// 操作时间
    pub timestamp: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_page_response_serialization() {
        let response = PageResponse {
            items: vec!["item1".to_string(), "item2".to_string()],
            has_more: true,
            page_token: Some("token123".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("item1"));
        assert!(json.contains("has_more"));
        assert!(json.contains("token123"));
    }

    #[test]
    fn test_i18n_text_complete() {
        let text = I18nText {
            zh_cn: Some("中文".to_string()),
            en_us: Some("English".to_string()),
            ja_jp: Some("日本語".to_string()),
        };
        let json = serde_json::to_string(&text).unwrap();
        let deserialized: I18nText = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.zh_cn, Some("中文".to_string()));
        assert_eq!(deserialized.en_us, Some("English".to_string()));
        assert_eq!(deserialized.ja_jp, Some("日本語".to_string()));
    }

    #[test]
    fn test_i18n_text_default() {
        let text = I18nText::default();
        assert_eq!(text.zh_cn, None);
        assert_eq!(text.en_us, None);
        assert_eq!(text.ja_jp, None);
    }

    #[test]
    fn test_user_id_serialization() {
        let user_id = UserId {
            id: "user123".to_string(),
            id_type: "open_id".to_string(),
        };
        let json = serde_json::to_string(&user_id).unwrap();
        assert!(json.contains("user123"));
        assert!(json.contains("open_id"));
    }

    #[test]
    fn test_department_id_serialization() {
        let dept_id = DepartmentId {
            id: "dept456".to_string(),
            id_type: "department_id".to_string(),
        };
        let json = serde_json::to_string(&dept_id).unwrap();
        assert!(json.contains("dept456"));
        assert!(json.contains("department_id"));
    }

    #[test]
    fn test_attachment_complete() {
        let attachment = Attachment {
            id: "att789".to_string(),
            name: "resume.pdf".to_string(),
            file_type: Some("application/pdf".to_string()),
            size: Some(1024000),
            created_time: Some("2024-01-01T00:00:00Z".to_string()),
        };
        let json = serde_json::to_string(&attachment).unwrap();
        assert!(json.contains("att789"));
        assert!(json.contains("resume.pdf"));
        assert!(json.contains("application/pdf"));
    }

    #[test]
    fn test_location_active() {
        let location = Location {
            id: "loc001".to_string(),
            name: I18nText {
                zh_cn: Some("北京".to_string()),
                en_us: Some("Beijing".to_string()),
                ja_jp: None,
            },
            location_type: "city".to_string(),
            parent_id: Some("china".to_string()),
            code: Some("BJ".to_string()),
            active_status: true,
        };
        let json = serde_json::to_string(&location).unwrap();
        assert!(json.contains("loc001"));
        assert!(json.contains("北京"));
        assert!(json.contains("Beijing"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_location_query_request_default() {
        let request = LocationQueryRequest::default();
        assert_eq!(request.location_type, None);
        assert_eq!(request.parent_id, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_common_response_success() {
        let response = CommonResponse {
            success: true,
            message: Some("Operation completed".to_string()),
            timestamp: Some("2024-01-01T00:00:00Z".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("true"));
        assert!(json.contains("Operation completed"));
    }

    #[test]
    fn test_common_response_error() {
        let response = CommonResponse {
            success: false,
            message: Some("Operation failed".to_string()),
            timestamp: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("false"));
        assert!(json.contains("Operation failed"));
    }

    #[test]
    fn test_attachment_create_request() {
        let request = AttachmentCreateRequest {
            name: "document.pdf".to_string(),
            content: "base64encodedcontent".to_string(),
            file_type: Some("application/pdf".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("document.pdf"));
        assert!(json.contains("base64encodedcontent"));
        assert!(json.contains("application/pdf"));
    }
}
