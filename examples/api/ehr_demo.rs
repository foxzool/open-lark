use dotenvy::dotenv;
use log::{error, info};
use open_lark::{
    prelude::*,
    service::ehr::models::{EmployeeAttachmentRequest, EmployeeListRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🏢 飞书人事(标准版)API演示");
    println!("================================");

    // 1. 员工花名册管理演示
    println!("\n1. 员工花名册管理演示");
    println!("-------------------------------");

    // 基本员工列表查询
    let basic_request = EmployeeListRequest {
        page_size: Some(10),
        page_token: None,
        status: Some("active".to_string()),
        department_id: None,
        user_id_type: Some("open_id".to_string()),
        department_id_type: Some("open_department_id".to_string()),
        include_resigned: Some(false),
        fields: Some(vec![
            "name".to_string(),
            "employee_number".to_string(),
            "email".to_string(),
            "mobile".to_string(),
            "department_info".to_string(),
            "job_info".to_string(),
        ]),
    };

    match client
        .ehr
        .employee
        .list_employees(basic_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 获取员工列表成功");
            if let Some(data) = &response.data {
                if let Some(employees) = &data.employees.items {
                    println!("员工总数: {}", employees.len());
                    for (index, employee) in employees.iter().enumerate().take(3) {
                        println!("\n员工 #{}: ", index + 1);
                        if let Some(name) = &employee.name {
                            println!("  姓名: {}", name);
                        }
                        if let Some(employee_number) = &employee.employee_number {
                            println!("  工号: {}", employee_number);
                        }
                        if let Some(email) = &employee.email {
                            println!("  邮箱: {}", email);
                        }
                        if let Some(mobile) = &employee.mobile {
                            println!("  手机: {}", mobile);
                        }
                        if let Some(department_info) = &employee.department_info {
                            if let Some(department_name) = &department_info.department_name {
                                println!("  部门: {}", department_name);
                            }
                        }
                        if let Some(job_info) = &employee.job_info {
                            if let Some(job_title) = &job_info.job_title {
                                println!("  职位: {}", job_title);
                            }
                            if let Some(job_level) = &job_info.job_level {
                                println!("  职级: {}", job_level);
                            }
                        }
                        if let Some(status) = &employee.status {
                            if let Some(status_text) = &status.status {
                                println!("  状态: {}", status_text);
                            }
                        }
                    }
                }

                // 分页信息
                if let Some(has_more) = data.employees.has_more {
                    println!("\n分页信息:");
                    println!("  是否有更多数据: {}", has_more);
                    if let Some(page_token) = &data.employees.page_token {
                        println!("  下一页标记: {}", page_token);
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 获取员工列表失败: {:?}", err);
        }
    }

    // 2. 高级查询演示（包含详细信息）
    println!("\n2. 高级员工信息查询演示");
    println!("-------------------------------");

    let advanced_request = EmployeeListRequest {
        page_size: Some(5),
        page_token: None,
        status: Some("active".to_string()),
        department_id: Some("dept_example_123".to_string()),
        user_id_type: Some("open_id".to_string()),
        department_id_type: Some("open_department_id".to_string()),
        include_resigned: Some(false),
        fields: Some(vec![
            "name".to_string(),
            "employee_number".to_string(),
            "email".to_string(),
            "mobile".to_string(),
            "gender".to_string(),
            "birthday".to_string(),
            "department_info".to_string(),
            "job_info".to_string(),
            "hire_info".to_string(),
            "personal_info".to_string(),
            "education_info".to_string(),
            "work_experience".to_string(),
            "emergency_contact".to_string(),
            "bank_account".to_string(),
            "social_security".to_string(),
        ]),
    };

    match client
        .ehr
        .employee
        .list_employees(advanced_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 获取详细员工信息成功");
            if let Some(data) = &response.data {
                if let Some(employees) = &data.employees.items {
                    println!("查询到员工数: {}", employees.len());

                    // 展示第一个员工的详细信息
                    if let Some(employee) = employees.first() {
                        println!("\n📋 员工详细档案:");

                        // 基本信息
                        println!("基本信息:");
                        if let Some(name) = &employee.name {
                            println!("  姓名: {}", name);
                        }
                        if let Some(en_name) = &employee.en_name {
                            println!("  英文名: {}", en_name);
                        }
                        if let Some(gender) = &employee.gender {
                            println!("  性别: {}", gender);
                        }
                        if let Some(birthday) = &employee.birthday {
                            println!("  生日: {}", birthday);
                        }

                        // 入职信息
                        if let Some(hire_info) = &employee.hire_info {
                            println!("入职信息:");
                            if let Some(hire_date) = &hire_info.hire_date {
                                println!("  入职日期: {}", hire_date);
                            }
                            if let Some(contract_type) = &hire_info.contract_type {
                                println!("  合同类型: {}", contract_type);
                            }
                        }

                        // 个人信息
                        if let Some(personal_info) = &employee.personal_info {
                            println!("个人信息:");
                            if let Some(nationality) = &personal_info.nationality {
                                println!("  国籍: {}", nationality);
                            }
                            if let Some(marital_status) = &personal_info.marital_status {
                                println!("  婚姻状况: {}", marital_status);
                            }
                            if let Some(highest_education) = &personal_info.highest_education {
                                println!("  最高学历: {}", highest_education);
                            }
                        }

                        // 教育经历
                        if let Some(education_info) = &employee.education_info {
                            if !education_info.is_empty() {
                                println!("教育经历:");
                                for (i, edu) in education_info.iter().enumerate() {
                                    println!("  教育经历 {}:", i + 1);
                                    if let Some(school_name) = &edu.school_name {
                                        println!("    学校: {}", school_name);
                                    }
                                    if let Some(major) = &edu.major {
                                        println!("    专业: {}", major);
                                    }
                                    if let Some(degree) = &edu.degree {
                                        println!("    学历: {}", degree);
                                    }
                                }
                            }
                        }

                        // 工作经历
                        if let Some(work_experience) = &employee.work_experience {
                            if !work_experience.is_empty() {
                                println!("工作经历:");
                                for (i, work) in work_experience.iter().enumerate() {
                                    println!("  工作经历 {}:", i + 1);
                                    if let Some(company_name) = &work.company_name {
                                        println!("    公司: {}", company_name);
                                    }
                                    if let Some(position) = &work.position {
                                        println!("    职位: {}", position);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 获取详细员工信息失败: {:?}", err);
        }
    }

    // 3. 人员附件下载演示
    println!("\n3. 人员附件下载演示");
    println!("-------------------------------");

    let attachment_request = EmployeeAttachmentRequest {
        employee_id: "example_employee_id".to_string(),
        attachment_id: "example_attachment_id".to_string(),
        user_id_type: Some("open_id".to_string()),
    };

    match client
        .ehr
        .attachment
        .download_attachment(attachment_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 下载员工附件成功");
            if let Some(data) = &response.data {
                println!("附件信息:");
                if let Some(file_name) = &data.attachment.file_name {
                    println!("  文件名: {}", file_name);
                }
                if let Some(content_type) = &data.attachment.content_type {
                    println!("  文件类型: {}", content_type);
                }
                if let Some(file_size) = data.attachment.file_size {
                    println!("  文件大小: {} bytes", file_size);
                }
                if let Some(file_content) = &data.attachment.file_content {
                    println!("  文件内容长度: {} chars (base64编码)", file_content.len());
                    println!(
                        "  文件内容预览: {}...",
                        file_content.chars().take(50).collect::<String>()
                    );

                    // 实际应用中，这里可以将base64内容解码并保存到文件
                    info!("💾 提示: 在实际应用中，可以使用base64解码保存文件");
                }
            }
        }
        Err(err) => {
            error!("❌ 下载员工附件失败: {:?}", err);
            println!("注意: 这可能是因为使用了示例的员工ID和附件ID");
        }
    }

    // 4. 分页查询演示
    println!("\n4. 分页查询演示");
    println!("-------------------------------");

    let mut current_page_token: Option<String> = None;
    let mut page_count = 0;
    let max_pages = 3; // 限制演示页数

    loop {
        page_count += 1;
        if page_count > max_pages {
            println!("达到最大演示页数限制，停止分页查询");
            break;
        }

        let page_request = EmployeeListRequest {
            page_size: Some(5),
            page_token: current_page_token.clone(),
            status: Some("active".to_string()),
            department_id: None,
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("open_department_id".to_string()),
            include_resigned: Some(false),
            fields: Some(vec![
                "name".to_string(),
                "employee_number".to_string(),
                "email".to_string(),
            ]),
        };

        match client.ehr.employee.list_employees(page_request, None).await {
            Ok(response) => {
                println!("\n📄 第 {} 页数据:", page_count);
                if let Some(data) = &response.data {
                    if let Some(employees) = &data.employees.items {
                        for employee in employees {
                            if let Some(name) = &employee.name {
                                print!("  - {}", name);
                                if let Some(employee_number) = &employee.employee_number {
                                    print!(" ({})", employee_number);
                                }
                                println!();
                            }
                        }
                    }

                    // 检查是否有下一页
                    if let Some(has_more) = data.employees.has_more {
                        if has_more {
                            current_page_token = data.employees.page_token.clone();
                            println!("有更多数据，继续查询下一页...");
                        } else {
                            println!("已到达最后一页");
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            Err(err) => {
                error!("❌ 分页查询失败: {:?}", err);
                break;
            }
        }
    }

    // 5. 特定部门员工查询演示
    println!("\n5. 特定部门员工查询演示");
    println!("-------------------------------");

    let department_request = EmployeeListRequest {
        page_size: Some(20),
        page_token: None,
        status: None, // 查询所有状态
        department_id: Some("specific_dept_id".to_string()),
        user_id_type: Some("open_id".to_string()),
        department_id_type: Some("open_department_id".to_string()),
        include_resigned: Some(true), // 包含离职员工
        fields: Some(vec![
            "name".to_string(),
            "employee_number".to_string(),
            "status".to_string(),
            "department_info".to_string(),
            "job_info".to_string(),
        ]),
    };

    match client
        .ehr
        .employee
        .list_employees(department_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 查询部门员工成功");
            if let Some(data) = &response.data {
                if let Some(employees) = &data.employees.items {
                    println!("部门员工统计:");

                    let mut active_count = 0;
                    let mut inactive_count = 0;
                    let mut other_count = 0;

                    for employee in employees {
                        if let Some(status) = &employee.status {
                            if let Some(status_text) = &status.status {
                                match status_text.as_str() {
                                    "active" => active_count += 1,
                                    "inactive" => inactive_count += 1,
                                    _ => other_count += 1,
                                }
                            }
                        }
                    }

                    println!("  在职员工: {} 人", active_count);
                    println!("  离职员工: {} 人", inactive_count);
                    println!("  其他状态: {} 人", other_count);
                    println!("  总计: {} 人", employees.len());
                }
            }
        }
        Err(err) => {
            error!("❌ 查询部门员工失败: {:?}", err);
            println!("注意: 这可能是因为使用了示例的部门ID");
        }
    }

    println!("\n🎉 人事管理API演示完成!");
    println!("\n💡 提示:");
    println!("- 确保具有相应的人事管理权限");
    println!("- 在生产环境中请使用真实的员工ID和附件ID");
    println!("- 下载的附件内容为base64编码，需要解码后使用");
    println!("- 建议对敏感的人事数据进行加密存储和传输");

    Ok(())
}
