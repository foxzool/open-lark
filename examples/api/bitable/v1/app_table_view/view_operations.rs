use dotenvy::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 替换为实际的多维表格和数据表 token
    let app_token = "bascnmBA*****yGehy8"; // 请替换为实际的 app_token
    let table_id = "tblsRc9GRRXKqhvW"; // 请替换为实际的 table_id

    // 创建视图示例
    println!("--- 1. 创建不同类型的视图 ---");

    use open_lark::service::bitable::v1::app_table_view::ViewData;

    // 创建表格视图
    let grid_view = ViewData::grid_view("项目列表视图");
    let create_grid_req = open_lark::service::bitable::v1::app_table_view::CreateViewRequest::new(
        app_token, table_id, grid_view,
    );

    let grid_view_id = match client
        .bitable
        .v1
        .app_table_view
        .create(create_grid_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 创建表格视图成功!");
                println!("🆔 View ID: {}", data.view_id);
                Some(data.view_id)
            } else {
                eprintln!("❌ 响应数据为空");
                None
            }
        }
        Err(e) => {
            eprintln!("❌ 创建表格视图失败: {:?}", e);
            None
        }
    };

    // 创建看板视图
    let kanban_view = ViewData::kanban_view("项目看板视图").with_property(serde_json::json!({
        "kanban_info": {
            "field_id": "fldxxxxxx", // 实际需要替换为有效的字段ID
            "group_info": []
        }
    }));

    let create_kanban_req =
        open_lark::service::bitable::v1::app_table_view::CreateViewRequest::builder()
            .app_token(app_token)
            .table_id(table_id)
            .view(kanban_view)
            .build();

    let kanban_view_id = match client
        .bitable
        .v1
        .app_table_view
        .create(create_kanban_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 创建看板视图成功!");
                println!("🆔 View ID: {}", data.view_id);
                Some(data.view_id)
            } else {
                eprintln!("❌ 响应数据为空");
                None
            }
        }
        Err(e) => {
            eprintln!("❌ 创建看板视图失败: {:?}", e);
            None
        }
    };

    // 创建画册视图
    let gallery_view = ViewData::gallery_view("项目画册视图");
    let create_gallery_req =
        open_lark::service::bitable::v1::app_table_view::CreateViewRequest::new(
            app_token,
            table_id,
            gallery_view,
        );

    let gallery_view_id = match client
        .bitable
        .v1
        .app_table_view
        .create(create_gallery_req, None)
        .await
    {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 创建画册视图成功!");
                println!("🆔 View ID: {}", data.view_id);
                Some(data.view_id)
            } else {
                eprintln!("❌ 响应数据为空");
                None
            }
        }
        Err(e) => {
            eprintln!("❌ 创建画册视图失败: {:?}", e);
            None
        }
    };

    // 列出所有视图
    println!("\n--- 2. 列出所有视图 ---");
    let list_req = open_lark::service::bitable::v1::app_table_view::ListViewsRequest::builder()
        .app_token(app_token)
        .table_id(table_id)
        .page_size(50)
        .build();

    match client.bitable.v1.app_table_view.list(list_req, None).await {
        Ok(resp) => {
            if let Some(data) = resp.data {
                println!("✅ 获取视图列表成功!");
                println!("📊 总数量: {}", data.total);
                println!("🔄 是否还有更多: {}", data.has_more);
                println!("📋 视图列表:");

                for (i, view) in data.items.iter().enumerate() {
                    println!(
                        "  {}. {} (ID: {}, 类型: {})",
                        i + 1,
                        view.view_name,
                        view.view_id,
                        view.view_type
                    );
                    if let Some(property) = &view.property {
                        println!("     属性: {}", property);
                    }
                }
            } else {
                println!("❌ 响应数据为空");
            }
        }
        Err(e) => {
            eprintln!("❌ 获取视图列表失败: {:?}", e);
        }
    }

    // 获取特定视图详情
    if let Some(view_id) = &grid_view_id {
        println!("\n--- 3. 获取视图详情 ---");
        let get_req = open_lark::service::bitable::v1::app_table_view::GetViewRequest::new(
            app_token, table_id, view_id,
        );

        match client.bitable.v1.app_table_view.get(get_req, None).await {
            Ok(resp) => {
                if let Some(data) = resp.data {
                    println!("✅ 获取视图详情成功!");
                    println!("🆔 View ID: {}", data.view.view_id);
                    println!("📋 视图名称: {}", data.view.view_name);
                    println!("🔧 视图类型: {}", data.view.view_type);
                    if let Some(property) = &data.view.property {
                        println!("⚙️  视图属性: {}", serde_json::to_string_pretty(property)?);
                    }
                } else {
                    println!("❌ 响应数据为空");
                }
            }
            Err(e) => {
                eprintln!("❌ 获取视图详情失败: {:?}", e);
            }
        }
    }

    // 更新视图
    if let Some(view_id) = &grid_view_id {
        println!("\n--- 4. 更新视图 ---");
        let patch_req =
            open_lark::service::bitable::v1::app_table_view::PatchViewRequest::builder()
                .app_token(app_token)
                .table_id(table_id)
                .view_id(view_id)
                .view_name("更新后的项目列表视图")
                .property(serde_json::json!({
                    "filter_info": {
                        "conjunction": "and",
                        "conditions": [
                            {
                                "field_id": "fldxxxxxx", // 实际需要替换为有效的字段ID
                                "operator": "is",
                                "value": ["进行中"]
                            }
                        ]
                    },
                    "sort_info": [
                        {
                            "field_id": "fldxxxxxx", // 实际需要替换为有效的字段ID
                            "desc": false
                        }
                    ]
                }))
                .build();

        match client
            .bitable
            .v1
            .app_table_view
            .patch(patch_req, None)
            .await
        {
            Ok(resp) => {
                if let Some(data) = resp.data {
                    println!("✅ 更新视图成功!");
                    println!("🆔 View ID: {}", data.view_id);
                    println!("📋 新名称: {}", data.view_name);
                    println!("🔧 视图类型: {}", data.view_type);
                } else {
                    println!("❌ 响应数据为空");
                }
            }
            Err(e) => {
                eprintln!("❌ 更新视图失败: {:?}", e);
            }
        }
    }

    // 删除视图 (演示用)
    if let Some(view_id) = &gallery_view_id {
        println!("\n--- 5. 删除视图 (演示用) ---");
        println!("⚠️  注意：这将永久删除视图!");

        // 取消注释以下代码来执行删除操作
        // let delete_req = open_lark::service::bitable::v1::app_table_view::DeleteViewRequest::new(
        // app_token,
        // table_id,
        // view_id
        // );
        //
        // match client.bitable.v1.app_table_view.delete(delete_req, None).await {
        // Ok(resp) => {
        // if let Some(data) = resp.data {
        // println!("✅ 删除视图成功: {}", data.deleted);
        // } else {
        // println!("❌ 响应数据为空");
        // }
        // }
        // Err(e) => {
        // eprintln!("❌ 删除视图失败: {:?}", e);
        // }
        // }
    }

    println!("\n💡 视图功能说明:");
    println!("- 视图是对数据表数据的不同展示方式");
    println!("- 同一个数据表可以有多个视图，每个视图可以有不同的筛选、排序设置");
    println!("- 视图不会影响数据本身，只是改变数据的展示方式");

    println!("\n🔧 视图类型说明:");
    println!("- grid (表格视图): 传统的表格形式，类似Excel");
    println!("- kanban (看板视图): 卡片式看板，适合项目管理");
    println!("- gallery (画册视图): 网格式卡片展示，适合图片内容");
    println!("- gantt (甘特视图): 时间线视图，适合项目进度管理");

    println!("\n⚙️  视图属性说明:");
    println!("- filter_info: 筛选条件设置");
    println!("- sort_info: 排序规则设置");
    println!("- group_info: 分组显示设置");
    println!("- field_order: 字段显示顺序");

    println!("\n🎯 视图使用场景:");
    println!("- 项目管理: 使用看板视图跟踪任务进度");
    println!("- 数据分析: 使用表格视图查看详细数据");
    println!("- 内容展示: 使用画册视图展示图片资源");
    println!("- 时间规划: 使用甘特视图管理项目时间线");

    Ok(())
}
