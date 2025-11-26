//! API实现验证检查工具
//!
//! 用于验证CSV中定义的API是否在代码中正确实现

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ApiDefinition {
    pub name: String,
    pub http_method: String,
    pub path: String,
    pub module: String,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct ValidationResult {
    pub api: ApiDefinition,
    pub implemented: bool,
    pub file_path: Option<String>,
    pub issues: Vec<String>,
}

pub struct ApiValidationChecker {
    api_definitions: Vec<ApiDefinition>,
    security_code_path: String,
}

impl ApiValidationChecker {
    pub fn new(security_code_path: impl Into<String>) -> Self {
        Self {
            api_definitions: Vec::new(),
            security_code_path: security_code_path.into(),
        }
    }

    pub fn load_apis_from_csv(&mut self, csv_content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = csv::Reader::from_reader(csv_content.as_bytes());
        for result in reader.deserialize() {
            let api: ApiDefinition = result?;
            self.api_definitions.push(api);
        }
        Ok(())
    }

    pub fn validate_all_apis(&self) -> Vec<ValidationResult> {
        let mut results = Vec::new();

        for api in &self.api_definitions {
            let result = self.validate_single_api(api.clone());
            results.push(result);
        }

        results
    }

    fn validate_single_api(&self, api: ApiDefinition) -> ValidationResult {
        let mut issues = Vec::new();
        let mut implemented = false;
        let mut file_path = None;

        // 根据API路径推断模块文件位置
        let expected_file = self.infer_file_path(&api);

        if let Some(expected_path) = &expected_file {
            if Path::new(&self.security_code_path).join(expected_path).exists() {
                file_path = Some(expected_path.clone());

                // 检查文件中是否包含对应的HTTP方法和路径
                if let Ok(content) = fs::read_to_string(
                    Path::new(&self.security_code_path).join(expected_path)
                ) {
                    implemented = self.check_api_implementation(&content, &api);

                    if !implemented {
                        issues.push("API未在对应文件中找到实现".to_string());
                    }
                } else {
                    issues.push("无法读取实现文件".to_string());
                }
            } else {
                issues.push(format!("实现文件不存在: {}", expected_path));
            }
        } else {
            issues.push("无法推断API对应的模块文件".to_string());
        }

        ValidationResult {
            api,
            implemented,
            file_path,
            issues,
        }
    }

    fn infer_file_path(&self, api: &ApiDefinition) -> Option<String> {
        // 根据API路径推断对应的Rust模块文件
        if api.path.contains("/acs/v1/") {
            if api.path.contains("/users") {
                Some("src/acs/v1/users/mod.rs".to_string())
            } else if api.path.contains("/face") {
                Some("src/acs/v1/user_faces/mod.rs".to_string())
            } else if api.path.contains("/rule_external") {
                Some("src/acs/v1/rule_external/mod.rs".to_string())
            } else if api.path.contains("/visitors") {
                Some("src/acs/v1/visitors/mod.rs".to_string())
            } else if api.path.contains("/devices") && api.path.starts_with("/open-apis/acs") {
                Some("src/acs/v1/devices/mod.rs".to_string())
            } else if api.path.contains("/access_records") {
                Some("src/acs/v1/access_records/mod.rs".to_string())
            }
        } else if api.path.contains("/security_and_compliance/") {
            if api.path.contains("/v2/device_records") {
                Some("src/security_and_compliance/v2/device_records/mod.rs".to_string())
            } else if api.path.contains("/v2/device_apply_records") {
                Some("src/security_and_compliance/v2/device_apply_records/mod.rs".to_string())
            } else if api.path.contains("/v1/openapi_logs") {
                Some("src/security_and_compliance/v1/openapi_logs/mod.rs".to_string())
            }
        } else {
            None
        }
    }

    fn check_api_implementation(&self, content: &str, api: &ApiDefinition) -> bool {
        // 检查HTTP方法
        let method_found = match api.http_method.to_uppercase().as_str() {
            "GET" => content.contains(".get(") || content.contains("reqwest::Client::new().get("),
            "POST" => content.contains(".post(") || content.contains("reqwest::Client::new().post("),
            "PUT" => content.contains(".put(") || content.contains("reqwest::Client::new().put("),
            "PATCH" => content.contains(".patch(") || content.contains("reqwest::Client::new().patch("),
            "DELETE" => content.contains(".delete(") || content.contains("reqwest::Client::new().delete("),
            _ => false,
        };

        // 检查API路径
        let path_found = content.contains(&api.path.replace(":user_id", "").replace(":visitor_id", "")
            .replace(":device_record_id", "").replace(":access_record_id", "")
            .replace(":device_apply_record_id", ""));

        method_found && path_found
    }

    pub fn generate_report(&self, results: &[ValidationResult]) -> String {
        let mut report = String::new();
        report.push_str("# API实现验证报告\n\n");

        // 统计信息
        let total_apis = results.len();
        let implemented_count = results.iter().filter(|r| r.implemented).count();
        let missing_count = total_apis - implemented_count;

        report.push_str(&format!("## 验证概况\n\n"));
        report.push_str(&format!("- 总API数量: {}\n", total_apis));
        report.push_str(&format!("- 已实现: {}\n", implemented_count));
        report.push_str(&format!("- 未实现: {}\n", missing_count));
        report.push_str(&format!("- 实现率: {:.1}%\n\n", (implemented_count as f64 / total_apis as f64) * 100.0));

        // 详细结果
        report.push_str("## 详细验证结果\n\n");

        for (index, result) in results.iter().enumerate() {
            report.push_str(&format!("### {}. {}\n\n", index + 1, result.api.name));
            report.push_str(&format!("- **HTTP方法**: {}\n", result.api.http_method));
            report.push_str(&format!("- **API路径**: {}\n", result.api.path));
            report.push_str(&format!("- **实现状态**: {}\n", if result.implemented { "✅ 已实现" } else { "❌ 未实现" }));

            if let Some(file_path) = &result.file_path {
                report.push_str(&format!("- **实现文件**: `{}`\n", file_path));
            }

            if !result.issues.is_empty() {
                report.push_str("- **问题列表**:\n");
                for issue in &result.issues {
                    report.push_str(&format!("  - {}\n", issue));
                }
            }

            report.push_str("\n");
        }

        // 缺失API总结
        let missing_apis: Vec<_> = results.iter().filter(|r| !r.implemented).collect();
        if !missing_apis.is_empty() {
            report.push_str("## 缺失实现总结\n\n");
            report.push_str("以下API需要实现:\n\n");

            for (index, result) in missing_apis.iter().enumerate() {
                report.push_str(&format!("{}. {} - {} {}\n",
                    index + 1, result.api.name, result.api.http_method, result.api.path));
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_validation_checker() {
        let checker = ApiValidationChecker::new("crates/openlark-security");

        let csv_content = r#"name,http_method,path,module,description
获取单个用户信息,GET,/open-apis/acs/v1/users/:user_id,acs/v1/users,获取单个用户信息
创建设备,POST,/open-apis/security_and_compliance/v2/device_records,security_and_compliance/v2/device_records,新增设备
"#;

        checker.load_apis_from_csv(csv_content).unwrap();
        let results = checker.validate_all_apis();
        assert!(!results.is_empty());

        let report = checker.generate_report(&results);
        assert!(report.contains("API实现验证报告"));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let security_code_path = "crates/openlark-security";
    let mut checker = ApiValidationChecker::new(security_code_path);

    // 示例CSV数据 - 实际使用时应该从文件读取
    let csv_content = r#"name,http_method,path,module,description
修改用户部分信息,PATCH,/open-apis/acs/v1/users/:user_id,acs/v1/users,修改用户部分信息
获取单个用户信息,GET,/open-apis/acs/v1/users/:user_id,acs/v1/users,获取单个用户信息
获取用户列表,GET,/open-apis/acs/v1/users,acs/v1/users,获取用户列表
上传人脸图片,PUT,/open-apis/acs/v1/users/:user_id/face,acs/v1/user_faces,上传人脸图片
下载人脸图片,GET,/open-apis/acs/v1/users/:user_id/face,acs/v1/user_faces,下载人脸图片
设备绑定权限组,POST,/open-apis/acs/v1/rule_external/device_bind,acs/v1/rule_external,设备绑定权限组
获取权限组信息,GET,/open-apis/acs/v1/rule_external,acs/v1/rule_external,获取权限组信息
删除权限组,DELETE,/open-apis/acs/v1/rule_external,acs/v1/rule_external,删除权限组
创建或更新权限组,POST,/open-apis/acs/v1/rule_external,acs/v1/rule_external,创建或更新权限组
删除访客,DELETE,/open-apis/acs/v1/visitors/:visitor_id,acs/v1/visitors,删除访客
添加访客,POST,/open-apis/acs/v1/visitors,acs/v1/visitors,添加访客
获取门禁设备列表,GET,/open-apis/acs/v1/devices,acs/v1/devices,获取门禁设备列表
获取门禁记录列表,GET,/open-apis/acs/v1/access_records,acs/v1/access_records,获取门禁记录列表
下载开门时的人脸识别图片,GET,/open-apis/acs/v1/access_records/:access_record_id/access_photo,acs/v1/access_records,下载开门时的人脸识别图片
获取客户端设备认证信息,GET,/open-apis/security_and_compliance/v2/device_records/mine,security_and_compliance/v2/device_records,获取客户端设备认证信息
新增设备,POST,/open-apis/security_and_compliance/v2/device_records,security_and_compliance/v2/device_records,新增设备
查询设备信息,GET,/open-apis/security_and_compliance/v2/device_records,security_and_compliance/v2/device_records,查询设备信息
获取设备信息,GET,/open-apis/security_and_compliance/v2/device_records/:device_record_id,security_and_compliance/v2/device_records,获取设备信息
更新设备,PUT,/open-apis/security_and_compliance/v2/device_records/:device_record_id,security_and_compliance/v2/device_records,更新设备
删除设备,DELETE,/open-apis/security_and_compliance/v2/device_records/:device_record_id,security_and_compliance/v2/device_records,删除设备
审批设备申报,PUT,/open-apis/security_and_compliance/v2/device_apply_records/:device_apply_record_id,security_and_compliance/v2/device_apply_records,审批设备申报
获取OpenAPI审计日志数据,POST,/open-apis/security_and_compliance/v1/openapi_logs/list_data,security_and_compliance/v1/openapi_logs,获取OpenAPI审计日志数据
"#;

    checker.load_apis_from_csv(csv_content)?;
    let results = checker.validate_all_apis();
    let report = checker.generate_report(&results);

    println!("{}", report);

    // 保存报告到文件
    fs::write("api_validation_report.md", report)?;

    Ok(())
}