use dotenvy::dotenv;
use log::error;
use open_lark::{
    prelude::*,
    service::corehr::models::{
        CompanyCreateRequest, CountryRegionSearchRequest, DepartmentCreateRequest,
        EmployeeBatchGetRequest, EmployeeSearchRequest, Employment, EnumSearchRequest, I18nText,
        IdConvertRequest, JobChangeCreateRequest, JobData, JobFamilyCreateRequest,
        JobLevelCreateRequest, NationalitySearchRequest, Person, PreHireCreateRequest,
    },
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

    println!("🏢 飞书人事企业版(CoreHR) API演示");
    println!("=====================================");

    // 1. 基础数据查询演示
    println!("\n1. 基础数据查询演示");
    println!("----------------------------------");

    // 查询枚举信息 - 性别
    let enum_request = EnumSearchRequest {
        enum_type: "gender".to_string(),
        page_size: Some(10),
        page_token: None,
    };

    match client
        .corehr
        .basic_info
        .search_enum(enum_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 查询性别枚举信息成功");
            if let Some(data) = &response.data {
                if let Some(enums) = &data.enums.items {
                    println!("性别选项:");
                    for enum_info in enums {
                        if let Some(enum_value) = &enum_info.enum_value {
                            print!("  - 值: {enum_value}");
                            if let Some(content) = &enum_info.content {
                                if let Some(zh_cn) = &content.zh_cn {
                                    print!(" ({zh_cn})");
                                }
                                if let Some(en_us) = &content.en_us {
                                    print!(" [{en_us}]");
                                }
                            }
                            println!();
                        }
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 查询枚举信息失败: {err:?}");
        }
    }

    // 查询国家/地区信息
    let country_request = CountryRegionSearchRequest {
        page_size: Some(5),
        page_token: None,
    };

    match client
        .corehr
        .basic_info
        .search_country_region(country_request, None)
        .await
    {
        Ok(response) => {
            println!("\n✅ 查询国家/地区信息成功");
            if let Some(data) = &response.data {
                if let Some(countries) = &data.country_regions.items {
                    println!("国家/地区列表:");
                    for country in countries.iter().take(3) {
                        if let Some(name) = &country.name {
                            if let Some(zh_cn) = &name.zh_cn {
                                print!("  - {zh_cn}");
                            }
                            if let Some(en_us) = &name.en_us {
                                print!(" ({en_us})");
                            }
                        }
                        if let Some(code) = &country.code {
                            print!(" [{code}]");
                        }
                        if let Some(time_zone) = &country.time_zone {
                            print!(" 时区: {time_zone}");
                        }
                        println!();
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 查询国家/地区信息失败: {err:?}");
        }
    }

    // 查询国籍信息
    let nationality_request = NationalitySearchRequest {
        page_size: Some(5),
        page_token: None,
    };

    match client
        .corehr
        .basic_info
        .search_nationality(nationality_request, None)
        .await
    {
        Ok(response) => {
            println!("\n✅ 查询国籍信息成功");
            if let Some(data) = &response.data {
                if let Some(nationalities) = &data.nationalities.items {
                    println!("国籍列表:");
                    for nationality in nationalities.iter().take(3) {
                        if let Some(name) = &nationality.name {
                            if let Some(zh_cn) = &name.zh_cn {
                                print!("  - {zh_cn}");
                            }
                            if let Some(en_us) = &name.en_us {
                                print!(" ({en_us})");
                            }
                        }
                        if let Some(code) = &nationality.code {
                            print!(" [{code}]");
                        }
                        println!();
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 查询国籍信息失败: {err:?}");
        }
    }

    // ID转换演示
    let id_convert_request = IdConvertRequest {
        source_id_type: "person_id".to_string(),
        target_id_type: "employee_id".to_string(),
        ids: vec!["example_person_id".to_string()],
    };

    match client
        .corehr
        .basic_info
        .convert_id(id_convert_request, None)
        .await
    {
        Ok(response) => {
            println!("\n✅ ID转换成功");
            if let Some(data) = &response.data {
                if let Some(items) = &data.items {
                    println!("转换结果:");
                    for item in items {
                        if let Some(source) = &item.source_id {
                            if let Some(target) = &item.target_id {
                                println!("  {source} -> {target}");
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ ID转换失败: {err:?}");
            println!("注意: 这可能是因为使用了示例ID");
        }
    }

    // 2. 员工信息管理演示
    println!("\n2. 员工信息管理演示");
    println!("----------------------------------");

    // 搜索员工信息
    let employee_search_request = EmployeeSearchRequest {
        page_size: Some(10),
        page_token: None,
        user_id_type: Some("open_id".to_string()),
        employee_id_type: Some("employee_id".to_string()),
        department_id_type: Some("open_department_id".to_string()),
        fields: Some(vec![
            "person".to_string(),
            "employment".to_string(),
            "job_datas".to_string(),
        ]),
        employment_status: Some(vec!["active".to_string()]),
        department_ids: None,
        employee_number: None,
    };

    match client
        .corehr
        .employee
        .search(employee_search_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 搜索员工信息成功");
            if let Some(data) = &response.data {
                if let Some(employees) = &data.employees.items {
                    println!("员工总数: {}", employees.len());
                    for (index, employee) in employees.iter().enumerate().take(3) {
                        println!("\n员工 #{}:", index + 1);
                        if let Some(employee_id) = &employee.employee_id {
                            println!("  员工ID: {employee_id}");
                        }
                        if let Some(employee_number) = &employee.employee_number {
                            println!("  工号: {employee_number}");
                        }
                        if let Some(status) = &employee.employment_status {
                            println!("  状态: {status}");
                        }

                        // 个人信息
                        if let Some(person) = &employee.person {
                            println!("  个人信息:");
                            if let Some(name) = &person.name {
                                if let Some(zh_cn) = &name.zh_cn {
                                    println!("    姓名: {zh_cn}");
                                }
                            }
                            if let Some(english_name) = &person.english_name {
                                println!("    英文名: {english_name}");
                            }
                            if let Some(gender) = &person.gender {
                                println!("    性别: {gender}");
                            }
                            if let Some(email) = &person.email {
                                println!("    邮箱: {email}");
                            }
                        }

                        // 雇佣信息
                        if let Some(employment) = &employee.employment {
                            println!("  雇佣信息:");
                            if let Some(hire_date) = &employment.hire_date {
                                println!("    入职日期: {hire_date}");
                            }
                            if let Some(employment_type) = &employment.employment_type {
                                println!("    雇佣类型: {employment_type}");
                            }
                            if let Some(work_email) = &employment.work_email {
                                println!("    工作邮箱: {work_email}");
                            }
                        }

                        // 任职信息
                        if let Some(job_datas) = &employee.job_datas {
                            if !job_datas.is_empty() {
                                println!("  任职信息:");
                                for job_data in job_datas.iter().take(2) {
                                    if let Some(department_id) = &job_data.department_id {
                                        println!("    部门ID: {department_id}");
                                    }
                                    if let Some(job_id) = &job_data.job_id {
                                        println!("    职务ID: {job_id}");
                                    }
                                    if let Some(effective_time) = &job_data.effective_time {
                                        println!("    生效时间: {effective_time}");
                                    }
                                }
                            }
                        }
                    }
                }

                // 分页信息
                if let Some(has_more) = data.employees.has_more {
                    println!("\n分页信息:");
                    println!("  是否有更多数据: {has_more}");
                    if let Some(page_token) = &data.employees.page_token {
                        println!("  下一页标记: {page_token}");
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 搜索员工信息失败: {err:?}");
        }
    }

    // 批量查询员工信息
    let batch_get_request = EmployeeBatchGetRequest {
        employee_ids: vec!["example_emp_1".to_string(), "example_emp_2".to_string()],
        user_id_type: Some("open_id".to_string()),
        employee_id_type: Some("employee_id".to_string()),
        department_id_type: Some("open_department_id".to_string()),
        fields: Some(vec![
            "person".to_string(),
            "employment".to_string(),
            "job_datas".to_string(),
        ]),
    };

    match client
        .corehr
        .employee
        .batch_get(batch_get_request, None)
        .await
    {
        Ok(response) => {
            println!("\n✅ 批量查询员工信息成功");
            if let Some(data) = &response.data {
                if let Some(employees) = &data.items {
                    println!("查询到 {} 个员工", employees.len());
                    for employee in employees {
                        if let Some(person) = &employee.person {
                            if let Some(name) = &person.name {
                                if let Some(zh_cn) = &name.zh_cn {
                                    println!("  - {zh_cn}");
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 批量查询员工信息失败: {err:?}");
            println!("注意: 这可能是因为使用了示例员工ID");
        }
    }

    // 3. 组织架构管理演示
    println!("\n3. 组织架构管理演示");
    println!("----------------------------------");

    // 创建部门
    let dept_request = DepartmentCreateRequest {
        name: I18nText {
            zh_cn: Some("技术研发部".to_string()),
            en_us: Some("Technology R&D Department".to_string()),
        },
        parent_department_id: Some("example_parent_dept".to_string()),
        manager: Some("example_manager_id".to_string()),
        code: Some("TECH_RD".to_string()),
        description: Some(I18nText {
            zh_cn: Some("负责产品技术研发和创新".to_string()),
            en_us: Some("Responsible for product technology R&D and innovation".to_string()),
        }),
        effective_time: Some("2024-01-01".to_string()),
        custom_fields: None,
    };

    match client
        .corehr
        .organization
        .create_department(dept_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 创建部门成功");
            if let Some(data) = &response.data {
                if let Some(department) = &data.department {
                    if let Some(dept_id) = &department.department_id {
                        println!("部门ID: {dept_id}");
                    }
                    if let Some(name) = &department.name {
                        if let Some(zh_cn) = &name.zh_cn {
                            println!("部门名称: {zh_cn}");
                        }
                    }
                    if let Some(code) = &department.code {
                        println!("部门编码: {code}");
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 创建部门失败: {err:?}");
            println!("注意: 这可能是因为使用了示例的上级部门ID或负责人ID");
        }
    }

    // 查询部门架构树
    match client
        .corehr
        .organization
        .get_department_tree(None, None)
        .await
    {
        Ok(response) => {
            println!("\n✅ 查询部门架构树成功");
            if let Some(data) = &response.data {
                if let Some(departments) = &data.items {
                    println!("部门架构树:");
                    for dept in departments.iter().take(3) {
                        if let Some(name) = &dept.name {
                            if let Some(zh_cn) = &name.zh_cn {
                                print!("  - {zh_cn}");
                            }
                        }
                        if let Some(code) = &dept.code {
                            print!(" [{code}]");
                        }
                        if let Some(active) = dept.active {
                            print!(" ({})", if active { "启用" } else { "停用" });
                        }
                        println!();

                        // 显示子部门
                        if let Some(children) = &dept.children {
                            for child in children.iter().take(2) {
                                if let Some(child_name) = &child.name {
                                    if let Some(zh_cn) = &child_name.zh_cn {
                                        println!("    └─ {zh_cn}");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 查询部门架构树失败: {err:?}");
        }
    }

    // 创建公司
    let company_request = CompanyCreateRequest {
        name: I18nText {
            zh_cn: Some("北京创新科技有限公司".to_string()),
            en_us: Some("Beijing Innovation Technology Co., Ltd.".to_string()),
        },
        company_type: Some("subsidiary".to_string()),
        legal_name: Some(I18nText {
            zh_cn: Some("北京创新科技有限公司".to_string()),
            en_us: Some("Beijing Innovation Technology Co., Ltd.".to_string()),
        }),
        code: Some("BJ_INNOVATION".to_string()),
        effective_time: Some("2024-01-01".to_string()),
        primary_location_id: Some("example_location_id".to_string()),
        custom_fields: None,
    };

    match client
        .corehr
        .organization
        .create_company(company_request, None)
        .await
    {
        Ok(response) => {
            println!("\n✅ 创建公司成功");
            if let Some(data) = &response.data {
                if let Some(company) = &data.company {
                    if let Some(company_id) = &company.company_id {
                        println!("公司ID: {company_id}");
                    }
                    if let Some(name) = &company.name {
                        if let Some(zh_cn) = &name.zh_cn {
                            println!("公司名称: {zh_cn}");
                        }
                    }
                    if let Some(code) = &company.code {
                        println!("公司编码: {code}");
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 创建公司失败: {err:?}");
            println!("注意: 这可能是因为使用了示例的地点ID");
        }
    }

    // 查询公司列表
    match client
        .corehr
        .organization
        .list_companies(Some(10), None, None)
        .await
    {
        Ok(response) => {
            println!("\n✅ 查询公司列表成功");
            if let Some(data) = &response.data {
                if let Some(companies) = &data.companies.items {
                    println!("公司列表:");
                    for company in companies.iter().take(3) {
                        if let Some(name) = &company.name {
                            if let Some(zh_cn) = &name.zh_cn {
                                print!("  - {zh_cn}");
                            }
                        }
                        if let Some(code) = &company.code {
                            print!(" [{code}]");
                        }
                        if let Some(company_type) = &company.company_type {
                            print!(" 类型: {company_type}");
                        }
                        if let Some(active) = company.active {
                            print!(" ({})", if active { "启用" } else { "停用" });
                        }
                        println!();
                    }
                }

                // 分页信息
                if let Some(has_more) = data.companies.has_more {
                    println!("是否有更多数据: {has_more}");
                }
            }
        }
        Err(err) => {
            error!("❌ 查询公司列表失败: {err:?}");
        }
    }

    // 7. 岗职务管理演示
    println!("\n7. 岗职务管理演示");
    println!("-------------------------------");

    // 创建序列
    let job_family_request = JobFamilyCreateRequest {
        name: I18nText {
            zh_cn: Some("技术序列".to_string()),
            en_us: Some("Technology".to_string()),
        },
        code: Some("TECH".to_string()),
        effective_time: Some("2024-01-01".to_string()),
        custom_fields: None,
    };

    match client
        .corehr
        .job_management
        .create_job_family(job_family_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 创建技术序列成功");
            if let Some(data) = &response.data {
                if let Some(job_family) = &data.job_family {
                    if let Some(job_family_id) = &job_family.job_family_id {
                        println!("序列ID: {job_family_id}");
                    }
                    if let Some(name) = &job_family.name {
                        if let Some(zh_cn) = &name.zh_cn {
                            println!("序列名称: {zh_cn}");
                        }
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 创建技术序列失败: {err:?}");
            println!("注意: 这可能是因为序列已存在或权限不足");
        }
    }

    // 创建职级
    let job_level_request = JobLevelCreateRequest {
        name: I18nText {
            zh_cn: Some("高级工程师".to_string()),
            en_us: Some("Senior Engineer".to_string()),
        },
        code: Some("P6".to_string()),
        description: Some(I18nText {
            zh_cn: Some("资深技术人员".to_string()),
            en_us: Some("Senior technical staff".to_string()),
        }),
        order: Some(6),
        effective_time: Some("2024-01-01".to_string()),
        custom_fields: None,
    };

    match client
        .corehr
        .job_management
        .create_job_level(job_level_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 创建职级成功");
            if let Some(data) = &response.data {
                if let Some(job_level) = &data.job_level {
                    if let Some(name) = &job_level.name {
                        if let Some(zh_cn) = &name.zh_cn {
                            println!("职级名称: {zh_cn}");
                        }
                    }
                    if let Some(order) = job_level.order {
                        println!("职级顺序: {order}");
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 创建职级失败: {err:?}");
        }
    }

    // 8. 员工生命周期管理演示
    println!("\n8. 员工生命周期管理演示");
    println!("-------------------------------");

    // 创建待入职
    let pre_hire_request = PreHireCreateRequest {
        person: Person {
            name: Some(I18nText {
                zh_cn: Some("李四".to_string()),
                en_us: Some("Li Si".to_string()),
            }),
            email: Some("lisi@example.com".to_string()),
            phone: Some("13900139000".to_string()),
            ..Default::default()
        },
        employment: Employment {
            hire_date: Some("2024-03-01".to_string()),
            employment_type: Some("full_time".to_string()),
            ..Default::default()
        },
        job_datas: Some(vec![JobData {
            job_id: Some("frontend_engineer".to_string()),
            job_level_id: Some("P5".to_string()),
            department_id: Some("tech_dept".to_string()),
            ..Default::default()
        }]),
        onboarding_flow_id: Some("flow_123".to_string()),
        expected_hire_date: Some("2024-03-01".to_string()),
        custom_fields: None,
    };

    match client
        .corehr
        .lifecycle
        .create_pre_hire(pre_hire_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 创建待入职成功");
            if let Some(data) = &response.data {
                if let Some(pre_hire) = &data.pre_hire {
                    if let Some(pre_hire_id) = &pre_hire.pre_hire_id {
                        println!("待入职ID: {pre_hire_id}");
                    }
                    if let Some(person) = &pre_hire.person {
                        if let Some(name) = &person.name {
                            if let Some(zh_cn) = &name.zh_cn {
                                println!("候选人姓名: {zh_cn}");
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 创建待入职失败: {err:?}");
            println!("注意: 这可能是因为使用了示例的流程ID或部门ID");
        }
    }

    // 发起异动（示例）
    let job_change_request = JobChangeCreateRequest {
        employee_id: "emp_example_123".to_string(),
        job_change_type_id: "promotion".to_string(),
        job_change_reason_id: Some("performance_excellent".to_string()),
        effective_date: "2024-04-01".to_string(),
        job_data: JobData {
            job_id: Some("senior_engineer".to_string()),
            job_level_id: Some("P7".to_string()),
            department_id: Some("tech_dept".to_string()),
            ..Default::default()
        },
        comments: Some("优秀员工晋升".to_string()),
        custom_fields: None,
    };

    match client
        .corehr
        .lifecycle
        .create_job_change(job_change_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 发起异动成功");
            if let Some(data) = &response.data {
                if let Some(job_change) = &data.job_change {
                    if let Some(job_change_id) = &job_change.job_change_id {
                        println!("异动ID: {job_change_id}");
                    }
                    if let Some(effective_date) = &job_change.effective_date {
                        println!("生效日期: {effective_date}");
                    }
                }
            }
        }
        Err(err) => {
            error!("❌ 发起异动失败: {err:?}");
            println!("注意: 这可能是因为使用了示例的员工ID");
        }
    }

    println!("\n🎉 飞书人事企业版API演示完成!");
    println!("\n💡 功能总结:");
    println!("✅ 基础数据查询 - 枚举、国家、国籍、ID转换");
    println!("✅ 员工信息管理 - 搜索、批量查询、个人档案");
    println!("✅ 组织架构管理 - 部门、公司创建和查询");
    println!("✅ 岗职务管理 - 序列、职级、职等、职务管理");
    println!("✅ 员工生命周期 - 入职、异动、离职流程管理");
    println!("\n📋 支持的核心功能:");
    println!("- 多语言支持（中英文）");
    println!("- 分页查询和批量操作");
    println!("- 自定义字段支持");
    println!("- 时间轴版本管理");
    println!("- 完整的员工档案管理");
    println!("- 灵活的组织架构设计");
    println!("- 全面的岗职务体系");
    println!("- 完整的员工生命周期管理");

    Ok(())
}
