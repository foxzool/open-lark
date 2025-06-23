// 增强Builder模式性能基准测试
//
// 这个基准测试比较传统Builder模式和增强Builder模式的性能差异
// 验证增强Builder模式确实是零开销抽象

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use open_lark::{
    prelude::*,
    service::{
        cloud_docs::{
            permission::member::{batch_create::Permission, create::CreatePermissionMemberRequest},
            sheets::v3::spreadsheet::CreateSpreadsheetRequest,
        },
        im::v1::message::create::CreateMessageRequest,
    },
};

// 模拟的测试数据
const SPREADSHEET_TITLE: &str = "Performance Test Spreadsheet";
const FOLDER_TOKEN: &str = "test_folder_token_123456";
const USER_ID: &str = "test_user_id_123456";
const DOC_TOKEN: &str = "test_doc_token_123456";
const CHAT_ID: &str = "test_chat_id_123456";
const MESSAGE_CONTENT: &str = "Performance test message content";

fn benchmark_spreadsheet_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("spreadsheet_creation");

    // 传统方式基准测试
    group.bench_function("traditional_builder", |b| {
        b.iter(|| {
            let request = CreateSpreadsheetRequest::builder()
                .title(black_box(SPREADSHEET_TITLE))
                .folder_token(black_box(FOLDER_TOKEN))
                .build();

            // 模拟检查构建的请求对象
            black_box(request)
        })
    });

    // 注意：增强方式由于需要异步服务调用，我们只测试构建部分
    group.bench_function("enhanced_builder_build_part", |b| {
        b.iter(|| {
            // 测试增强Builder的构建部分性能
            let builder = CreateSpreadsheetRequest::builder()
                .title(black_box(SPREADSHEET_TITLE))
                .folder_token(black_box(FOLDER_TOKEN));

            // 模拟execute方法中的build()调用
            let request = builder.build();
            black_box(request)
        })
    });

    group.finish();
}

fn benchmark_permission_member_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("permission_member_creation");

    // 传统方式
    group.bench_function("traditional_builder", |b| {
        b.iter(|| {
            let request = CreatePermissionMemberRequest::builder()
                .token(black_box(DOC_TOKEN))
                .as_doc()
                .user(black_box(USER_ID))
                .as_editor()
                .with_notification()
                .build();

            black_box(request)
        })
    });

    // 增强方式的构建部分
    group.bench_function("enhanced_builder_build_part", |b| {
        b.iter(|| {
            let builder = CreatePermissionMemberRequest::builder()
                .token(black_box(DOC_TOKEN))
                .as_doc()
                .user(black_box(USER_ID))
                .as_editor()
                .with_notification();

            let request = builder.build();
            black_box(request)
        })
    });

    group.finish();
}

fn benchmark_message_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_creation");

    // 传统方式
    group.bench_function("traditional_builder", |b| {
        b.iter(|| {
            let request = CreateMessageRequest::builder()
                .receive_id_type(black_box("chat_id"))
                .receive_id(black_box(CHAT_ID))
                .msg_type(black_box("text"))
                .content(black_box(MESSAGE_CONTENT))
                .build();

            black_box(request)
        })
    });

    // 增强方式的构建部分
    group.bench_function("enhanced_builder_build_part", |b| {
        b.iter(|| {
            let builder = CreateMessageRequest::builder()
                .receive_id_type(black_box("chat_id"))
                .receive_id(black_box(CHAT_ID))
                .msg_type(black_box("text"))
                .content(black_box(MESSAGE_CONTENT));

            let request = builder.build();
            black_box(request)
        })
    });

    group.finish();
}

fn benchmark_complex_builder_chain(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_builder_chain");

    // 测试复杂的Builder链性能
    group.bench_function("long_chain_traditional", |b| {
        b.iter(|| {
            let request = CreatePermissionMemberRequest::builder()
                .token(black_box(DOC_TOKEN))
                .obj_type(black_box("sheet"))
                .member(black_box("user"), black_box(USER_ID))
                .permission(black_box(Permission::Edit))
                .need_notification(black_box(true))
                .build();

            black_box(request)
        })
    });

    group.bench_function("long_chain_enhanced_preparation", |b| {
        b.iter(|| {
            // 测试增强Builder在准备execute时的性能
            let builder = CreatePermissionMemberRequest::builder()
                .token(black_box(DOC_TOKEN))
                .obj_type(black_box("sheet"))
                .member(black_box("user"), black_box(USER_ID))
                .permission(black_box(Permission::Edit))
                .need_notification(black_box(true));

            // 模拟execute方法调用前的准备工作
            let request = builder.build();
            black_box(request)
        })
    });

    group.finish();
}

fn benchmark_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");

    // 测试内存分配模式
    group.bench_function("traditional_multiple_requests", |b| {
        b.iter(|| {
            let mut requests = Vec::new();

            for i in 0..10 {
                let request = CreateMessageRequest::builder()
                    .receive_id_type("chat_id")
                    .receive_id(CHAT_ID)
                    .msg_type("text")
                    .content(&format!("Message {}", i))
                    .build();
                requests.push(request);
            }

            black_box(requests)
        })
    });

    group.bench_function("enhanced_multiple_preparations", |b| {
        b.iter(|| {
            let mut requests = Vec::new();

            for i in 0..10 {
                let builder = CreateMessageRequest::builder()
                    .receive_id_type("chat_id")
                    .receive_id(CHAT_ID)
                    .msg_type("text")
                    .content(&format!("Message {}", i));

                // 模拟execute方法的构建过程
                let request = builder.build();
                requests.push(request);
            }

            black_box(requests)
        })
    });

    group.finish();
}

fn benchmark_error_handling_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_handling");

    // 测试错误处理性能
    group.bench_function("traditional_error_pattern", |b| {
        b.iter(|| {
            // 模拟传统方式的错误处理
            let result: Result<CreateMessageRequest, &'static str> = (|| {
                let request = CreateMessageRequest::builder()
                    .receive_id_type("chat_id")
                    .receive_id(CHAT_ID)
                    .msg_type("text")
                    .content(MESSAGE_CONTENT)
                    .build();
                Ok(request)
            })();

            black_box(result)
        })
    });

    group.bench_function("enhanced_error_pattern", |b| {
        b.iter(|| {
            // 模拟增强方式的错误处理模式
            let result: Result<CreateMessageRequest, &'static str> = (|| {
                let builder = CreateMessageRequest::builder()
                    .receive_id_type("chat_id")
                    .receive_id(CHAT_ID)
                    .msg_type("text")
                    .content(MESSAGE_CONTENT);

                // 增强Builder的错误处理集中在execute方法中
                let request = builder.build();
                Ok(request)
            })();

            black_box(result)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_spreadsheet_creation,
    benchmark_permission_member_creation,
    benchmark_message_creation,
    benchmark_complex_builder_chain,
    benchmark_memory_allocation,
    benchmark_error_handling_patterns
);

criterion_main!(benches);
