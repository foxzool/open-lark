use open_lark::{
    core::config::{AppType, Config},
    service::wiki::v2::space_setting::UpdateSpaceSettingRequest,
    LarkClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 配置必要参数
    let space_id = "spcxxxxxx"; // 需要一个真实的知识空间ID

    // 1. 开启评论和复制功能
    println!("1. 开启知识空间的评论和复制功能...");

    let enable_request = UpdateSpaceSettingRequest::builder()
        .space_id(space_id)
        .enable_comment()
        .enable_copy()
        .build();

    match client
        .wiki
        .v2
        .space_setting
        .update(enable_request, None)
        .await
    {
        Ok(enable_response) => {
            let setting = &enable_response.data.setting;
            println!("设置更新成功:");
            println!("  - 空间ID: {}", setting.space_id);
            if let Some(comment_enabled) = setting.comment_enabled {
                println!(
                    "  - 评论功能: {}",
                    if comment_enabled {
                        "已开启"
                    } else {
                        "已关闭"
                    }
                );
            }
            if let Some(copy_enabled) = setting.copy_enabled {
                println!(
                    "  - 复制功能: {}",
                    if copy_enabled {
                        "已开启"
                    } else {
                        "已关闭"
                    }
                );
            }
        }
        Err(e) => {
            println!("开启功能失败: {:?}", e);
            println!("可能的原因：");
            println!("- 空间ID不存在或无效");
            println!("- 没有管理权限");
            println!("- 空间设置不允许修改");
        }
    }

    // 2. 关闭评论功能，保持复制功能开启
    println!("\n2. 关闭评论功能，保持复制功能开启...");

    let partial_disable_request = UpdateSpaceSettingRequest::builder()
        .space_id(space_id)
        .disable_comment()
        .build(); // 只更新评论设置，不修改复制设置

    match client
        .wiki
        .v2
        .space_setting
        .update(partial_disable_request, None)
        .await
    {
        Ok(partial_response) => {
            let setting = &partial_response.data.setting;
            println!("部分设置更新成功:");
            println!("  - 空间ID: {}", setting.space_id);
            if let Some(comment_enabled) = setting.comment_enabled {
                println!(
                    "  - 评论功能: {}",
                    if comment_enabled {
                        "已开启"
                    } else {
                        "已关闭"
                    }
                );
            }
            if let Some(copy_enabled) = setting.copy_enabled {
                println!(
                    "  - 复制功能: {}",
                    if copy_enabled {
                        "已开启"
                    } else {
                        "已关闭"
                    }
                );
            }
        }
        Err(e) => {
            println!("部分设置更新失败: {:?}", e);
        }
    }

    // 3. 关闭所有功能
    println!("\n3. 关闭所有功能...");

    let disable_all_request = UpdateSpaceSettingRequest::builder()
        .space_id(space_id)
        .disable_comment()
        .disable_copy()
        .build();

    match client
        .wiki
        .v2
        .space_setting
        .update(disable_all_request, None)
        .await
    {
        Ok(disable_response) => {
            let setting = &disable_response.data.setting;
            println!("全部功能关闭成功:");
            println!("  - 空间ID: {}", setting.space_id);
            if let Some(comment_enabled) = setting.comment_enabled {
                println!(
                    "  - 评论功能: {}",
                    if comment_enabled {
                        "已开启"
                    } else {
                        "已关闭"
                    }
                );
            }
            if let Some(copy_enabled) = setting.copy_enabled {
                println!(
                    "  - 复制功能: {}",
                    if copy_enabled {
                        "已开启"
                    } else {
                        "已关闭"
                    }
                );
            }
        }
        Err(e) => {
            println!("关闭所有功能失败: {:?}", e);
        }
    }

    // 4. 恢复所有功能到开启状态
    println!("\n4. 恢复所有功能到开启状态...");

    let restore_request = UpdateSpaceSettingRequest::builder()
        .space_id(space_id)
        .comment_enabled(true)
        .copy_enabled(true)
        .build();

    match client
        .wiki
        .v2
        .space_setting
        .update(restore_request, None)
        .await
    {
        Ok(restore_response) => {
            let setting = &restore_response.data.setting;
            println!("功能恢复成功:");
            println!("  - 空间ID: {}", setting.space_id);
            if let Some(comment_enabled) = setting.comment_enabled {
                println!(
                    "  - 评论功能: {}",
                    if comment_enabled {
                        "已开启"
                    } else {
                        "已关闭"
                    }
                );
            }
            if let Some(copy_enabled) = setting.copy_enabled {
                println!(
                    "  - 复制功能: {}",
                    if copy_enabled {
                        "已开启"
                    } else {
                        "已关闭"
                    }
                );
            }
        }
        Err(e) => {
            println!("功能恢复失败: {:?}", e);
        }
    }

    println!("\n知识空间设置操作示例完成！");
    println!("提示：");
    println!("- 需要知识空间的管理权限才能修改设置");
    println!("- 评论功能控制是否允许用户在文档中添加评论");
    println!("- 复制功能控制是否允许用户复制空间内容");
    println!("- 可以单独修改某个设置项，不影响其他设置");
    println!("- 设置修改会实时生效，影响所有空间成员");

    Ok(())
}
