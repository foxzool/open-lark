use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============ 员工花名册相关结构 ============

/// 批量获取员工花名册信息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeListRequest {
    /// 分页大小，最大值100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记，当has_more为true时，会同时返回新的page_token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 员工状态筛选，不传则返回所有状态的员工
    /// 可选值：active(在职)、inactive(离职)、trial(试用期)等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 部门ID，用于筛选特定部门的员工
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 用户ID类型，支持open_id、union_id、user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型，支持open_department_id、department_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 是否包含离职员工，默认false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resigned: Option<bool>,
    /// 查询的字段列表，不传则返回所有字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}

/// 员工信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    /// 员工ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 用户ID（根据user_id_type返回对应类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 员工工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    /// 员工姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 英文姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// 员工邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// 生日
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    /// 身份证号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    /// 员工状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EmployeeStatus>,
    /// 部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_info: Option<EmployeeDepartment>,
    /// 职位信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_info: Option<EmployeeJob>,
    /// 入职信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_info: Option<EmployeeHire>,
    /// 个人信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_info: Option<EmployeePersonal>,
    /// 教育经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_info: Option<Vec<EmployeeEducation>>,
    /// 工作经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_experience: Option<Vec<EmployeeWorkExperience>>,
    /// 紧急联系人信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_contact: Option<Vec<EmergencyContact>>,
    /// 银行卡信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<Vec<BankAccount>>,
    /// 社保信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_security: Option<SocialSecurity>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
}

/// 员工状态信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeStatus {
    /// 员工状态：active(在职)、inactive(离职)、trial(试用期)等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 状态生效日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 离职日期（如果是离职状态）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resignation_date: Option<String>,
    /// 离职原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resignation_reason: Option<String>,
}

/// 员工部门信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeDepartment {
    /// 部门ID（根据department_id_type返回对应类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    /// 上级部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_path: Option<String>,
}

/// 员工职位信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeJob {
    /// 职位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// 职级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level: Option<String>,
    /// 职位序列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family: Option<String>,
    /// 直属上级用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supervisor_id: Option<String>,
    /// 直属上级姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supervisor_name: Option<String>,
    /// 员工类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<String>,
    /// 工作地点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location: Option<String>,
}

/// 员工入职信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeHire {
    /// 入职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    /// 试用期结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    /// 合同类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<String>,
    /// 合同开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_start_date: Option<String>,
    /// 合同结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_end_date: Option<String>,
}

/// 员工个人信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeePersonal {
    /// 国籍
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// 民族
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<String>,
    /// 婚姻状况
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<String>,
    /// 政治面貌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_status: Option<String>,
    /// 户籍地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<String>,
    /// 现居住地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_address: Option<String>,
    /// 毕业院校
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduate_school: Option<String>,
    /// 最高学历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_education: Option<String>,
}

/// 教育经历
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeEducation {
    /// 学校名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school_name: Option<String>,
    /// 专业
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    /// 学历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

/// 工作经历
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeWorkExperience {
    /// 公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// 职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 工作描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 紧急联系人
#[derive(Debug, Serialize, Deserialize)]
pub struct EmergencyContact {
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 关系
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    /// 电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// 银行卡信息
#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    /// 银行名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// 银行卡号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// 开户行
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// 账户类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
}

/// 社保信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialSecurity {
    /// 社保号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_security_number: Option<String>,
    /// 公积金号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub housing_fund_number: Option<String>,
    /// 社保缴纳地
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_security_location: Option<String>,
}

/// 分页响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<T>>,
}

// ============ 附件下载相关结构 ============

/// 下载人员附件请求
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeAttachmentRequest {
    /// 员工ID
    pub employee_id: String,
    /// 附件ID
    pub attachment_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 附件信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeAttachment {
    /// 附件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// 附件名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 附件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 附件大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 附件分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 上传时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_time: Option<String>,
    /// 附件描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 附件下载响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentDownloadResponse {
    /// 文件内容（base64编码）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_content: Option<String>,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

// ============ 通用响应结构 ============

/// EHR响应包装器
#[derive(Debug, Serialize, Deserialize)]
pub struct EhrResponse<T> {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// 错误码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_employee_list_request_default() {
        let request = EmployeeListRequest {
            page_size: Some(50),
            page_token: None,
            status: Some("active".to_string()),
            department_id: None,
            user_id_type: Some("open_id".to_string()),
            department_id_type: None,
            include_resigned: Some(false),
            fields: Some(vec!["employee_id".to_string(), "name".to_string()]),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("50"));
        assert!(json.contains("active"));
        assert!(json.contains("open_id"));
        assert!(json.contains("employee_id"));
    }

    #[test]
    fn test_employee_list_request_minimal() {
        let request = EmployeeListRequest {
            page_size: None,
            page_token: None,
            status: None,
            department_id: None,
            user_id_type: None,
            department_id_type: None,
            include_resigned: None,
            fields: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_employee_complete() {
        let employee = Employee {
            employee_id: Some("emp123".to_string()),
            user_id: Some("usr456".to_string()),
            employee_number: Some("E001".to_string()),
            name: Some("张三".to_string()),
            en_name: Some("Zhang San".to_string()),
            email: Some("zhangsan@company.com".to_string()),
            mobile: Some("+86-13800138000".to_string()),
            gender: Some("male".to_string()),
            birthday: Some("1990-05-20".to_string()),
            id_number: Some("110101199005200001".to_string()),
            status: Some(EmployeeStatus {
                status: Some("active".to_string()),
                effective_date: Some("2023-01-15".to_string()),
                resignation_date: None,
                resignation_reason: None,
            }),
            department_info: Some(EmployeeDepartment {
                department_id: Some("dept789".to_string()),
                department_name: Some("技术部".to_string()),
                parent_department_id: Some("company".to_string()),
                department_path: Some("/company/tech".to_string()),
            }),
            job_info: Some(EmployeeJob {
                job_title: Some("软件工程师".to_string()),
                job_level: Some("L3".to_string()),
                job_family: Some("技术序列".to_string()),
                supervisor_id: Some("mgr001".to_string()),
                supervisor_name: Some("张经理".to_string()),
                employee_type: Some("full_time".to_string()),
                work_location: Some("北京".to_string()),
            }),
            hire_info: None,
            personal_info: None,
            education_info: None,
            work_experience: None,
            emergency_contact: None,
            bank_account: None,
            social_security: None,
            create_time: Some("2023-01-15T00:00:00Z".to_string()),
            update_time: None,
            custom_fields: Some(HashMap::new()),
        };
        let json = serde_json::to_string(&employee).unwrap();
        assert!(json.contains("emp123"));
        assert!(json.contains("张三"));
        assert!(json.contains("zhangsan@company.com"));
        assert!(json.contains("active"));
        assert!(json.contains("技术部"));
    }

    #[test]
    fn test_employee_status_active() {
        let status = EmployeeStatus {
            status: Some("active".to_string()),
            effective_date: Some("2023-01-01".to_string()),
            resignation_date: None,
            resignation_reason: None,
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("active"));
        assert!(json.contains("2023-01-01"));
        assert!(!json.contains("resignation_date"));
    }

    #[test]
    fn test_employee_status_resigned() {
        let status = EmployeeStatus {
            status: Some("inactive".to_string()),
            effective_date: Some("2023-01-01".to_string()),
            resignation_date: Some("2024-01-01".to_string()),
            resignation_reason: Some("career_change".to_string()),
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("inactive"));
        assert!(json.contains("2024-01-01"));
        assert!(json.contains("career_change"));
    }

    #[test]
    fn test_employee_department_hierarchy() {
        let department = EmployeeDepartment {
            department_id: Some("tech001".to_string()),
            department_name: Some("前端开发组".to_string()),
            parent_department_id: Some("tech".to_string()),
            department_path: Some("/company/tech/frontend".to_string()),
        };
        let json = serde_json::to_string(&department).unwrap();
        assert!(json.contains("tech001"));
        assert!(json.contains("前端开发组"));
        assert!(json.contains("tech"));
        assert!(json.contains("frontend"));
    }

    #[test]
    fn test_employee_job_complete() {
        let job = EmployeeJob {
            job_title: Some("高级软件工程师".to_string()),
            job_level: Some("L4".to_string()),
            job_family: Some("技术序列".to_string()),
            supervisor_id: Some("mgr002".to_string()),
            supervisor_name: Some("李经理".to_string()),
            employee_type: Some("full_time".to_string()),
            work_location: Some("上海".to_string()),
        };
        let json = serde_json::to_string(&job).unwrap();
        assert!(json.contains("高级软件工程师"));
        assert!(json.contains("L4"));
        assert!(json.contains("技术序列"));
        assert!(json.contains("上海"));
    }

    #[test]
    fn test_employee_hire_info() {
        let hire = EmployeeHire {
            hire_date: Some("2023-01-15".to_string()),
            probation_end_date: Some("2023-04-15".to_string()),
            contract_type: Some("permanent".to_string()),
            contract_start_date: Some("2023-01-15".to_string()),
            contract_end_date: None,
        };
        let json = serde_json::to_string(&hire).unwrap();
        assert!(json.contains("2023-01-15"));
        assert!(json.contains("2023-04-15"));
        assert!(json.contains("permanent"));
    }

    #[test]
    fn test_employee_personal_info() {
        let personal = EmployeePersonal {
            nationality: Some("中国".to_string()),
            ethnicity: Some("汉".to_string()),
            marital_status: Some("married".to_string()),
            political_status: Some("party_member".to_string()),
            registered_address: Some("北京市朝阳区".to_string()),
            current_address: Some("上海市浦东区".to_string()),
            graduate_school: Some("清华大学".to_string()),
            highest_education: Some("bachelor".to_string()),
        };
        let json = serde_json::to_string(&personal).unwrap();
        assert!(json.contains("中国"));
        assert!(json.contains("married"));
        assert!(json.contains("北京市朝阳区"));
        assert!(json.contains("清华大学"));
    }

    #[test]
    fn test_employee_education() {
        let education = EmployeeEducation {
            school_name: Some("清华大学".to_string()),
            major: Some("计算机科学与技术".to_string()),
            degree: Some("bachelor".to_string()),
            start_date: Some("2008-09-01".to_string()),
            end_date: Some("2012-06-30".to_string()),
        };
        let json = serde_json::to_string(&education).unwrap();
        assert!(json.contains("bachelor"));
        assert!(json.contains("清华大学"));
        assert!(json.contains("计算机科学与技术"));
        assert!(json.contains("2008-09-01"));
    }

    #[test]
    fn test_employee_work_experience() {
        let experience = EmployeeWorkExperience {
            company_name: Some("阿里巴巴".to_string()),
            position: Some("软件工程师".to_string()),
            start_date: Some("2019-07-01".to_string()),
            end_date: Some("2022-12-31".to_string()),
            description: Some("负责电商平台后端开发".to_string()),
        };
        let json = serde_json::to_string(&experience).unwrap();
        assert!(json.contains("阿里巴巴"));
        assert!(json.contains("软件工程师"));
        assert!(json.contains("负责电商平台后端开发"));
    }

    #[test]
    fn test_emergency_contact() {
        let contact = EmergencyContact {
            name: Some("李四".to_string()),
            relationship: Some("spouse".to_string()),
            phone: Some("+86-13900139000".to_string()),
            address: Some("北京市海淀区".to_string()),
        };
        let json = serde_json::to_string(&contact).unwrap();
        assert!(json.contains("李四"));
        assert!(json.contains("spouse"));
        assert!(json.contains("13900139000"));
        assert!(json.contains("海淀区"));
    }

    #[test]
    fn test_employee_list_response() {
        let create_minimal_employee = |id: &str, name: &str, email: &str| Employee {
            employee_id: Some(id.to_string()),
            user_id: None,
            employee_number: None,
            name: Some(name.to_string()),
            en_name: None,
            email: Some(email.to_string()),
            mobile: None,
            gender: None,
            birthday: None,
            id_number: None,
            status: Some(EmployeeStatus {
                status: Some("active".to_string()),
                effective_date: Some("2023-01-01".to_string()),
                resignation_date: None,
                resignation_reason: None,
            }),
            department_info: None,
            job_info: None,
            hire_info: None,
            personal_info: None,
            education_info: None,
            work_experience: None,
            emergency_contact: None,
            bank_account: None,
            social_security: None,
            create_time: None,
            update_time: None,
            custom_fields: None,
        };

        let response = EhrResponse {
            data: Some(vec![
                create_minimal_employee("emp001", "员工一", "emp1@company.com"),
                create_minimal_employee("emp002", "员工二", "emp2@company.com"),
            ]),
            code: Some(0),
            msg: Some("success".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("emp001"));
        assert!(json.contains("员工一"));
        assert!(json.contains("emp2@company.com"));
        assert!(json.contains("success"));
    }

    #[test]
    fn test_ehr_response_error() {
        let response: EhrResponse<Employee> = EhrResponse {
            data: None,
            code: Some(400),
            msg: Some("Invalid request".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("400"));
        assert!(json.contains("Invalid request"));
        assert!(!json.contains("data"));
    }
}
