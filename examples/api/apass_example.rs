use open_lark::{
    prelude::*,
    service::apass::models::{
        AuditLogListRequest, EnvironmentVariableQueryRequest, FlowExecuteRequest,
        FunctionInvokeRequest, RecordQueryRequest, RecordSearchRequest, SeatActivityListRequest,
        SeatAssignmentListRequest, UserTaskQueryRequest,
    },
};
use serde_json::json;
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID is required");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET is required");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 1. 席位管理示例
    println!("=== 席位管理示例 ===");
    seat_management_examples(&client, &app_id).await;

    // 2. 审计日志示例
    println!("\n=== 审计日志示例 ===");
    audit_log_examples(&client, &app_id).await;

    // 3. 权限管理示例
    println!("\n=== 权限管理示例 ===");
    permission_management_examples(&client, &app_id).await;

    // 4. 对象操作示例
    println!("\n=== 对象操作示例 ===");
    object_operation_examples(&client, &app_id).await;

    // 5. 函数执行示例
    println!("\n=== 函数执行示例 ===");
    function_execution_examples(&client, &app_id).await;

    // 6. 环境变量示例
    println!("\n=== 环境变量示例 ===");
    environment_variable_examples(&client, &app_id).await;

    // 7. 流程管理示例
    println!("\n=== 流程管理示例 ===");
    flow_management_examples(&client, &app_id).await;
}

/// 席位管理示例
async fn seat_management_examples(client: &LarkClient, _app_id: &str) {
    // 查询席位分配
    let request = SeatAssignmentListRequest {
        page_size: Some(10),
        page_token: None,
    };

    match client.apass.seat.list_seat_assignment(request, None).await {
        Ok(response) => {
            println!("席位分配列表:");
            if let Some(items) = &response.data.unwrap().page_response.items {
                for seat in items {
                    println!("  - 用户ID: {:?}, 状态: {:?}", seat.user_id, seat.status);
                }
            }
        }
        Err(e) => println!("查询席位分配失败: {e:?}"),
    }

    // 查询席位活跃
    let request = SeatActivityListRequest {
        page_size: Some(10),
        page_token: None,
        start_time: None,
        end_time: None,
    };

    match client.apass.seat.list_seat_activity(request, None).await {
        Ok(response) => {
            println!("席位活跃列表:");
            if let Some(items) = &response.data.unwrap().page_response.items {
                for activity in items {
                    println!(
                        "  - 用户ID: {:?}, 活跃时间: {:?}",
                        activity.user_id, activity.activity_time
                    );
                }
            }
        }
        Err(e) => println!("查询席位活跃失败: {e:?}"),
    }
}

/// 审计日志示例
async fn audit_log_examples(client: &LarkClient, app_id: &str) {
    // 查询审计日志
    let request = AuditLogListRequest {
        app_id: app_id.to_string(),
        start_time: None,
        end_time: None,
        operation_type: None,
        page_size: Some(10),
        page_token: None,
    };

    match client.apass.audit_log.list_audit_logs(request, None).await {
        Ok(response) => {
            println!("审计日志列表:");
            if let Some(items) = &response.data.unwrap().page_response.items {
                for log in items {
                    println!(
                        "  - 操作类型: {:?}, 操作时间: {:?}",
                        log.operation_type, log.operation_time
                    );
                }
            }
        }
        Err(e) => println!("查询审计日志失败: {e:?}"),
    }
}

/// 权限管理示例
async fn permission_management_examples(client: &LarkClient, app_id: &str) {
    // 获取角色成员信息
    match client
        .apass
        .permission
        .get_role_member(
            app_id.to_string(),
            "example_role".to_string(),
            "ou_example_user_id".to_string(),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("角色成员信息: {:?}", response.data.unwrap().role_member);
        }
        Err(e) => println!("查询角色成员失败: {e:?}"),
    }
}

/// 对象操作示例
async fn object_operation_examples(client: &LarkClient, app_id: &str) {
    // 搜索记录
    let request = RecordSearchRequest {
        app_id: app_id.to_string(),
        object_api_name: "example_object".to_string(),
        keyword: "test".to_string(),
        page_size: Some(10),
        page_token: None,
    };

    match client.apass.object.search_records(request, None).await {
        Ok(response) => {
            println!("记录搜索结果:");
            if let Some(items) = &response.data.unwrap().page_response.items {
                for record in items {
                    println!("  - 记录ID: {:?}", record.record_id);
                }
            }
        }
        Err(e) => println!("记录搜索失败: {e:?}"),
    }

    // 获取记录详情
    let request = RecordQueryRequest {
        app_id: app_id.to_string(),
        object_api_name: "example_object".to_string(),
        record_id: "example_record_id".to_string(),
        fields: None,
    };

    match client.apass.object.get_record(request, None).await {
        Ok(response) => {
            println!("记录详情: {:?}", response.data.unwrap().record);
        }
        Err(e) => println!("获取记录详情失败: {e:?}"),
    }
}

/// 函数执行示例
async fn function_execution_examples(client: &LarkClient, app_id: &str) {
    // 执行自定义函数
    let mut parameters = HashMap::new();
    parameters.insert("input_param".to_string(), json!("test_value"));

    let request = FunctionInvokeRequest {
        app_id: app_id.to_string(),
        function_api_name: "example_function".to_string(),
        parameters: Some(json!(parameters)),
    };

    match client.apass.function.invoke_function(request, None).await {
        Ok(response) => {
            println!("函数执行结果: {:?}", response.data.unwrap().invoke_result);
        }
        Err(e) => println!("函数执行失败: {e:?}"),
    }
}

/// 环境变量示例
async fn environment_variable_examples(client: &LarkClient, app_id: &str) {
    // 查询环境变量列表
    let request = EnvironmentVariableQueryRequest {
        app_id: app_id.to_string(),
        page_size: Some(10),
        page_token: None,
    };

    match client
        .apass
        .environment_variable
        .query_environment_variables(request, None)
        .await
    {
        Ok(response) => {
            println!("环境变量列表:");
            if let Some(items) = &response.data.unwrap().page_response.items {
                for env_var in items {
                    println!("  - 变量名: {:?}", env_var.variable_name);
                }
            }
        }
        Err(e) => println!("查询环境变量失败: {e:?}"),
    }
}

/// 流程管理示例
async fn flow_management_examples(client: &LarkClient, app_id: &str) {
    // 发起流程
    let mut parameters = HashMap::new();
    parameters.insert("flow_param".to_string(), json!("test_value"));

    let request = FlowExecuteRequest {
        app_id: app_id.to_string(),
        flow_api_name: "example_flow".to_string(),
        parameters: Some(json!(parameters)),
    };

    match client.apass.flow.execute_flow(request, None).await {
        Ok(response) => {
            println!("流程执行结果: {:?}", response.data.unwrap().execute_result);
        }
        Err(e) => println!("流程执行失败: {e:?}"),
    }

    // 查询人工任务
    let request = UserTaskQueryRequest {
        app_id: app_id.to_string(),
        status: Some("pending".to_string()),
        page_size: Some(10),
        page_token: None,
    };

    match client.apass.flow.query_user_tasks(request, None).await {
        Ok(response) => {
            println!("人工任务列表:");
            if let Some(items) = &response.data.unwrap().page_response.items {
                for task in items {
                    println!("  - 任务ID: {:?}, 状态: {:?}", task.task_id, task.status);
                }
            }
        }
        Err(e) => println!("查询人工任务失败: {e:?}"),
    }
}
