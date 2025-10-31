//! 搜索用户服务 V1 演示
//!
//! 本示例展示了如何使用Search V1 User服务的各种功能：
//! - 基础用户搜索
//! - Builder模式API
//! - 分页验证搜索
//! - 迭代器模式处理大量数据

use open_lark::prelude::*;
use open_lark::service::search::v1::user::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    // 创建客户端
    let client = LarkClient::builder("your_app_id", "your_app_secret")
        .build();

    println!("🔍 搜索用户服务 V1 演示");
    println!("================================");

    // 示例1: 基础搜索功能
    println!("\n1️⃣ 基础搜索功能");
    println!("---------------");

    // 构建搜索请求
    let search_request = SearchUserRequest::builder()
        .query("张三")
        .page_size(10)
        .build();

    println!("搜索请求: {:#?}", search_request);

    // 实际API调用（需要有效的凭证）
    // let response = client.search.v1.user.search(&search_request, None).await?;
    // println!("搜索结果: {} 个用户", response.data?.users.len());

    // 示例2: Builder模式API
    println!("\n2️⃣ Builder模式API");
    println!("---------------");

    let builder_response = client.search.v1.user
        .search_user_builder()
        .query("技术部")
        .page_size(20)
        .page_token("example_token");

    println!("Builder请求已构建");

    // 实际API调用（需要有效的凭证）
    // let response = builder_response.execute(&client.search.v1.user).await?;
    // println!("Builder搜索结果: {} 个用户", response.data?.users.len());

    // 示例3: 安全分页搜索
    println!("\n3️⃣ 安全分页搜索");
    println!("---------------");

    // 测试有效的分页参数
    let result = SearchUserRequest::builder()
        .query("测试")
        .with_pagination(Some(20), None);

    match result {
        Ok(builder) => {
            println!("✅ 有效分页参数验证通过");
            let _request = builder.build();
        }
        Err(e) => {
            println!("❌ 分页参数验证失败: {}", e);
        }
    }

    // 测试无效的分页参数（超出限制）
    let result = SearchUserRequest::builder()
        .query("测试")
        .with_pagination(Some(300), None); // 超出200的限制

    match result {
        Ok(_) => {
            println!("❌ 应该拒绝无效的分页参数");
        }
        Err(e) => {
            println!("✅ 正确拒绝了无效分页参数: {}", e);
        }
    }

    // 示例4: 迭代器模式
    println!("\n4️⃣ 迭代器模式");
    println!("---------------");

    let iter_request = SearchUserRequest::builder()
        .query("研发")
        .page_size(50)
        .build();

    let mut iter = client.search.v1.user.search_user_iter(iter_request, None);
    println!("迭代器已创建，可以逐页获取数据");

    // 模拟迭代器使用（不实际发送请求）
    println!("模拟迭代器使用:");
    let mut page_count = 0;

    // 这里不实际调用 iter.next() 来避免API请求
    println!("  第{}页: 获取50个用户", page_count + 1);
    page_count += 1;
    println!("  第{}页: 获取50个用户", page_count + 1);
    page_count += 1;
    println!("  总共处理了{}页数据", page_count);

    // 示例5: 直接验证分页搜索
    println!("\n5️⃣ 直接验证分页搜索");
    println!("---------------");

    // 这个方法会自动验证分页参数
    println!("使用search_user_with_validated_pagination方法");

    // 实际API调用（需要有效的凭证）
    // let response = client.search.v1.user
    //     .search_user_with_validated_pagination(
    //         "李四",
    //         Some(15),
    //         None,
    //         None
    //     )
    //     .await?;
    //
    // println!("验证搜索结果: {} 个用户", response.data?.users.len());

    println!("\n✨ 演示完成！");
    println!("================================");
    println!("💡 提示: 要实际测试API调用，请配置有效的app_id和app_secret");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_user_request_builder() {
        let request = SearchUserRequest::builder()
            .query("测试用户")
            .page_size(20)
            .page_token("test_token")
            .build();

        assert_eq!(request.query, Some("测试用户".to_string()));
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("test_token".to_string()));
    }

    #[test]
    fn test_pagination_validation() {
        // 测试有效分页
        let result = SearchUserRequest::builder()
            .query("测试")
            .with_pagination(Some(20), None);
        assert!(result.is_ok());

        // 测试超出限制的分页
        let result = SearchUserRequest::builder()
            .query("测试")
            .with_pagination(Some(300), None);
        assert!(result.is_err());

        // 测试页面大小为0
        let result = SearchUserRequest::builder()
            .query("测试")
            .with_pagination(Some(0), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_search_user_request_to_query_params() {
        let request = SearchUserRequest {
            query: Some("张三".to_string()),
            page_size: Some(20),
            page_token: Some("token123".to_string()),
        };

        let params = request.to_query_params();

        assert_eq!(params.get("query"), Some(&"张三".to_string()));
        assert_eq!(params.get("page_size"), Some(&"20".to_string()));
        assert_eq!(params.get("page_token"), Some(&"token123".to_string()));
    }
}