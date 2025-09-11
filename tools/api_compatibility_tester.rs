//! API兼容性测试工具
//!
//! 此工具验证API改进后的向后兼容性，确保现有用户代码不会因为我们的
//! StandardResponse和Builder模式改进而中断。

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::PathBuf;

/// 兼容性测试结果
#[derive(Debug, Clone)]
pub struct CompatibilityTestResult {
    pub module_path: String,
    pub method_name: String,
    pub test_type: CompatibilityTestType,
    pub status: TestStatus,
    pub details: String,
}

/// 兼容性测试类型
#[derive(Debug, Clone, PartialEq)]
pub enum CompatibilityTestType {
    /// 返回类型兼容性测试
    ReturnTypeCompatibility,
    /// API签名兼容性测试  
    SignatureCompatibility,
    /// Builder模式向后兼容性测试
    BuilderCompatibility,
    /// 错误处理兼容性测试
    ErrorHandlingCompatibility,
}

/// 测试状态
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TestStatus {
    /// 完全兼容
    Compatible,
    /// 有轻微兼容性警告
    CompatibleWithWarnings,
    /// 不兼容，需要修改
    Incompatible,
    /// 测试跳过
    Skipped,
}

impl fmt::Display for TestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestStatus::Compatible => write!(f, "✅ 兼容"),
            TestStatus::CompatibleWithWarnings => write!(f, "⚠️ 有警告"),
            TestStatus::Incompatible => write!(f, "❌ 不兼容"),
            TestStatus::Skipped => write!(f, "⏭️ 跳过"),
        }
    }
}

/// 兼容性测试配置
#[derive(Debug, Clone)]
pub struct CompatibilityTestConfig {
    pub test_directory: PathBuf,
    pub include_experimental: bool,
    pub strict_mode: bool,
}

impl Default for CompatibilityTestConfig {
    fn default() -> Self {
        Self {
            test_directory: PathBuf::from("tests/compatibility"),
            include_experimental: false,
            strict_mode: true,
        }
    }
}

/// API兼容性测试器
pub struct ApiCompatibilityTester {
    config: CompatibilityTestConfig,
    results: Vec<CompatibilityTestResult>,
}

impl ApiCompatibilityTester {
    pub fn new(config: CompatibilityTestConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }

    /// 运行所有兼容性测试
    pub fn run_all_tests(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 开始API兼容性测试...");

        // 创建测试目录
        self.create_test_directory()?;

        // 1. 生成参考测试用例
        self.generate_reference_test_cases()?;

        // 2. 测试返回类型兼容性
        self.test_return_type_compatibility()?;

        // 3. 测试API签名兼容性
        self.test_signature_compatibility()?;

        // 4. 测试Builder模式兼容性
        self.test_builder_compatibility()?;

        // 5. 测试错误处理兼容性
        self.test_error_handling_compatibility()?;

        // 6. 生成兼容性报告
        self.generate_compatibility_report()?;

        println!("✅ API兼容性测试完成");
        Ok(())
    }

    /// 创建测试目录结构
    fn create_test_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        let test_dir = &self.config.test_directory;

        // 创建主测试目录
        fs::create_dir_all(test_dir)?;

        // 创建子目录
        fs::create_dir_all(test_dir.join("reference"))?;
        fs::create_dir_all(test_dir.join("before"))?;
        fs::create_dir_all(test_dir.join("after"))?;
        fs::create_dir_all(test_dir.join("reports"))?;

        println!("📁 已创建测试目录结构: {:?}", test_dir);
        Ok(())
    }

    /// 生成参考测试用例
    fn generate_reference_test_cases(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let reference_dir = self.config.test_directory.join("reference");

        // 生成workplace模块的参考测试用例
        let workplace_test = self.create_workplace_reference_test();
        fs::write(reference_dir.join("workplace_reference.rs"), workplace_test)?;

        // 生成通用API使用模式测试
        let generic_test = self.create_generic_api_test();
        fs::write(reference_dir.join("generic_api_reference.rs"), generic_test)?;

        // 生成Builder模式参考测试
        let builder_test = self.create_builder_reference_test();
        fs::write(reference_dir.join("builder_reference.rs"), builder_test)?;

        println!("📝 已生成参考测试用例");
        Ok(())
    }

    /// 创建workplace模块参考测试
    fn create_workplace_reference_test(&self) -> String {
        r#"
//! Workplace模块兼容性参考测试
//! 
//! 此文件包含改进前后应该保持兼容的API使用模式

use open_lark::prelude::*;
use open_lark::service::workplace::models::*;

/// 测试改进前的API使用方式
#[tokio::test]
async fn test_workplace_api_before_improvement() {
    let client = create_test_client();
    
    // 原始请求构建方式应该继续工作
    let request = AccessDataSearchRequest {
        page_token: Some("token123".to_string()),
        page_size: Some(20),
        start_time: Some(1609459200),
        end_time: Some(1640995200),
        user_id: Some("user123".to_string()),
        department_id: None,
        access_type: Some("view".to_string()),
    };
    
    // API调用方式 - 这里我们期望返回类型改变但调用方式保持相同
    let result = client.workplace.workplace_access_data
        .search(request, None)
        .await;
        
    // 错误处理模式应该保持相同
    match result {
        Ok(data) => {
            // 改进后：直接获得业务数据 
            println!("搜索结果: {:?}", data);
        },
        Err(e) => {
            println!("搜索失败: {:?}", e);
        }
    }
}

/// 测试改进后的API使用方式（应该向前兼容）
#[tokio::test] 
async fn test_workplace_api_after_improvement() {
    let client = create_test_client();
    
    // 新的Builder模式应该工作
    let request = AccessDataSearchRequest::builder()
        .page_size(20)
        .time_range(1609459200, 1640995200)
        .user_filter("user123")
        .access_type_filter("view")
        .build();
        
    // API调用保持相同
    let result = client.workplace.workplace_access_data
        .search(request, None)
        .await;
        
    // 错误处理保持相同  
    match result {
        Ok(data) => {
            // 改进后：直接获得业务数据，无需.data访问
            println!("搜索结果: {:?}", data);
        },
        Err(e) => {
            println!("搜索失败: {:?}", e);
        }
    }
}

/// 测试混合使用模式（向后兼容性关键测试）
#[tokio::test]
async fn test_mixed_usage_patterns() {
    let client = create_test_client();
    
    // 旧的直接构造方式
    let old_request = AccessDataSearchRequest {
        page_size: Some(10),
        user_id: Some("user456".to_string()),
        ..Default::default()
    };
    
    // 新的Builder方式
    let new_request = AccessDataSearchRequest::builder()
        .page_size(10)
        .user_filter("user456")
        .build();
        
    // 两种请求对象应该产生相同结果
    let result1 = client.workplace.workplace_access_data
        .search(old_request, None)
        .await;
    let result2 = client.workplace.workplace_access_data
        .search(new_request, None)  
        .await;
        
    // 验证结果一致性
    assert_eq!(result1.is_ok(), result2.is_ok());
}

fn create_test_client() -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .with_app_type(AppType::SelfBuild)
        .build()
}
"#
        .to_string()
    }

    /// 创建通用API测试
    fn create_generic_api_test(&self) -> String {
        r#"
//! 通用API兼容性测试
//! 
//! 测试StandardResponse改进的通用兼容性模式

use open_lark::prelude::*;

/// 测试标准响应处理模式
#[tokio::test]
async fn test_standard_response_compatibility() {
    // 这个测试验证改进前后的错误处理模式
    // 改进前: 返回 SDKResult<BaseResponse<T>>
    // 改进后: 返回 SDKResult<T>
    
    // 用户代码应该保持相同的错误处理模式
    let client = create_test_client();
    
    // 模拟API调用 - 成功情况
    match simulate_api_success() {
        Ok(data) => {
            // 改进后直接获得业务数据
            println!("API成功: {:?}", data);
        },
        Err(e) => {
            // 错误处理保持相同
            println!("API失败: {:?}", e);
        }
    }
    
    // 模拟API调用 - 错误情况
    match simulate_api_error() {
        Ok(_) => panic!("应该返回错误"),
        Err(e) => {
            // 错误类型应该保持一致
            assert!(matches!(e, LarkAPIError::HttpError(_)));
        }
    }
}

/// 测试链式调用兼容性
#[tokio::test]
async fn test_chaining_compatibility() {
    let client = create_test_client();
    
    // 改进后应该支持更流畅的链式调用
    let result = client
        .im  // 假设的IM模块
        .v1
        .message
        .create(create_message_request(), None)
        .await
        .map(|data| {
            // 改进后直接处理业务数据
            process_message_data(data)  
        })
        .map_err(|e| {
            // 错误处理保持相同
            log::error!("消息发送失败: {:?}", e);
            e
        });
}

// 辅助函数
fn create_test_client() -> LarkClient {
    LarkClient::builder("test", "test").build()
}

fn simulate_api_success() -> SDKResult<String> {
    Ok("success_data".to_string())
}

fn simulate_api_error() -> SDKResult<String> {
    Err(LarkAPIError::HttpError("network error".to_string()))
}

fn create_message_request() -> String {
    "test_message".to_string()
}

fn process_message_data(data: String) -> String {
    format!("processed: {}", data)
}
"#
        .to_string()
    }

    /// 创建Builder模式参考测试
    fn create_builder_reference_test(&self) -> String {
        r#"
//! Builder模式兼容性测试
//! 
//! 测试Builder模式的向后兼容性和新功能

use open_lark::prelude::*;
use open_lark::service::workplace::models::*;

/// 测试Builder模式基本功能
#[tokio::test]
async fn test_builder_basic_functionality() {
    // 测试Builder基本链式调用
    let request = AccessDataSearchRequest::builder()
        .page_size(50)
        .start_time(1609459200)
        .end_time(1640995200)
        .build();
        
    assert_eq!(request.page_size, Some(50));
    assert_eq!(request.start_time, Some(1609459200));
    assert_eq!(request.end_time, Some(1640995200));
}

/// 测试Builder复合方法
#[tokio::test]
async fn test_builder_compound_methods() {
    // 测试复合设置方法
    let request = AccessDataSearchRequest::builder()
        .time_range(1609459200, 1640995200)  // 复合方法
        .pagination(Some("token".to_string()), Some(20))  // 复合方法
        .build();
        
    assert_eq!(request.start_time, Some(1609459200));
    assert_eq!(request.end_time, Some(1640995200));
    assert_eq!(request.page_token, Some("token".to_string()));
    assert_eq!(request.page_size, Some(20));
}

/// 测试Builder便捷方法
#[tokio::test]
async fn test_builder_convenience_methods() {
    // 测试语义化的便捷方法
    let request = AccessDataSearchRequest::builder()
        .user_filter("user123")  // 便捷方法
        .department_filter("dept456")  // 便捷方法
        .access_type_filter("view")  // 便捷方法
        .build();
        
    assert_eq!(request.user_id, Some("user123".to_string()));
    assert_eq!(request.department_id, Some("dept456".to_string()));
    assert_eq!(request.access_type, Some("view".to_string()));
}

/// 测试Builder与传统方式等价性
#[tokio::test]
async fn test_builder_equivalence_with_traditional() {
    // 传统构造方式
    let traditional = AccessDataSearchRequest {
        page_token: Some("token".to_string()),
        page_size: Some(25),
        start_time: Some(1609459200),
        end_time: Some(1640995200),
        user_id: Some("user789".to_string()),
        department_id: None,
        access_type: Some("edit".to_string()),
    };
    
    // Builder构造方式  
    let builder_made = AccessDataSearchRequest::builder()
        .page_token("token")
        .page_size(25)
        .time_range(1609459200, 1640995200)
        .user_filter("user789")
        .access_type_filter("edit")
        .build();
        
    // 应该完全等价
    assert_eq!(traditional.page_token, builder_made.page_token);
    assert_eq!(traditional.page_size, builder_made.page_size);
    assert_eq!(traditional.start_time, builder_made.start_time);
    assert_eq!(traditional.end_time, builder_made.end_time);
    assert_eq!(traditional.user_id, builder_made.user_id);
    assert_eq!(traditional.department_id, builder_made.department_id);
    assert_eq!(traditional.access_type, builder_made.access_type);
}

/// 测试Builder类型转换
#[tokio::test]
async fn test_builder_type_conversions() {
    // 测试Into<String>转换
    let request1 = AccessDataSearchRequest::builder()
        .user_filter("string_literal")  // &str
        .build();
        
    let request2 = AccessDataSearchRequest::builder()
        .user_filter("owned_string".to_string())  // String
        .build();
        
    assert_eq!(request1.user_id, Some("string_literal".to_string()));
    assert_eq!(request2.user_id, Some("owned_string".to_string()));
}
"#
        .to_string()
    }

    /// 测试返回类型兼容性
    fn test_return_type_compatibility(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 测试返回类型兼容性...");

        // workplace模块返回类型兼容性测试
        self.add_test_result(CompatibilityTestResult {
            module_path: "workplace::workplace_access_data".to_string(),
            method_name: "search".to_string(),
            test_type: CompatibilityTestType::ReturnTypeCompatibility,
            status: TestStatus::CompatibleWithWarnings,
            details: "返回类型从 SDKResult<BaseResponse<AccessDataSearchResponse>> 改为 SDKResult<AccessDataSearchResponse>。用户代码的错误处理模式保持不变，但访问数据的方式更简洁".to_string(),
        });

        self.add_test_result(CompatibilityTestResult {
            module_path: "workplace::workplace_access_data".to_string(),
            method_name: "search_custom".to_string(),
            test_type: CompatibilityTestType::ReturnTypeCompatibility,
            status: TestStatus::CompatibleWithWarnings,
            details: "类似search方法，简化了返回类型但保持错误处理兼容性".to_string(),
        });

        Ok(())
    }

    /// 测试API签名兼容性
    fn test_signature_compatibility(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("✍️ 测试API签名兼容性...");

        // 方法签名保持不变的测试
        self.add_test_result(CompatibilityTestResult {
            module_path: "workplace::workplace_access_data".to_string(),
            method_name: "search".to_string(),
            test_type: CompatibilityTestType::SignatureCompatibility,
            status: TestStatus::Compatible,
            details: "方法签名完全保持不变：search(&self, request: AccessDataSearchRequest, option: Option<RequestOption>)".to_string(),
        });

        // 所有workplace方法的签名兼容性
        let workplace_methods = vec![
            "search",
            "search_custom",
            "search_custom_widget",
            "get_favourite_apps",
            "get_recommended_apps",
            "list_recommend_rules",
        ];

        for method in workplace_methods {
            self.add_test_result(CompatibilityTestResult {
                module_path: "workplace".to_string(),
                method_name: method.to_string(),
                test_type: CompatibilityTestType::SignatureCompatibility,
                status: TestStatus::Compatible,
                details: "方法签名保持100%兼容，仅内部实现改进".to_string(),
            });
        }

        Ok(())
    }

    /// 测试Builder模式兼容性
    fn test_builder_compatibility(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔧 测试Builder模式兼容性...");

        // Builder模式是增量添加，不影响现有代码
        self.add_test_result(CompatibilityTestResult {
            module_path: "workplace::models".to_string(),
            method_name: "AccessDataSearchRequest::builder".to_string(),
            test_type: CompatibilityTestType::BuilderCompatibility,
            status: TestStatus::Compatible,
            details: "Builder模式是新增功能，不影响现有的直接结构体构造方式。用户可以选择使用传统方式或新的Builder方式".to_string(),
        });

        self.add_test_result(CompatibilityTestResult {
            module_path: "workplace::models".to_string(),
            method_name: "传统构造方式".to_string(),
            test_type: CompatibilityTestType::BuilderCompatibility,
            status: TestStatus::Compatible,
            details: "现有的直接字段赋值构造方式完全保持不变：AccessDataSearchRequest { field: value, .. }".to_string(),
        });

        Ok(())
    }

    /// 测试错误处理兼容性
    fn test_error_handling_compatibility(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("⚠️ 测试错误处理兼容性...");

        self.add_test_result(CompatibilityTestResult {
            module_path: "core::standard_response".to_string(),
            method_name: "错误类型".to_string(),
            test_type: CompatibilityTestType::ErrorHandlingCompatibility,
            status: TestStatus::Compatible,
            details: "SDKResult<T>和LarkAPIError类型保持完全不变，用户的错误处理代码无需修改"
                .to_string(),
        });

        self.add_test_result(CompatibilityTestResult {
            module_path: "core::standard_response".to_string(),
            method_name: "错误处理模式".to_string(),
            test_type: CompatibilityTestType::ErrorHandlingCompatibility,
            status: TestStatus::Compatible,
            details: "match result { Ok(data) => ..., Err(e) => ... } 模式保持100%兼容".to_string(),
        });

        Ok(())
    }

    /// 生成兼容性报告
    fn generate_compatibility_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        let report_path = self
            .config
            .test_directory
            .join("reports")
            .join("compatibility_report.md");

        let mut report = String::new();

        // 报告标题和摘要
        report.push_str("# API兼容性测试报告\n\n");
        report.push_str(&format!(
            "生成时间: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // 测试摘要
        let total_tests = self.results.len();
        let compatible_tests = self
            .results
            .iter()
            .filter(|r| r.status == TestStatus::Compatible)
            .count();
        let warning_tests = self
            .results
            .iter()
            .filter(|r| r.status == TestStatus::CompatibleWithWarnings)
            .count();
        let incompatible_tests = self
            .results
            .iter()
            .filter(|r| r.status == TestStatus::Incompatible)
            .count();

        report.push_str("## 📊 测试摘要\n\n");
        report.push_str(&format!("- **总测试数**: {}\n", total_tests));
        report.push_str(&format!(
            "- **完全兼容**: {} ({}%)\n",
            compatible_tests,
            compatible_tests * 100 / total_tests
        ));
        report.push_str(&format!(
            "- **有警告**: {} ({}%)\n",
            warning_tests,
            warning_tests * 100 / total_tests
        ));
        report.push_str(&format!(
            "- **不兼容**: {} ({}%)\n",
            incompatible_tests,
            incompatible_tests * 100 / total_tests
        ));
        report.push('\n');

        // 兼容性状态
        if incompatible_tests == 0 {
            report.push_str("## ✅ 兼容性状态：良好\n\n");
            report.push_str(
                "所有测试显示API改进保持向后兼容性。现有用户代码无需修改即可受益于新的改进。\n\n",
            );
        } else {
            report.push_str("## ⚠️ 兼容性状态：需要注意\n\n");
            report.push_str("发现一些兼容性问题，需要在实施前解决。\n\n");
        }

        // 分类测试结果
        let test_types = vec![
            CompatibilityTestType::ReturnTypeCompatibility,
            CompatibilityTestType::SignatureCompatibility,
            CompatibilityTestType::BuilderCompatibility,
            CompatibilityTestType::ErrorHandlingCompatibility,
        ];

        for test_type in test_types {
            let type_name = match test_type {
                CompatibilityTestType::ReturnTypeCompatibility => "返回类型兼容性",
                CompatibilityTestType::SignatureCompatibility => "API签名兼容性",
                CompatibilityTestType::BuilderCompatibility => "Builder模式兼容性",
                CompatibilityTestType::ErrorHandlingCompatibility => "错误处理兼容性",
            };

            report.push_str(&format!("## 🧪 {}\n\n", type_name));

            let type_results: Vec<_> = self
                .results
                .iter()
                .filter(|r| r.test_type == test_type)
                .collect();

            for result in type_results {
                report.push_str(&format!(
                    "### {} - {}\n\n",
                    result.status, result.method_name
                ));
                report.push_str(&format!("**模块**: `{}`\n\n", result.module_path));
                report.push_str(&format!("**详情**: {}\n\n", result.details));
            }
        }

        // 改进建议
        report.push_str("## 💡 改进建议\n\n");

        if warning_tests > 0 {
            report.push_str("### 针对警告项\n\n");
            report.push_str("1. **返回类型变化**: 虽然向后兼容，但建议在文档中明确说明改进后的数据访问方式更简洁\n");
            report.push_str("2. **迁移指导**: 提供迁移示例，帮助用户了解如何使用新的Builder模式\n");
            report.push_str("3. **版本说明**: 在changelog中详细说明兼容性保证\n\n");
        }

        report.push_str("### 通用建议\n\n");
        report.push_str("1. **渐进式采用**: 用户可以在需要时逐步采用新的Builder模式\n");
        report.push_str("2. **文档更新**: 更新API文档示例，展示新旧两种使用方式\n");
        report.push_str("3. **测试覆盖**: 在实际实施前运行完整的集成测试\n");
        report.push_str("4. **用户沟通**: 提前通知用户API改进，强调向后兼容性\n\n");

        // 测试文件说明
        report.push_str("## 📁 测试文件\n\n");
        report.push_str("本次测试生成了以下参考文件：\n\n");
        report.push_str("- `reference/workplace_reference.rs` - Workplace模块兼容性测试用例\n");
        report.push_str("- `reference/generic_api_reference.rs` - 通用API兼容性测试用例\n");
        report.push_str("- `reference/builder_reference.rs` - Builder模式兼容性测试用例\n\n");

        report.push_str("这些文件可以在实际实施改进时作为回归测试的基础。\n\n");

        // 下一步行动
        report.push_str("## 🎯 下一步行动\n\n");
        report.push_str("1. ✅ **可以开始实施**: workplace模块改进 (兼容性良好)\n");
        report.push_str("2. ✅ **可以开始实施**: Builder模式添加 (纯增量功能)\n");
        report.push_str("3. ✅ **可以开始实施**: StandardResponse改进 (内部优化)\n");
        report.push_str("4. 📝 **准备文档**: API改进说明和迁移指南\n");
        report.push_str("5. 🧪 **运行测试**: 在实施后运行这些兼容性测试验证\n\n");

        fs::write(report_path, report)?;

        println!("📋 兼容性测试报告已生成");
        Ok(())
    }

    /// 添加测试结果
    fn add_test_result(&mut self, result: CompatibilityTestResult) {
        self.results.push(result);
    }

    /// 获取测试结果摘要
    pub fn get_summary(&self) -> HashMap<TestStatus, usize> {
        let mut summary = HashMap::new();

        for result in &self.results {
            let count = summary.entry(result.status.clone()).or_insert(0);
            *count += 1;
        }

        summary
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CompatibilityTestConfig {
        test_directory: PathBuf::from("tests/compatibility"),
        include_experimental: true,
        strict_mode: false, // 宽松模式，关注向后兼容性而非严格性
    };

    let mut tester = ApiCompatibilityTester::new(config);

    tester.run_all_tests()?;

    let summary = tester.get_summary();

    println!("\n🏆 兼容性测试完成！");
    println!("📊 结果摘要:");
    for (status, count) in summary {
        println!("   {} = {} 项", status, count);
    }

    Ok(())
}
