/// 组织架构模块演示
///
/// 展示directory v1 API的基本使用方法：
/// - 创建员工
/// - 批量获取员工列表
/// - 创建部门
///
/// 使用方法：
/// ```bash
/// cargo run --example directory_demo
/// ```
///
/// 环境变量要求：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
use dotenvy::dotenv;
use open_lark::{
    client::LarkClient,
    service::directory::v1::{
        department::CreateDepartmentRequest,
        employee::{CreateEmployeeRequest, FilterEmployeeRequest},
        models::{EmployeeStatus, UserIdType},
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("请设置 APP_ID 环境变量");
    let app_secret = std::env::var("APP_SECRET").expect("请设置 APP_SECRET 环境变量");

    // 创建Lark客户端
    let _client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(open_lark::core::constants::AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🏢 组织架构模块演示");
    println!("================");
    println!();

    // 演示组织架构服务初始化
    println!("📋 组织架构服务初始化:");
    println!("✅ DirectoryService 已成功集成到 LarkClient");
    println!("✅ 支持的功能模块:");
    println!("   - 员工管理 (employee)");
    println!("   - 部门管理 (department)");
    println!();

    // 演示Builder模式的使用
    println!("🔧 Builder模式演示:");
    println!("```rust");
    println!("// 1. 创建员工");
    println!("let employee_request = CreateEmployeeRequest::builder()");
    println!("    .name(\"张三\")");
    println!("    .email(\"zhangsan@example.com\")");
    println!("    .employee_no(\"001\")");
    println!("    .job_title(\"软件工程师\")");
    println!("    .user_id_type(UserIdType::UserId)");
    println!("    .build();");
    println!();
    println!("// 2. 批量获取员工列表");
    println!("let filter_request = FilterEmployeeRequest::builder()");
    println!("    .page_size(20)");
    println!("    .status(EmployeeStatus::Active)");
    println!("    .build();");
    println!();
    println!("// 3. 创建部门");
    println!("let department_request = CreateDepartmentRequest::builder()");
    println!("    .name(\"技术部\")");
    println!("    .en_name(\"Technology Department\")");
    println!("    .build();");
    println!();
    println!("// 使用execute方法调用");
    println!("let response = request.execute(&client.directory.v1.employee).await?;");
    println!("```");
    println!();

    // 实际构建请求（不执行）
    let _employee_request = CreateEmployeeRequest::builder()
        .name("张三")
        .email("zhangsan@example.com")
        .employee_no("001")
        .job_title("软件工程师")
        .user_id_type(UserIdType::UserId)
        .build();

    let _filter_request = FilterEmployeeRequest::builder()
        .page_size(20)
        .status(EmployeeStatus::Active)
        .build();

    let _department_request = CreateDepartmentRequest::builder()
        .name("技术部")
        .en_name("Technology Department")
        .build();

    println!("✅ Builder模式构建成功 (employee, filter, department)");
    println!();

    // 演示API调用结构
    println!("📡 API调用结构:");
    println!("- 基础路径: /open-apis/directory/v1/");
    println!("- 支持的HTTP方法: GET, POST, PATCH, DELETE");
    println!("- 认证方式: Tenant Access Token");
    println!("- 返回格式: 标准飞书API响应格式");
    println!();

    // 演示服务访问路径
    println!("🌐 服务访问路径:");
    println!("client.directory.v1.employee            // 员工管理");
    println!("client.directory.v1.department          // 部门管理");
    println!();

    // 演示API功能
    println!("📋 支持的API功能:");
    println!("🔹 员工管理:");
    println!("  - create            ✅ 创建员工");
    println!("  - filter            ✅ 批量获取员工列表");
    println!("  - patch             🔧 更新员工 (待实现)");
    println!("  - delete            🔧 离职员工 (待实现)");
    println!("  - resurrect         🔧 恢复离职员工 (待实现)");
    println!("  - to_be_resigned    🔧 更新在职员工为待离职 (待实现)");
    println!("  - regular           🔧 更新待离职成员为在职 (待实现)");
    println!("  - mget              🔧 批量获取员工信息 (待实现)");
    println!("  - search            🔧 搜索员工 (待实现)");
    println!();
    println!("🔹 部门管理:");
    println!("  - create            ✅ 创建部门");
    println!("  - patch             🔧 更新部门 (待实现)");
    println!("  - delete            🔧 删除部门 (待实现)");
    println!("  - mget              🔧 批量获取部门信息 (待实现)");
    println!("  - filter            🔧 批量获取部门列表 (待实现)");
    println!("  - search            🔧 搜索部门 (待实现)");
    println!();

    // 演示数据模型
    println!("📊 数据模型:");
    println!("- Employee: 员工信息模型");
    println!("- Department: 部门信息模型");
    println!("- EmployeeStatus: 员工状态枚举 (Active, Inactive, ToBeResigned)");
    println!("- DepartmentStatus: 部门状态枚举 (Normal, Deleted)");
    println!("- UserIdType: 用户ID类型枚举");
    println!("- DepartmentIdType: 部门ID类型枚举");
    println!();

    println!("🎉 组织架构模块演示完成！");
    println!();
    println!("💡 提示:");
    println!("  1. 已完成基础架构和核心功能: 员工创建、列表查询、部门创建");
    println!("  2. 其他API功能待实现 (patch, delete, search等)");
    println!("  3. 所有功能都支持Builder模式和ExecutableBuilder trait");
    println!("  4. 遵循open-lark SDK的统一架构模式");
    println!("  5. 支持完整的错误处理和响应格式");

    Ok(())
}
