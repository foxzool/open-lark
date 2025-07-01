use dotenvy::dotenv;
use open_lark::{prelude::*, service::ehr::models::*};
use std::env;

/// 飞书人事(标准版) v1 API 综合演示
///
/// 本示例展示了飞书人事(标准版)系统的主要功能：
/// - 批量获取员工花名册信息
/// - 下载人员的附件文件
/// - 人事数据筛选和分页查询
/// - 完整的员工档案信息获取
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🚀 开始演示飞书人事(标准版) v1 API 功能");

    // ========== 员工花名册管理演示 ==========
    println!("\n👥 1. 员工花名册管理");

    // 1.1 获取所有员工信息（分页查询）
    let employee_request = EmployeeListRequest {
        page_size: Some(20),
        page_token: None,
        status: None,        // 获取所有状态员工
        department_id: None, // 获取所有部门员工
        user_id_type: Some("open_id".to_string()),
        department_id_type: Some("open_department_id".to_string()),
        include_resigned: Some(false), // 不包含离职员工
        fields: Some(vec![
            "name".to_string(),
            "employee_number".to_string(),
            "email".to_string(),
            "mobile".to_string(),
            "status".to_string(),
            "department_info".to_string(),
            "job_info".to_string(),
            "hire_info".to_string(),
        ]),
    };

    match client
        .ehr
        .employee
        .list_employees(employee_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 员工花名册获取成功:");
            if let Some(data) = &response.data {
                if let Some(items) = &data.employees.items {
                    println!("   - 员工数量: {}", items.len());
                    println!(
                        "   - 是否还有更多: {:?}",
                        data.employees.has_more.unwrap_or(false)
                    );

                    for (i, employee) in items.iter().take(5).enumerate() {
                        println!("\n   {}. 员工信息:", i + 1);
                        if let Some(name) = &employee.name {
                            println!("      姓名: {name}");
                        }
                        if let Some(employee_number) = &employee.employee_number {
                            println!("      工号: {employee_number}");
                        }
                        if let Some(email) = &employee.email {
                            println!("      邮箱: {email}");
                        }
                        if let Some(mobile) = &employee.mobile {
                            println!("      手机: {mobile}");
                        }

                        // 显示部门信息
                        if let Some(dept_info) = &employee.department_info {
                            if let Some(dept_name) = &dept_info.department_name {
                                println!("      部门: {dept_name}");
                            }
                        }

                        // 显示职位信息
                        if let Some(job_info) = &employee.job_info {
                            if let Some(job_title) = &job_info.job_title {
                                println!("      职位: {job_title}");
                            }
                            if let Some(supervisor_name) = &job_info.supervisor_name {
                                println!("      上级: {supervisor_name}");
                            }
                        }

                        // 显示入职信息
                        if let Some(hire_info) = &employee.hire_info {
                            if let Some(hire_date) = &hire_info.hire_date {
                                println!("      入职日期: {hire_date}");
                            }
                        }
                    }
                } else {
                    println!("   未查询到员工信息");
                }
            }
        }
        Err(e) => println!("❌ 员工花名册获取失败: {e:?}"),
    }

    // 1.2 按部门筛选员工
    println!("\n🏢 2. 按部门筛选员工");

    let dept_employee_request = EmployeeListRequest {
        page_size: Some(10),
        page_token: None,
        status: Some("active".to_string()), // 只查询在职员工
        department_id: Some("od-example".to_string()), // 示例部门ID
        user_id_type: Some("open_id".to_string()),
        department_id_type: Some("open_department_id".to_string()),
        include_resigned: Some(false),
        fields: Some(vec![
            "name".to_string(),
            "employee_number".to_string(),
            "department_info".to_string(),
            "job_info".to_string(),
            "status".to_string(),
        ]),
    };

    match client
        .ehr
        .employee
        .list_employees(dept_employee_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 部门员工查询成功:");
            if let Some(data) = &response.data {
                if let Some(items) = &data.employees.items {
                    println!("   - 部门员工数量: {}", items.len());
                    for employee in items {
                        if let Some(name) = &employee.name {
                            print!("   - {name}");
                            if let Some(job_info) = &employee.job_info {
                                if let Some(job_title) = &job_info.job_title {
                                    print!(" ({job_title})");
                                }
                            }
                            println!();
                        }
                    }
                } else {
                    println!("   该部门暂无员工");
                }
            }
        }
        Err(e) => println!("❌ 部门员工查询失败: {e:?}"),
    }

    // 1.3 获取详细员工信息（包含个人信息、教育经历等）
    println!("\n📋 3. 获取详细员工档案");

    let detailed_request = EmployeeListRequest {
        page_size: Some(3),
        page_token: None,
        status: Some("active".to_string()),
        department_id: None,
        user_id_type: Some("open_id".to_string()),
        department_id_type: Some("open_department_id".to_string()),
        include_resigned: Some(false),
        fields: Some(vec![
            "name".to_string(),
            "personal_info".to_string(),
            "education_info".to_string(),
            "work_experience".to_string(),
            "emergency_contact".to_string(),
            "bank_account".to_string(),
            "social_security".to_string(),
            "custom_fields".to_string(),
        ]),
    };

    match client
        .ehr
        .employee
        .list_employees(detailed_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 详细员工档案获取成功:");
            if let Some(data) = &response.data {
                if let Some(items) = &data.employees.items {
                    for (i, employee) in items.iter().enumerate() {
                        println!("\n   {}. 员工档案:", i + 1);
                        if let Some(name) = &employee.name {
                            println!("      姓名: {name}");
                        }

                        // 个人信息
                        if let Some(personal) = &employee.personal_info {
                            println!("      个人信息:");
                            if let Some(nationality) = &personal.nationality {
                                println!("        国籍: {nationality}");
                            }
                            if let Some(highest_education) = &personal.highest_education {
                                println!("        最高学历: {highest_education}");
                            }
                        }

                        // 教育经历
                        if let Some(educations) = &employee.education_info {
                            if !educations.is_empty() {
                                println!("      教育经历:");
                                for edu in educations.iter().take(2) {
                                    if let Some(school) = &edu.school_name {
                                        if let Some(major) = &edu.major {
                                            println!("        - {school} {major}");
                                        }
                                    }
                                }
                            }
                        }

                        // 工作经历
                        if let Some(experiences) = &employee.work_experience {
                            if !experiences.is_empty() {
                                println!("      工作经历:");
                                for exp in experiences.iter().take(2) {
                                    if let Some(company) = &exp.company_name {
                                        if let Some(position) = &exp.position {
                                            println!("        - {company} {position}");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => println!("❌ 详细员工档案获取失败: {e:?}"),
    }

    // ========== 人员附件管理演示 ==========
    println!("\n📎 4. 人员附件管理");

    // 4.1 下载员工附件示例
    let attachment_request = EmployeeAttachmentRequest {
        employee_id: "emp_example_123".to_string(),
        attachment_id: "attach_example_456".to_string(),
        user_id_type: Some("open_id".to_string()),
    };

    match client
        .ehr
        .attachment
        .download_attachment(attachment_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 员工附件下载成功:");
            if let Some(data) = &response.data {
                if let Some(file_name) = &data.attachment.file_name {
                    println!("   - 文件名: {file_name}");
                }
                if let Some(content_type) = &data.attachment.content_type {
                    println!("   - 文件类型: {content_type}");
                }
                if let Some(file_size) = &data.attachment.file_size {
                    println!("   - 文件大小: {file_size} 字节");
                }
                if let Some(file_content) = &data.attachment.file_content {
                    println!("   - 文件内容: base64编码 ({} 字符)", file_content.len());

                    // 示例：保存文件（实际使用时请解码base64）
                    println!("   💡 提示: 可使用 base64::decode() 解码文件内容");
                }
            }
        }
        Err(e) => println!("❌ 员工附件下载失败: {e:?} (这是正常的，因为使用了示例ID)"),
    }

    println!("\n🎉 飞书人事(标准版) v1 API 综合演示完成!");
    println!("本示例展示了:");
    println!("  👥 员工花名册管理: 批量获取、筛选查询、分页处理");
    println!("  📋 完整档案信息: 基本信息、部门职位、个人档案、教育经历");
    println!("  📎 附件文件管理: 安全下载员工相关文档");
    println!("  🔧 灵活配置参数: 支持多种ID类型和自定义字段");
    println!("  🔒 权限控制: 租户级别的安全访问控制");

    println!("\n💡 使用建议:");
    println!("  - 合理设置分页大小，避免一次查询过多数据");
    println!("  - 根据需求选择合适的fields参数，提高查询效率");
    println!("  - 妥善处理敏感的人事档案数据，确保数据安全");
    println!("  - 对下载的附件文件进行适当的安全检查");

    Ok(())
}
