use open_lark::core::query_params::QueryParams;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("=== 查询参数性能测试 ===\n");

    const ITERATIONS: usize = 100_000;

    // 测试1: 旧方法 - 字符串分配
    println!("测试1: 旧方法 - 每次分配新字符串作为键");
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut params = HashMap::<String, String>::new();
        params.insert("page_size".to_string(), "20".to_string());
        params.insert("page_token".to_string(), "token_123".to_string());
        params.insert("user_id".to_string(), "user_456".to_string());
        params.insert("start_time".to_string(), "2024-01-01T00:00:00Z".to_string());
        params.insert("end_time".to_string(), "2024-01-31T23:59:59Z".to_string());
        std::hint::black_box(params);
    }
    let old_duration = start.elapsed();
    println!("旧方法用时: {:.2?}", old_duration);

    // 测试2: 新方法 - 静态常量
    println!("\n测试2: 新方法 - 使用静态常量作为键");
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut params = HashMap::<&'static str, String>::new();
        params.insert(QueryParams::PAGE_SIZE, "20".to_string());
        params.insert(QueryParams::PAGE_TOKEN, "token_123".to_string());
        params.insert(QueryParams::USER_ID, "user_456".to_string());
        params.insert(QueryParams::START_TIME, "2024-01-01T00:00:00Z".to_string());
        params.insert(QueryParams::END_TIME, "2024-01-31T23:59:59Z".to_string());
        std::hint::black_box(params);
    }
    let new_duration = start.elapsed();
    println!("新方法用时: {:.2?}", new_duration);

    // 测试3: 查询构建器方法
    println!("\n测试3: 查询构建器方法");
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let params = open_lark::core::query_params::QueryParamsBuilder::new()
            .page_size(20)
            .page_token("token_123")
            .user_id("user_456")
            .start_time("2024-01-01T00:00:00Z")
            .end_time("2024-01-31T23:59:59Z")
            .build();
        std::hint::black_box(params);
    }
    let builder_duration = start.elapsed();
    println!("构建器方法用时: {:.2?}", builder_duration);

    // 性能改进计算
    println!("\n=== 性能改进结果 ===");
    let improvement_new = (old_duration.as_nanos() as f64 - new_duration.as_nanos() as f64)
        / old_duration.as_nanos() as f64
        * 100.0;
    let improvement_builder = (old_duration.as_nanos() as f64 - builder_duration.as_nanos() as f64)
        / old_duration.as_nanos() as f64
        * 100.0;

    println!("静态常量方法改进: {:.1}%", improvement_new);
    println!("构建器方法改进: {:.1}%", improvement_builder);

    // 内存使用估算
    let old_memory_per_call = 5
        * ("page_size".len()
            + "page_token".len()
            + "user_id".len()
            + "start_time".len()
            + "end_time".len());
    let new_memory_per_call = 0; // 静态常量无额外分配

    println!("\n=== 内存使用估算 ===");
    println!("旧方法每次调用键分配: ~{} 字节", old_memory_per_call);
    println!("新方法每次调用键分配: ~{} 字节", new_memory_per_call);
    println!(
        "每次调用节省: {} 字节",
        old_memory_per_call - new_memory_per_call
    );

    let total_saved_bytes = (old_memory_per_call - new_memory_per_call) * ITERATIONS;
    println!(
        "{} 次迭代总共节省: {} 字节 ({:.1} KB)",
        ITERATIONS,
        total_saved_bytes,
        total_saved_bytes as f64 / 1024.0
    );
}
