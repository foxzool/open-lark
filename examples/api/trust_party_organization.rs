use dotenvy::dotenv;
use open_lark::{prelude::*, service::trust_party::collaboration_organization::*};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 环境变量未设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 环境变量未设置");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    // 获取可见关联组织列表
    println!("=== 获取可见关联组织列表 ===");
    let list_request = OrganizationListRequest {
        page_size: Some(10),
        ..Default::default()
    };

    match client
        .trust_party
        .collaboration_organization
        .list_organizations(list_request, None)
        .await
    {
        Ok(response) => {
            if let Some(org_data) = response.data {
                println!("获取关联组织列表成功：");
                for org in &org_data.organizations.items {
                    println!("  - 组织ID: {}", org.org_id);
                    println!("    组织名称: {}", org.org_name);
                    if let Some(org_type) = &org.org_type {
                        println!("    组织类型: {org_type}");
                    }
                    if let Some(status) = &org.status {
                        println!("    状态: {status}");
                    }
                    if let Some(domain) = &org.domain {
                        println!("    域名: {domain}");
                    }
                    println!();
                }

                // 如果有组织，获取第一个组织的详情
                if let Some(first_org) = org_data.organizations.items.first() {
                    println!("=== 获取关联组织详情 ===");
                    match client
                        .trust_party
                        .collaboration_organization
                        .get_organization(&first_org.org_id, None)
                        .await
                    {
                        Ok(response) => {
                            if let Some(detail_data) = response.data {
                                let org = &detail_data.organization;
                                println!("获取组织详情成功：");
                                println!("  组织ID: {}", org.org_id);
                                println!("  组织名称: {}", org.org_name);
                                if let Some(description) = &org.description {
                                    println!("  描述: {description}");
                                }
                                if let Some(created_at) = org.created_at {
                                    println!("  创建时间: {created_at}");
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("获取组织详情失败: {e:?}");
                        }
                    }

                    // 获取组织架构信息
                    println!("\n=== 获取组织架构信息 ===");
                    match client
                        .trust_party
                        .collaboration_organization
                        .get_organization_structure(&first_org.org_id, None)
                        .await
                    {
                        Ok(response) => {
                            if let Some(structure_data) = response.data {
                                let structure = &structure_data.organization_structure;
                                println!("获取组织架构成功：");

                                if let Some(departments) = &structure.departments {
                                    println!("  部门数量: {}", departments.len());
                                    for (i, dept) in departments.iter().enumerate().take(3) {
                                        println!(
                                            "    {}. 部门: {} (ID: {})",
                                            i + 1,
                                            dept.name,
                                            dept.department_id
                                        );
                                    }
                                }

                                if let Some(users) = &structure.users {
                                    println!("  成员数量: {}", users.len());
                                    for (i, user) in users.iter().enumerate().take(3) {
                                        println!(
                                            "    {}. 成员: {} (ID: {})",
                                            i + 1,
                                            user.name,
                                            user.user_id
                                        );
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("获取组织架构失败: {e:?}");
                        }
                    }

                    // 获取共享成员范围
                    println!("\n=== 获取共享成员范围 ===");
                    let scope_request = SharedMemberScopeListRequest {
                        page_size: Some(5),
                        ..Default::default()
                    };

                    match client
                        .trust_party
                        .collaboration_organization
                        .list_shared_member_scope(&first_org.org_id, scope_request, None)
                        .await
                    {
                        Ok(response) => {
                            if let Some(scope_data) = response.data {
                                println!("获取共享成员范围成功：");
                                for (i, scope) in scope_data.shared_scopes.items.iter().enumerate()
                                {
                                    println!("  {}. 共享范围:", i + 1);
                                    if let Some(scope_type) = &scope.scope_type {
                                        println!("    类型: {scope_type}");
                                    }
                                    if let Some(dept_ids) = &scope.department_ids {
                                        println!("    部门数量: {}", dept_ids.len());
                                    }
                                    if let Some(user_ids) = &scope.user_ids {
                                        println!("    用户数量: {}", user_ids.len());
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("获取共享成员范围失败: {e:?}");
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("获取关联组织列表失败: {e:?}");
        }
    }

    // 管理员获取所有关联组织列表
    println!("\n=== 管理员获取所有关联组织列表 ===");
    let admin_request = AdminOrganizationListRequest {
        page_size: Some(10),
        ..Default::default()
    };

    match client
        .trust_party
        .collaboration_organization
        .admin_list_organizations(admin_request, None)
        .await
    {
        Ok(response) => {
            if let Some(admin_data) = response.data {
                println!("管理员获取组织列表成功：");
                for org in &admin_data.organizations.items {
                    println!("  - 组织ID: {}", org.org_id);
                    println!("    组织名称: {}", org.org_name);
                    if let Some(status) = &org.status {
                        println!("    状态: {status}");
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("管理员获取组织列表失败: {e:?}");
        }
    }

    Ok(())
}
