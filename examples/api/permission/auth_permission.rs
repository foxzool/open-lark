use dotenv::dotenv;
use open_lark::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 示例1：检查当前用户是否有编辑权限
    println!("🔹 示例1: 检查当前用户的编辑权限");
    let request = AuthPermissionRequest::builder()
        .token("doccnxxxxxxxxxxxxxx") // 替换为实际的文档token
        .as_doc()
        .check_edit()
        .build();

    match client.permission.auth_permission(&request, None).await {
        Ok(response) => {
            println!("✅ 权限检查完成!");
            
            if let Some(data) = response.data {
                println!("{}", data.summary());
                
                let auth_result = &data.auth_result;
                
                if auth_result.has_permission() {
                    println!("  ✅ 当前用户有编辑权限");
                    
                    // 检查是否有更高级别的权限
                    if auth_result.has_higher_permission() {
                        println!("  🚀 用户拥有更高级别的权限");
                        if let Some(actual) = auth_result.actual_permission() {
                            println!("     实际权限: {}", match actual {
                                "full_access" => "所有者",
                                _ => actual,
                            });
                        }
                    }
                    
                    // 检查可执行的操作
                    println!("  📋 可执行操作:");
                    if data.can_perform_action("read") {
                        println!("     👁️  查看文档");
                    }
                    if data.can_perform_action("comment") {
                        println!("     💬 添加评论");
                    }
                    if data.can_perform_action("edit") {
                        println!("     ✏️  编辑内容");
                    }
                    if data.can_perform_action("manage") {
                        println!("     🛡️  管理权限");
                    }
                } else {
                    println!("  ❌ 当前用户没有编辑权限");
                    if let Some(actual) = auth_result.actual_permission() {
                        println!("     实际权限: {}", match actual {
                            "view" => "仅查看",
                            "comment" => "可评论",
                            _ => actual,
                        });
                    } else {
                        println!("     无任何权限");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 权限检查失败: {:?}", e);
        }
    }

    println!("\n" + &"=".repeat(50));

    // 示例2：检查所有者权限
    println!("🔹 示例2: 检查当前用户的所有者权限");
    let request = AuthPermissionRequest::new(
        "doccnxxxxxxxxxxxxxx", // 替换为实际的文档token
        "doc",
        Permission::FullAccess,
    );

    match client.permission.auth_permission(&request, None).await {
        Ok(response) => {
            println!("✅ 所有者权限检查完成!");
            
            if let Some(data) = response.data {
                println!("{}", data.summary());
                
                if data.has_permission() {
                    println!("  👑 当前用户是文档所有者");
                    println!("  📋 拥有完全管理权限:");
                    println!("     🛡️  管理协作者权限");
                    println!("     ⚙️  修改文档设置"); 
                    println!("     🗑️  删除文档");
                    println!("     📤 转移所有权");
                } else {
                    println!("  ❌ 当前用户不是文档所有者");
                    println!("  💡 如需管理权限，请联系文档所有者");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 所有者权限检查失败: {:?}", e);
        }
    }

    println!("\n" + &"=".repeat(50));

    // 示例3：检查电子表格的查看权限
    println!("🔹 示例3: 检查电子表格的查看权限");
    let request = AuthPermissionRequest::builder()
        .token("shtcnxxxxxxxxxxxxxx") // 替换为实际的电子表格token
        .as_sheet()
        .check_view()
        .build();

    match client.permission.auth_permission(&request, None).await {
        Ok(response) => {
            println!("✅ 查看权限检查完成!");
            
            if let Some(data) = response.data {
                println!("{}", data.summary());
                
                if data.has_permission() {
                    println!("  ✅ 当前用户可以查看电子表格");
                    
                    // 权限级别分析
                    let auth_result = &data.auth_result;
                    let level = auth_result.actual_permission_level();
                    
                    println!("  📊 权限级别: {}/4", level);
                    match level {
                        1 => println!("     👁️  基础查看权限"),
                        2 => println!("     💬 可查看和评论"),
                        3 => println!("     ✏️  可编辑数据"),
                        4 => println!("     🛡️  完全管理权限"),
                        _ => println!("     ❓ 无权限"),
                    }
                    
                    // 建议的操作
                    println!("  💡 建议操作:");
                    if data.can_perform_action("edit") {
                        println!("     - 可以编辑表格数据");
                        println!("     - 可以添加/删除行列");
                    } else if data.can_perform_action("comment") {
                        println!("     - 可以添加评论和批注");
                        println!("     - 不能修改数据内容");
                    } else {
                        println!("     - 仅可查看表格内容");
                        println!("     - 如需编辑请申请更高权限");
                    }
                } else {
                    println!("  ❌ 当前用户没有查看权限");
                    println!("  💡 请联系文档所有者获取访问权限");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 查看权限检查失败: {:?}", e);
        }
    }

    println!("\n🔐 权限系统说明:");
    println!("   权限级别 (从低到高):");
    println!("   1. 查看 (view) - 仅可查看内容");
    println!("   2. 评论 (comment) - 可查看和评论");
    println!("   3. 编辑 (edit) - 可编辑内容");
    println!("   4. 所有者 (full_access) - 完全管理权限");

    Ok(())
}