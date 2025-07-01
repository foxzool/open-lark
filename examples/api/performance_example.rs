use dotenvy::dotenv;
use open_lark::{prelude::*, service::performance::*};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    // 获取必要的环境变量
    let app_id = env::var("APP_ID").expect("APP_ID 必须设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 必须设置");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("=== 飞书绩效管理 API 示例 ===\n");

    // 1. 后台配置管理示例
    println!("1. 后台配置管理");

    // 获取周期列表
    println!("  1.1 获取周期列表");
    let semester_request = review_config::SemesterListRequest {
        page_size: Some(10),
        ..Default::default()
    };

    match client
        .performance
        .review_config
        .list_semesters(semester_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!(
                    "    ✅ 获取周期列表成功，数量: {}",
                    data.semesters.items.len()
                );
                if !data.semesters.items.is_empty() {
                    let first_semester = &data.semesters.items[0];
                    println!(
                        "    📊 第一个周期: {} (ID: {})",
                        first_semester.name, first_semester.semester_id
                    );

                    // 使用第一个周期获取项目列表
                    println!("  1.2 获取项目列表");
                    let activity_request = review_config::ActivityQueryRequest {
                        semester_id: Some(first_semester.semester_id.clone()),
                        page_size: Some(10),
                        ..Default::default()
                    };

                    match client
                        .performance
                        .review_config
                        .query_activities(activity_request, None)
                        .await
                    {
                        Ok(response) => {
                            if let Some(data) = response.data {
                                println!(
                                    "    ✅ 获取项目列表成功，数量: {}",
                                    data.activities.items.len()
                                );
                                if !data.activities.items.is_empty() {
                                    let first_activity = &data.activities.items[0];
                                    println!(
                                        "    📈 第一个项目: {} (ID: {})",
                                        first_activity.name, first_activity.activity_id
                                    );

                                    // 获取被评估人信息
                                    println!("  1.3 获取被评估人信息");
                                    let reviewee_request = review_config::RevieweeQueryRequest {
                                        activity_id: first_activity.activity_id.clone(),
                                        user_ids: None,
                                    };

                                    match client
                                        .performance
                                        .review_config
                                        .query_reviewees(reviewee_request, None)
                                        .await
                                    {
                                        Ok(response) => {
                                            if let Some(data) = response.data {
                                                println!(
                                                    "    ✅ 获取被评估人信息成功，数量: {}",
                                                    data.reviewees.len()
                                                );
                                            }
                                        }
                                        Err(e) => {
                                            println!("    ❌ 获取被评估人信息失败: {e:?}");
                                        }
                                    }

                                    // 获取评估模板配置
                                    println!("  1.4 获取评估模板配置");
                                    let template_request =
                                        review_config::ReviewTemplateQueryRequest {
                                            activity_id: first_activity.activity_id.clone(),
                                            template_type: None,
                                        };

                                    match client
                                        .performance
                                        .review_config
                                        .query_review_templates(template_request, None)
                                        .await
                                    {
                                        Ok(response) => {
                                            if let Some(data) = response.data {
                                                println!(
                                                    "    ✅ 获取评估模板成功，数量: {}",
                                                    data.review_templates.len()
                                                );

                                                // 如果有评估模板，获取评估项
                                                if !data.review_templates.is_empty() {
                                                    let first_template = &data.review_templates[0];
                                                    println!("  1.5 获取评估项列表");
                                                    let item_request =
                                                        review_config::ReviewItemQueryRequest {
                                                            template_id: first_template
                                                                .template_id
                                                                .clone(),
                                                        };

                                                    match client
                                                        .performance
                                                        .review_config
                                                        .query_review_items(item_request, None)
                                                        .await
                                                    {
                                                        Ok(response) => {
                                                            if let Some(data) = response.data {
                                                                println!(
                                                        "    ✅ 获取评估项成功，数量: {}",
                                                        data.review_items.len()
                                                    );
                                                            }
                                                        }
                                                        Err(e) => {
                                                            println!(
                                                                "    ❌ 获取评估项失败: {e:?}"
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            println!("    ❌ 获取评估模板失败: {e:?}");
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("    ❌ 获取项目列表失败: {e:?}");
                        }
                    }
                } else {
                    println!("    ⚠️  未获取到项目数据");
                }
            } else {
                println!("    ⚠️  未获取到周期数据");
            }
        }
        Err(e) => {
            println!("    ❌ 获取周期列表失败: {e:?}");
        }
    }

    // 2. 指标管理示例
    println!("\n2. 指标管理");

    // 获取指标列表
    println!("  2.1 获取指标列表");
    let metric_request = review_config::MetricQueryRequest {
        key_only: Some(true),
        ..Default::default()
    };

    match client
        .performance
        .review_config
        .query_metrics(metric_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("    ✅ 获取指标列表成功，数量: {}", data.metrics.len());
            }
        }
        Err(e) => {
            println!("    ❌ 获取指标列表失败: {e:?}");
        }
    }

    // 获取指标标签列表
    println!("  2.2 获取指标标签列表");
    let tag_request = review_config::MetricTagListRequest {
        page_size: Some(10),
        ..Default::default()
    };

    match client
        .performance
        .review_config
        .list_metric_tags(tag_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!(
                    "    ✅ 获取指标标签成功，数量: {}",
                    data.metric_tags.items.len()
                );
            }
        }
        Err(e) => {
            println!("    ❌ 获取指标标签失败: {e:?}");
        }
    }

    // 3. 评估任务管理示例
    println!("\n3. 评估任务管理");

    // 获取周期任务（分页）
    println!("  3.1 获取周期任务（分页）");
    let task_page_request = stage_task::TaskFindByPageRequest {
        page_size: Some(10),
        ..Default::default()
    };

    match client
        .performance
        .stage_task
        .find_tasks_by_page(task_page_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("    ✅ 获取任务列表成功，数量: {}", data.tasks.items.len());
            }
        }
        Err(e) => {
            println!("    ❌ 获取任务列表失败: {e:?}");
        }
    }

    // 4. 指标数据管理示例
    println!("\n4. 指标数据管理");

    // 示例：查询指标详情（需要实际的activity_id）
    println!("  4.1 查询指标详情");
    // 注意：这里使用示例ID，实际使用时需要替换为真实的ID
    let metric_detail_request = metric_detail::MetricDetailQueryRequest {
        activity_id: "example_activity_id".to_string(),
        reviewee_ids: None,
        metric_ids: None,
    };

    match client
        .performance
        .metric_detail
        .query_metric_details(metric_detail_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!(
                    "    ✅ 查询指标详情成功，数量: {}",
                    data.metric_details.len()
                );
            }
        }
        Err(e) => {
            println!("    ⚠️  查询指标详情失败（可能是示例ID不存在）: {e:?}");
        }
    }

    // 5. 评估数据管理示例
    println!("\n5. 评估数据管理");

    // 获取绩效结果
    println!("  5.1 获取绩效结果");
    let result_request = review_data::ResultQueryRequest {
        result_opened_only: Some(true),
        ..Default::default()
    };

    match client
        .performance
        .review_data
        .query_results(result_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!(
                    "    ✅ 获取绩效结果成功，数量: {}",
                    data.performance_results.len()
                );
            }
        }
        Err(e) => {
            println!("    ❌ 获取绩效结果失败: {e:?}");
        }
    }

    // 6. 补充信息管理示例
    println!("\n6. 补充信息管理");

    // 示例：查询补充信息（需要实际的activity_id）
    println!("  6.1 查询补充信息");
    let additional_info_request = review_config::AdditionalInfoQueryRequest {
        activity_id: "example_activity_id".to_string(),
        user_ids: None,
    };

    match client
        .performance
        .review_config
        .query_additional_information(additional_info_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!(
                    "    ✅ 查询补充信息成功，数量: {}",
                    data.additional_information.len()
                );
            }
        }
        Err(e) => {
            println!("    ⚠️  查询补充信息失败（可能是示例ID不存在）: {e:?}");
        }
    }

    println!("\n=== 绩效管理功能演示完成 ===");
    println!("💡 提示：部分接口可能需要具体的项目ID才能正常返回数据");
    println!("🔍 主要功能包括：");
    println!("   • 后台配置：周期、项目、补充信息、人员组、评估模板、指标配置");
    println!("   • 评估任务：任务查询和管理");
    println!("   • 指标数据：关键指标数据获取和录入");
    println!("   • 评估数据：绩效结果和详情数据查询");
    println!("   • 事件推送：绩效结果开通和详情变更事件");

    Ok(())
}
