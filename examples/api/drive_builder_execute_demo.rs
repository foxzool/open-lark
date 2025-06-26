use dotenvy::dotenv;
use open_lark::client::LarkClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID not found in environment");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET not found in environment");

    // 初始化客户端
    let _client = LarkClient::builder(&app_id, &app_secret).build();

    // 演示使用Builder模式的execute方法
    println!("🚀 演示Drive服务Builder的execute方法");

    // 1. 演示媒体文件上传请求构建
    println!("\n📁 构建媒体文件上传请求...");
    let _upload_request =
        open_lark::service::cloud_docs::drive::v1::media::UploadMediaRequest::builder()
            .file_name("test.jpg")
            .parent_token("your_parent_token")
            .size(1024)
            .file(vec![1, 2, 3, 4]) // 模拟文件数据
            .build();

    println!("✅ 媒体文件上传请求构建完成");

    // 注意: 下面的execute方法需要有效的token和权限才能执行
    // 在实际使用中，你可以这样调用:
    // let result = UploadMediaRequest::builder()
    //     .file_name("test.jpg")
    //     .parent_token("valid_parent_token")
    //     .size(1024)
    //     .file(file_data)
    //     .execute(&client.drive.v1.media)
    //     .await;

    // 2. 演示权限设置请求构建
    println!("\n🔐 构建权限设置请求...");
    let _permission_request =
        open_lark::service::cloud_docs::drive::v1::permissions::GetPermissionRequest::builder()
            .token("your_file_token")
            .r#type("doc")
            .build();

    println!("✅ 权限设置请求构建完成");

    // 3. 演示文件夹创建请求构建
    println!("\n📂 构建文件夹创建请求...");
    let _folder_request =
        open_lark::service::cloud_docs::drive::v2::explorer::CreateFolderRequest::builder()
            .name("新建文件夹")
            .folder_token("parent_folder_token")
            .build();

    println!("✅ 文件夹创建请求构建完成");

    // 4. 演示文件夹列表请求构建
    println!("\n📋 构建文件夹列表请求...");
    let _list_request =
        open_lark::service::cloud_docs::drive::v2::explorer::ListFolderRequest::builder()
            .folder_token("folder_token")
            .page_size(50)
            .order_by("EditedTime")
            .direction("DESC")
            .build();

    println!("✅ 文件夹列表请求构建完成");

    println!("\n🎉 所有Builder的execute方法都已成功添加！");
    println!("💡 提示: 在实际使用中，可以直接调用 .execute(&service) 来执行请求");
    println!("💡 提示: 也可以使用 .execute_with_options(&service, options) 来带额外选项执行");

    Ok(())
}
