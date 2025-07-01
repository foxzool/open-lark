use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::directory::v1::{employee::*, models::*},
};
use std::env;

/// 演示飞书 directory v1 员工管理扩展接口
/// 包括恢复离职员工、更新员工状态、批量获取和搜索等功能
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    // 从环境变量获取配置
    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    // 初始化客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🚀 开始演示飞书 directory v1 员工管理扩展接口...");

    // 1. 演示恢复离职员工
    println!("\n📋 1. 恢复离职员工");
    if let Ok(response) = client
        .directory
        .v1
        .employee
        .resurrect(
            ResurrectEmployeeRequest::builder("employee_id_example")
                .leader_id("leader_id_example")
                .department_ids(vec!["department_id_example".to_string()])
                .work_location("北京")
                .job_level("P6")
                .job_title("高级工程师")
                .user_id_type(UserIdType::UserId)
                .department_id_type(DepartmentIdType::DepartmentId)
                .build(),
            None,
        )
        .await
    {
        println!("✅ 恢复离职员工成功");
        if let Some(data) = response.data {
            println!("   员工ID: {:?}", data.data.employee.employee_id);
            println!("   员工姓名: {:?}", data.data.employee.name);
        }
    } else {
        println!("❌ 恢复离职员工失败（可能员工ID不存在）");
    }

    // 2. 演示更新在职员工为待离职
    println!("\n📋 2. 更新在职员工为待离职");
    if let Ok(response) = client
        .directory
        .v1
        .employee
        .to_be_resigned(
            ToBeResignedEmployeeRequest::builder("employee_id_example")
                .resign_time(1640995200000) // 2022-01-01 00:00:00
                .resign_reason("个人原因")
                .resign_type("主动离职")
                .user_id_type(UserIdType::UserId)
                .build(),
            None,
        )
        .await
    {
        println!("✅ 更新员工为待离职成功");
        if let Some(data) = response.data {
            println!("   员工ID: {:?}", data.data.employee.employee_id);
            println!("   员工姓名: {:?}", data.data.employee.name);
        }
    } else {
        println!("❌ 更新员工为待离职失败（可能员工ID不存在）");
    }

    // 3. 演示更新待离职成员为在职
    println!("\n📋 3. 更新待离职成员为在职");
    if let Ok(response) = client
        .directory
        .v1
        .employee
        .regular(
            RegularEmployeeRequest::builder("employee_id_example")
                .user_id_type(UserIdType::UserId)
                .build(),
            None,
        )
        .await
    {
        println!("✅ 更新待离职成员为在职成功");
        if let Some(data) = response.data {
            println!("   员工ID: {:?}", data.data.employee.employee_id);
            println!("   员工姓名: {:?}", data.data.employee.name);
        }
    } else {
        println!("❌ 更新待离职成员为在职失败（可能员工ID不存在）");
    }

    // 4. 演示批量获取员工信息
    println!("\n📋 4. 批量获取员工信息");
    if let Ok(response) = client
        .directory
        .v1
        .employee
        .mget(
            MgetEmployeeRequest::builder(vec![
                "employee_id_1".to_string(),
                "employee_id_2".to_string(),
                "employee_id_3".to_string(),
            ])
            .user_id_type(UserIdType::UserId)
            .department_id_type(DepartmentIdType::DepartmentId)
            .build(),
            None,
        )
        .await
    {
        println!("✅ 批量获取员工信息成功");
        if let Some(data) = response.data {
            println!("   获取到 {} 个员工信息", data.data.employees.len());
            for (index, employee) in data.data.employees.iter().enumerate() {
                println!(
                    "   员工{}: {:?} ({})",
                    index + 1,
                    employee.name.as_ref().unwrap_or(&"未知".to_string()),
                    employee.employee_id.as_ref().unwrap_or(&"无ID".to_string())
                );
            }
        }
    } else {
        println!("❌ 批量获取员工信息失败（可能员工ID不存在）");
    }

    // 5. 演示搜索员工
    println!("\n📋 5. 搜索员工");
    if let Ok(response) = client
        .directory
        .v1
        .employee
        .search(
            SearchEmployeeRequest::builder("张三")
                .department_id("department_id_example")
                .page_size(20)
                .user_id_type(UserIdType::UserId)
                .department_id_type(DepartmentIdType::DepartmentId)
                .build(),
            None,
        )
        .await
    {
        println!("✅ 搜索员工成功");
        if let Some(data) = response.data {
            println!("   找到 {} 个员工", data.data.employees.len());
            println!("   是否还有更多: {}", data.data.has_more);
            if let Some(page_token) = &data.data.page_token {
                println!("   下一页Token: {page_token}");
            }

            for (index, employee) in data.data.employees.iter().enumerate() {
                println!(
                    "   员工{}: {:?} ({})",
                    index + 1,
                    employee.name.as_ref().unwrap_or(&"未知".to_string()),
                    employee.employee_id.as_ref().unwrap_or(&"无ID".to_string())
                );
            }
        }
    } else {
        println!("❌ 搜索员工失败");
    }

    // 6. 演示使用 Builder 模式的链式调用
    println!("\n📋 6. 演示Builder模式链式调用");
    let request = ResurrectEmployeeRequest::builder("employee_id_example")
        .leader_id("leader_id")
        .department_ids(vec!["dept_1".to_string(), "dept_2".to_string()])
        .work_location("上海")
        .job_level("P7")
        .job_title("资深工程师")
        .user_id_type(UserIdType::UserId)
        .department_id_type(DepartmentIdType::DepartmentId)
        .build();

    println!("✅ Builder模式构建请求成功");
    println!("   员工ID: {}", request.employee_id);
    println!("   上级ID: {:?}", request.leader_id);
    println!("   部门列表: {:?}", request.department_ids);

    println!("\n🎉 飞书 directory v1 员工管理扩展接口演示完成！");
    println!("💡 提示: 实际使用时请替换为真实的员工ID、部门ID等参数");

    Ok(())
}
