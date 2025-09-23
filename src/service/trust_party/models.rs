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

/// 关联组织信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationOrganization {
    /// 关联组织ID
    pub org_id: String,
    /// 关联组织名称
    pub org_name: String,
    /// 关联组织类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_type: Option<String>,
    /// 关联组织状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 关联关系创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 关联关系更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// 关联组织域名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 关联组织描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 关联组织部门信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationDepartment {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 父部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 部门层级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// 部门成员数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 关联组织成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationUser {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 用户头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// 用户状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 用户类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    /// 所属部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 职位信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 组织架构信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationStructure {
    /// 部门列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departments: Option<Vec<CollaborationDepartment>>,
    /// 成员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<CollaborationUser>>,
}

/// 共享成员范围信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedMemberScope {
    /// 共享范围类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// 共享的部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 共享的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 共享范围描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 可搜可见规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchableVisibleRule {
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
    /// 适用的组织ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    /// 规则配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<RuleConfig>,
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

/// 规则配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleConfig {
    /// 可见性（如 public/restricted/private）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// 是否可搜索
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
    /// 适用范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<RuleScope>,
    /// 例外对象（用户/部门/组等标识）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<Vec<String>>,
}

/// 规则适用范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleScope {
    /// 适用的部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 适用的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 适用的用户组ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_page_response() {
        let response = PageResponse {
            items: vec!["org1".to_string(), "org2".to_string()],
            page_token: Some("token_123".to_string()),
            has_more: Some(true),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("org1"));
        assert!(json.contains("org2"));
        assert!(json.contains("token_123"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_collaboration_organization() {
        let org = CollaborationOrganization {
            org_id: "org_12345".to_string(),
            org_name: "合作伙伴公司".to_string(),
            org_type: Some("partner".to_string()),
            status: Some("active".to_string()),
            created_at: Some(1640995200),
            updated_at: Some(1640995200),
            domain: Some("partner.com".to_string()),
            description: Some("重要合作伙伴公司".to_string()),
        };
        let json = serde_json::to_string(&org).unwrap();
        assert!(json.contains("org_12345"));
        assert!(json.contains("合作伙伴公司"));
        assert!(json.contains("partner"));
        assert!(json.contains("active"));
        assert!(json.contains("partner.com"));
        assert!(json.contains("重要合作伙伴公司"));
    }

    #[test]
    fn test_collaboration_department() {
        let dept = CollaborationDepartment {
            department_id: "dept_engineering".to_string(),
            name: "工程部".to_string(),
            parent_department_id: Some("dept_root".to_string()),
            status: Some("active".to_string()),
            level: Some(2),
            member_count: Some(25),
            created_at: Some(1640995200),
            updated_at: Some(1640995200),
        };
        let json = serde_json::to_string(&dept).unwrap();
        assert!(json.contains("dept_engineering"));
        assert!(json.contains("工程部"));
        assert!(json.contains("dept_root"));
        assert!(json.contains("active"));
        assert!(json.contains("\"level\":2"));
        assert!(json.contains("\"member_count\":25"));
    }

    #[test]
    fn test_collaboration_user() {
        let user = CollaborationUser {
            user_id: "user_alice".to_string(),
            name: "Alice Wang".to_string(),
            email: Some("alice.wang@partner.com".to_string()),
            mobile: Some("+86-138-0000-0000".to_string()),
            avatar_url: Some("https://cdn.partner.com/avatars/alice.jpg".to_string()),
            status: Some("active".to_string()),
            user_type: Some("external".to_string()),
            department_ids: Some(vec![
                "dept_engineering".to_string(),
                "dept_product".to_string(),
            ]),
            job_title: Some("高级工程师".to_string()),
            created_at: Some(1640995200),
            updated_at: Some(1640995200),
        };
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("user_alice"));
        assert!(json.contains("Alice Wang"));
        assert!(json.contains("alice.wang@partner.com"));
        assert!(json.contains("+86-138-0000-0000"));
        assert!(json.contains("external"));
        assert!(json.contains("dept_engineering"));
        assert!(json.contains("高级工程师"));
    }

    #[test]
    fn test_organization_structure() {
        let dept1 = CollaborationDepartment {
            department_id: "dept_001".to_string(),
            name: "研发部".to_string(),
            parent_department_id: None,
            status: Some("active".to_string()),
            level: Some(1),
            member_count: Some(15),
            created_at: Some(1640995200),
            updated_at: Some(1640995200),
        };

        let user1 = CollaborationUser {
            user_id: "user_bob".to_string(),
            name: "Bob Chen".to_string(),
            email: Some("bob.chen@partner.com".to_string()),
            mobile: None,
            avatar_url: None,
            status: Some("active".to_string()),
            user_type: Some("external".to_string()),
            department_ids: Some(vec!["dept_001".to_string()]),
            job_title: Some("产品经理".to_string()),
            created_at: Some(1640995200),
            updated_at: Some(1640995200),
        };

        let structure = OrganizationStructure {
            departments: Some(vec![dept1]),
            users: Some(vec![user1]),
        };

        let json = serde_json::to_string(&structure).unwrap();
        assert!(json.contains("研发部"));
        assert!(json.contains("Bob Chen"));
        assert!(json.contains("bob.chen@partner.com"));
        assert!(json.contains("产品经理"));
    }

    #[test]
    fn test_shared_member_scope() {
        let scope = SharedMemberScope {
            scope_type: Some("department".to_string()),
            department_ids: Some(vec!["dept_tech".to_string(), "dept_product".to_string()]),
            user_ids: Some(vec!["user_lead1".to_string(), "user_lead2".to_string()]),
            description: Some("技术和产品团队共享范围".to_string()),
        };
        let json = serde_json::to_string(&scope).unwrap();
        assert!(json.contains("department"));
        assert!(json.contains("dept_tech"));
        assert!(json.contains("dept_product"));
        assert!(json.contains("user_lead1"));
        assert!(json.contains("技术和产品团队共享范围"));
    }

    #[test]
    fn test_searchable_visible_rule() {
        let config = RuleConfig {
            visibility: Some("public".to_string()),
            searchable: Some(true),
            scope: Some(RuleScope {
                department_ids: Some(vec!["dept_all".to_string()]),
                user_ids: None,
                group_ids: None,
            }),
            exceptions: Some(vec!["sensitive_dept".to_string()]),
        };

        let rule = SearchableVisibleRule {
            rule_id: "rule_visibility_001".to_string(),
            name: "公开可见规则".to_string(),
            description: Some("允许外部组织成员搜索和查看公开信息".to_string()),
            rule_type: Some("visibility".to_string()),
            status: Some("active".to_string()),
            org_id: Some("partner_org_001".to_string()),
            config: Some(config),
            creator: Some("admin_user".to_string()),
            created_at: Some(1640995200),
            updated_at: Some(1640995200),
        };

        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("rule_visibility_001"));
        assert!(json.contains("公开可见规则"));
        assert!(json.contains("允许外部组织成员搜索"));
        assert!(json.contains("visibility"));
        assert!(json.contains("partner_org_001"));
        assert!(json.contains("\"searchable\":true"));
        assert!(json.contains("dept_all"));
        assert!(json.contains("sensitive_dept"));
    }

    #[test]
    fn test_rule_config() {
        let config = RuleConfig {
            visibility: Some("private".to_string()),
            searchable: Some(false),
            scope: Some(RuleScope {
                department_ids: Some(vec!["hr_dept".to_string()]),
                user_ids: Some(vec!["hr_manager".to_string()]),
                group_ids: Some(vec!["hr_team".to_string()]),
            }),
            exceptions: Some(vec!["ceo".to_string(), "cto".to_string()]),
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("private"));
        assert!(json.contains("\"searchable\":false"));
        assert!(json.contains("hr_dept"));
        assert!(json.contains("hr_manager"));
        assert!(json.contains("hr_team"));
        assert!(json.contains("ceo"));
        assert!(json.contains("cto"));
    }

    #[test]
    fn test_rule_scope() {
        let scope = RuleScope {
            department_ids: Some(vec!["sales".to_string(), "marketing".to_string()]),
            user_ids: Some(vec!["sales_lead".to_string()]),
            group_ids: Some(vec!["sales_team".to_string(), "marketing_team".to_string()]),
        };
        let json = serde_json::to_string(&scope).unwrap();
        assert!(json.contains("sales"));
        assert!(json.contains("marketing"));
        assert!(json.contains("sales_lead"));
        assert!(json.contains("sales_team"));
        assert!(json.contains("marketing_team"));
    }

    #[test]
    fn test_minimal_structs() {
        let minimal_org = CollaborationOrganization {
            org_id: "minimal_org".to_string(),
            org_name: "Minimal Org".to_string(),
            org_type: None,
            status: None,
            created_at: None,
            updated_at: None,
            domain: None,
            description: None,
        };
        let json = serde_json::to_string(&minimal_org).unwrap();
        assert!(json.contains("minimal_org"));
        assert!(json.contains("Minimal Org"));
        assert!(!json.contains("org_type"));
        assert!(!json.contains("status"));

        let minimal_user = CollaborationUser {
            user_id: "minimal_user".to_string(),
            name: "Minimal User".to_string(),
            email: None,
            mobile: None,
            avatar_url: None,
            status: None,
            user_type: None,
            department_ids: None,
            job_title: None,
            created_at: None,
            updated_at: None,
        };
        let json = serde_json::to_string(&minimal_user).unwrap();
        assert!(json.contains("minimal_user"));
        assert!(json.contains("Minimal User"));
        assert!(!json.contains("email"));
        assert!(!json.contains("mobile"));
    }

    #[test]
    fn test_nested_structures() {
        let page_response = PageResponse {
            items: vec![
                CollaborationOrganization {
                    org_id: "org_1".to_string(),
                    org_name: "Organization 1".to_string(),
                    org_type: Some("supplier".to_string()),
                    status: Some("active".to_string()),
                    created_at: None,
                    updated_at: None,
                    domain: None,
                    description: None,
                },
                CollaborationOrganization {
                    org_id: "org_2".to_string(),
                    org_name: "Organization 2".to_string(),
                    org_type: Some("customer".to_string()),
                    status: Some("pending".to_string()),
                    created_at: Some(1640995200),
                    updated_at: None,
                    domain: Some("customer.com".to_string()),
                    description: Some("Important customer".to_string()),
                },
            ],
            page_token: Some("next_orgs".to_string()),
            has_more: Some(true),
        };
        let json = serde_json::to_string(&page_response).unwrap();
        assert!(json.contains("org_1"));
        assert!(json.contains("org_2"));
        assert!(json.contains("supplier"));
        assert!(json.contains("customer"));
        assert!(json.contains("customer.com"));
        assert!(json.contains("Important customer"));
        assert!(json.contains("next_orgs"));
    }

    #[test]
    fn test_complex_trust_party_scenario() {
        let engineering_dept = CollaborationDepartment {
            department_id: "eng_dept".to_string(),
            name: "Engineering".to_string(),
            parent_department_id: None,
            status: Some("active".to_string()),
            level: Some(1),
            member_count: Some(30),
            created_at: Some(1640995200),
            updated_at: Some(1640995200),
        };

        let senior_engineer = CollaborationUser {
            user_id: "user_senior_eng".to_string(),
            name: "Senior Engineer".to_string(),
            email: Some("senior@partner.com".to_string()),
            mobile: Some("+1-555-0123".to_string()),
            avatar_url: Some("https://partner.com/avatars/senior.jpg".to_string()),
            status: Some("active".to_string()),
            user_type: Some("external_contractor".to_string()),
            department_ids: Some(vec!["eng_dept".to_string()]),
            job_title: Some("Senior Software Engineer".to_string()),
            created_at: Some(1640995200),
            updated_at: Some(1640995200),
        };

        let org_structure = OrganizationStructure {
            departments: Some(vec![engineering_dept]),
            users: Some(vec![senior_engineer]),
        };

        let scope = SharedMemberScope {
            scope_type: Some("project_based".to_string()),
            department_ids: Some(vec!["eng_dept".to_string()]),
            user_ids: Some(vec!["user_senior_eng".to_string()]),
            description: Some(
                "Project-based collaboration scope for external contractors".to_string(),
            ),
        };

        let rule_config = RuleConfig {
            visibility: Some("restricted".to_string()),
            searchable: Some(true),
            scope: Some(RuleScope {
                department_ids: Some(vec!["eng_dept".to_string()]),
                user_ids: Some(vec!["user_senior_eng".to_string()]),
                group_ids: None,
            }),
            exceptions: Some(vec![
                "external_contractors".to_string(),
                "confidential_projects".to_string(),
            ]),
        };

        let visibility_rule = SearchableVisibleRule {
            rule_id: "contractor_visibility_rule".to_string(),
            name: "External Contractor Visibility Rule".to_string(),
            description: Some(
                "Controls visibility and search permissions for external contractors".to_string(),
            ),
            rule_type: Some("contractor_access".to_string()),
            status: Some("active".to_string()),
            org_id: Some("partner_tech_org".to_string()),
            config: Some(rule_config),
            creator: Some("security_admin".to_string()),
            created_at: Some(1640995200),
            updated_at: Some(1640995200),
        };

        let json_org = serde_json::to_string(&org_structure).unwrap();
        let json_scope = serde_json::to_string(&scope).unwrap();
        let json_rule = serde_json::to_string(&visibility_rule).unwrap();

        assert!(json_org.contains("Engineering"));
        assert!(json_org.contains("Senior Engineer"));
        assert!(json_org.contains("external_contractor"));

        assert!(json_scope.contains("project_based"));
        assert!(json_scope.contains("Project-based collaboration"));

        assert!(json_rule.contains("contractor_visibility_rule"));
        assert!(json_rule.contains("External Contractor Visibility Rule"));
        assert!(json_rule.contains("restricted"));
        assert!(json_rule.contains("external_contractors"));
        assert!(json_rule.contains("confidential_projects"));
    }
}
