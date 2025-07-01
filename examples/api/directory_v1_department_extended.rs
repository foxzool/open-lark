use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::directory::v1::{department::*, models::*},
};
use std::env;

/// 演示飞书 directory v1 部门管理扩展接口
/// 包括更新、删除、批量获取、过滤和搜索等功能
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("Need APP_ID env");
    let app_secret = env::var("APP_SECRET").expect("Need APP_SECRET env");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("🚀 开始演示飞书 directory v1 部门管理扩展接口");

    // 1. 演示创建部门（用于后续操作）
    println!("\n📝 1. 创建部门");
    let create_request = CreateDepartmentRequest::builder()
        .name("测试技术部")
        .en_name("Test Technology Department")
        .parent_department_id("0") // 根部门
        .user_id_type(UserIdType::UserId)
        .department_id_type(DepartmentIdType::DepartmentId)
        .build();

    let mut department_id = String::new();
    if let Ok(response) = client
        .directory
        .v1
        .department
        .create(create_request, None)
        .await
    {
        println!("✅ 创建部门成功");
        if let Some(data) = response.data {
            if let Some(dept_id) = &data.data.department.department_id {
                department_id = dept_id.clone();
                println!("   新部门ID: {department_id}");
                println!(
                    "   部门名称: {}",
                    data.data
                        .department
                        .name
                        .as_ref()
                        .unwrap_or(&"未知".to_string())
                );
            }
        }
    } else {
        println!("❌ 创建部门失败，使用示例部门ID进行后续演示");
        department_id = "example_dept_id".to_string();
    }

    // 2. 演示更新部门
    println!("\n✏️ 2. 更新部门信息");
    let patch_request = PatchDepartmentRequest::builder(&department_id)
        .name("更新后的技术部")
        .en_name("Updated Technology Department")
        .order(100)
        .user_id_type(UserIdType::UserId)
        .department_id_type(DepartmentIdType::DepartmentId)
        .build();

    if let Ok(response) = client
        .directory
        .v1
        .department
        .patch(patch_request, None)
        .await
    {
        println!("✅ 更新部门成功");
        if let Some(data) = response.data {
            println!(
                "   更新后名称: {}",
                data.data
                    .department
                    .name
                    .as_ref()
                    .unwrap_or(&"未知".to_string())
            );
            println!("   更新后顺序: {}", data.data.department.order.unwrap_or(0));
        }
    } else {
        println!("❌ 更新部门失败（可能部门ID不存在）");
    }

    // 3. 演示批量获取部门信息
    println!("\n📋 3. 批量获取部门信息");
    let mget_request = MgetDepartmentRequest::builder()
        .department_ids(vec![department_id.clone(), "another_dept_id".to_string()])
        .user_id_type(UserIdType::UserId)
        .department_id_type(DepartmentIdType::DepartmentId)
        .build();

    if let Ok(response) = client
        .directory
        .v1
        .department
        .mget(mget_request, None)
        .await
    {
        println!("✅ 批量获取部门信息成功");
        if let Some(data) = response.data {
            println!("   获取到 {} 个部门信息", data.data.departments.len());
            for (index, department) in data.data.departments.iter().enumerate() {
                println!(
                    "   部门{}: {} ({})",
                    index + 1,
                    department.name.as_ref().unwrap_or(&"未知".to_string()),
                    department
                        .department_id
                        .as_ref()
                        .unwrap_or(&"无ID".to_string())
                );
            }
        }
    } else {
        println!("❌ 批量获取部门信息失败（可能部门ID不存在）");
    }

    // 4. 演示获取部门列表（过滤）
    println!("\n📄 4. 获取部门列表（过滤）");
    let filter_request = FilterDepartmentRequest::builder()
        .page_size(20)
        .parent_department_id("0") // 获取根部门下的子部门
        .fetch_deleted(false)
        .user_id_type(UserIdType::UserId)
        .department_id_type(DepartmentIdType::DepartmentId)
        .build();

    if let Ok(response) = client
        .directory
        .v1
        .department
        .filter(filter_request, None)
        .await
    {
        println!("✅ 获取部门列表成功");
        if let Some(data) = response.data {
            println!("   找到 {} 个部门", data.data.departments.len());
            println!("   是否还有更多: {}", data.data.has_more.unwrap_or(false));
            if let Some(page_token) = &data.data.page_token {
                println!("   下一页Token: {page_token}");
            }

            for (index, department) in data.data.departments.iter().enumerate() {
                println!(
                    "   部门{}: {} ({})",
                    index + 1,
                    department.name.as_ref().unwrap_or(&"未知".to_string()),
                    department
                        .department_id
                        .as_ref()
                        .unwrap_or(&"无ID".to_string())
                );
            }
        }
    } else {
        println!("❌ 获取部门列表失败");
    }

    // 5. 演示搜索部门
    println!("\n🔍 5. 搜索部门");
    let search_request = SearchDepartmentRequest::builder("技术")
        .page_size(10)
        .user_id_type(UserIdType::UserId)
        .department_id_type(DepartmentIdType::DepartmentId)
        .build();

    if let Ok(response) = client
        .directory
        .v1
        .department
        .search(search_request, None)
        .await
    {
        println!("✅ 搜索部门成功");
        if let Some(data) = response.data {
            println!("   找到 {} 个匹配部门", data.data.departments.len());
            println!("   是否还有更多: {}", data.data.has_more.unwrap_or(false));
            if let Some(page_token) = &data.data.page_token {
                println!("   下一页Token: {page_token}");
            }

            for (index, department) in data.data.departments.iter().enumerate() {
                println!(
                    "   部门{}: {} ({})",
                    index + 1,
                    department.name.as_ref().unwrap_or(&"未知".to_string()),
                    department
                        .department_id
                        .as_ref()
                        .unwrap_or(&"无ID".to_string())
                );
            }
        }
    } else {
        println!("❌ 搜索部门失败");
    }

    // 6. 演示删除部门（最后执行）
    println!("\n🗑️ 6. 删除部门");
    let delete_request = DeleteDepartmentRequest::builder(&department_id)
        .department_id_type(DepartmentIdType::DepartmentId)
        .build();

    if let Ok(response) = client
        .directory
        .v1
        .department
        .delete(delete_request, None)
        .await
    {
        println!("✅ 删除部门成功");
        if let Some(data) = response.data {
            println!("   删除结果: {:?}", data.deleted);
        }
    } else {
        println!("❌ 删除部门失败（可能部门ID不存在或有子部门/员工）");
    }

    // 7. 演示Builder模式构建复杂请求
    println!("\n🔨 7. Builder模式构建复杂请求");
    let complex_request = FilterDepartmentRequest::builder()
        .page_size(50)
        .page_token("example_token")
        .parent_department_id("root_dept_id")
        .fetch_deleted(true)
        .user_id_type(UserIdType::OpenId)
        .department_id_type(DepartmentIdType::OpenDepartmentId)
        .build();

    println!("✅ Builder模式构建请求成功");
    println!("   分页大小: {}", complex_request.page_size.unwrap_or(0));
    println!(
        "   父部门ID: {}",
        complex_request
            .parent_department_id
            .as_ref()
            .unwrap_or(&"无".to_string())
    );
    println!(
        "   获取已删除: {}",
        complex_request.fetch_deleted.unwrap_or(false)
    );

    println!("\n🎉 飞书 directory v1 部门管理扩展接口演示完成！");
    println!("💡 提示: 实际使用时请替换为真实的部门ID、父部门ID等参数");

    Ok(())
}
