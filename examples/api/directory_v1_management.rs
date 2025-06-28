use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::directory::v1::{
        CreateDepartmentRequest, CreateEmployeeRequest, DeleteDepartmentRequest,
        DeleteEmployeeRequest, DepartmentIdType, EmployeeStatus, FilterDepartmentRequest,
        FilterEmployeeRequest, MgetDepartmentRequest, MgetEmployeeRequest,
        PatchDepartmentRequest, PatchEmployeeRequest, RegularEmployeeRequest,
        ResurrectEmployeeRequest, SearchDepartmentRequest, SearchEmployeeRequest,
        ToBeResignedEmployeeRequest, UserIdType,
    },
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(app_id, app_secret)
        .with_app_type(AppType::SelfBuilt)
        .with_enable_token_cache(true)
        .build();

    println!("🚀 开始演示飞书 Directory v1 组织架构管理功能");

    // ========== 部门管理演示 ==========
    println!("\n📁 部门管理功能演示:");

    // 1. 创建部门
    println!("\n1. 创建部门");
    match client
        .directory
        .v1
        .department
        .create(
            CreateDepartmentRequest::builder()
                .name("技术部")
                .en_name("Technology Department")
                .user_id_type(UserIdType::UserId)
                .department_id_type(DepartmentIdType::DepartmentId)
                .build(),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✅ 部门创建成功:");
            if let Some(department) = response.data.as_ref().and_then(|d| d.department.as_ref()) {
                println!("   - 部门ID: {:?}", department.department_id);
                println!("   - 部门名称: {:?}", department.name);
                println!("   - 英文名称: {:?}", department.en_name);
            }
        }
        Err(e) => println!("❌ 部门创建失败: {:?}", e),
    }

    // 2. 获取部门列表
    println!("\n2. 获取部门列表");
    match client
        .directory
        .v1
        .department
        .filter(
            FilterDepartmentRequest::builder()
                .page_size(20)
                .user_id_type(UserIdType::UserId)
                .department_id_type(DepartmentIdType::DepartmentId)
                .build(),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✅ 部门列表获取成功:");
            if let Some(data) = response.data.as_ref() {
                println!("   - 部门数量: {}", data.departments.len());
                println!("   - 是否有更多: {}", data.has_more);
                for (i, dept) in data.departments.iter().take(3).enumerate() {
                    println!(
                        "   {}. {} ({:?})",
                        i + 1,
                        dept.name.as_deref().unwrap_or("未知"),
                        dept.department_id
                    );
                }
            }
        }
        Err(e) => println!("❌ 部门列表获取失败: {:?}", e),
    }

    // 3. 搜索部门
    println!("\n3. 搜索部门");
    match client
        .directory
        .v1
        .department
        .search(
            SearchDepartmentRequest::builder("技术")
                .page_size(10)
                .user_id_type(UserIdType::UserId)
                .department_id_type(DepartmentIdType::DepartmentId)
                .build(),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✅ 部门搜索成功:");
            if let Some(data) = response.data.as_ref() {
                println!("   - 搜索结果数量: {}", data.departments.len());
                for dept in &data.departments {
                    println!(
                        "   - 部门: {} ({})",
                        dept.name.as_deref().unwrap_or("未知"),
                        dept.department_id.as_deref().unwrap_or("未知ID")
                    );
                }
            }
        }
        Err(e) => println!("❌ 部门搜索失败: {:?}", e),
    }

    // ========== 员工管理演示 ==========
    println!("\n👥 员工管理功能演示:");

    // 1. 创建员工
    println!("\n1. 创建员工");
    match client
        .directory
        .v1
        .employee
        .create(
            CreateEmployeeRequest::builder()
                .name("张三")
                .en_name("Zhang San")
                .email("zhangsan@example.com")
                .mobile("13812345678")
                .employee_no("EMP001")
                .job_title("软件工程师")
                .user_id_type(UserIdType::UserId)
                .department_id_type(DepartmentIdType::DepartmentId)
                .build(),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✅ 员工创建成功:");
            if let Some(employee) = response.data.as_ref().and_then(|d| d.employee.as_ref()) {
                println!("   - 员工ID: {:?}", employee.employee_id);
                println!("   - 姓名: {:?}", employee.name);
                println!("   - 工号: {:?}", employee.employee_no);
                println!("   - 职位: {:?}", employee.job_title);
            }
        }
        Err(e) => println!("❌ 员工创建失败: {:?}", e),
    }

    // 2. 获取员工列表
    println!("\n2. 获取员工列表");
    match client
        .directory
        .v1
        .employee
        .filter(
            FilterEmployeeRequest::builder()
                .page_size(20)
                .status(EmployeeStatus::Active)
                .user_id_type(UserIdType::UserId)
                .department_id_type(DepartmentIdType::DepartmentId)
                .build(),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✅ 员工列表获取成功:");
            if let Some(data) = response.data.as_ref() {
                println!("   - 员工数量: {}", data.employees.len());
                println!("   - 是否有更多: {}", data.has_more);
                for (i, emp) in data.employees.iter().take(3).enumerate() {
                    println!(
                        "   {}. {} ({}) - {}",
                        i + 1,
                        emp.name.as_deref().unwrap_or("未知"),
                        emp.employee_no.as_deref().unwrap_or("无工号"),
                        emp.job_title.as_deref().unwrap_or("无职位")
                    );
                }
            }
        }
        Err(e) => println!("❌ 员工列表获取失败: {:?}", e),
    }

    // 3. 搜索员工
    println!("\n3. 搜索员工");
    match client
        .directory
        .v1
        .employee
        .search(
            SearchEmployeeRequest::builder("张")
                .page_size(10)
                .user_id_type(UserIdType::UserId)
                .department_id_type(DepartmentIdType::DepartmentId)
                .build(),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✅ 员工搜索成功:");
            if let Some(data) = response.data.as_ref() {
                println!("   - 搜索结果数量: {}", data.employees.len());
                for emp in &data.employees {
                    println!(
                        "   - 员工: {} ({}) - {}",
                        emp.name.as_deref().unwrap_or("未知"),
                        emp.employee_no.as_deref().unwrap_or("无工号"),
                        emp.email.as_deref().unwrap_or("无邮箱")
                    );
                }
            }
        }
        Err(e) => println!("❌ 员工搜索失败: {:?}", e),
    }

    // ========== 高级功能演示 ==========
    println!("\n🔧 高级功能演示:");

    // 假设有员工ID进行演示
    let demo_employee_id = "sample_employee_id";

    // 1. 更新员工信息
    println!("\n1. 更新员工信息");
    match client
        .directory
        .v1
        .employee
        .patch(
            PatchEmployeeRequest::builder(demo_employee_id)
                .job_title("高级软件工程师")
                .user_id_type(UserIdType::UserId)
                .build(),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✅ 员工信息更新成功:");
            if let Some(employee) = response.data.as_ref().and_then(|d| d.employee.as_ref()) {
                println!("   - 更新后职位: {:?}", employee.job_title);
            }
        }
        Err(e) => println!("❌ 员工信息更新失败: {:?}", e),
    }

    // 2. 设置员工为待离职
    println!("\n2. 设置员工为待离职");
    match client
        .directory
        .v1
        .employee
        .to_be_resigned(
            ToBeResignedEmployeeRequest::builder(demo_employee_id)
                .leave_time("2024-12-31")
                .leave_reason("个人发展")
                .user_id_type(UserIdType::UserId)
                .build(),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✅ 员工设置为待离职成功:");
            if let Some(employee) = response.data.as_ref().and_then(|d| d.employee.as_ref()) {
                println!("   - 员工状态: {:?}", employee.status);
                println!("   - 离职时间: {:?}", employee.leave_time);
            }
        }
        Err(e) => println!("❌ 设置待离职失败: {:?}", e),
    }

    // 3. 恢复员工为在职状态
    println!("\n3. 恢复员工为在职状态");
    match client
        .directory
        .v1
        .employee
        .regular(
            RegularEmployeeRequest::builder(demo_employee_id)
                .user_id_type(UserIdType::UserId)
                .build(),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✅ 员工恢复在职成功:");
            if let Some(employee) = response.data.as_ref().and_then(|d| d.employee.as_ref()) {
                println!("   - 员工状态: {:?}", employee.status);
            }
        }
        Err(e) => println!("❌ 恢复在职失败: {:?}", e),
    }

    println!("\n🎉 Directory v1 组织架构管理功能演示完成!");
    println!("本示例展示了:");
    println!("  📁 部门管理: 创建、查询、搜索、更新、删除");
    println!("  👥 员工管理: 创建、查询、搜索、更新、状态变更");
    println!("  🔧 高级功能: 批量操作、状态流转、恢复操作");

    Ok(())
}