//! Sheets v2 批量写入数据示例
//!
//! 本示例展示如何使用飞书开放平台SDK v2版本的电子表格API来批量写入多个范围的数据。
//! 支持一次性写入多个单元格范围，提高数据更新效率。

use open_lark::core::config::Config;
use open_lark::prelude::*;
use open_lark::service::sheets::v2::{
    BatchWriteService, CellValue, WriteMultipleRangesRequest, WriteRange,
};

#[tokio::main]
async fn main() -> SDKResult<()> {
    // 初始化配置和服务
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build();

    let batch_service = BatchWriteService::new(config);

    // 示例1: 批量写入两个范围的基础数据
    println!("=== 示例1: 批量写入两个范围的基础数据 ===");
    let request = WriteMultipleRangesRequest::new(
        "shtcnmBA*****yGehy8", // 电子表格令牌
        vec![
            WriteRange::new(
                "Sheet1!A1:C3",
                vec![
                    vec![
                        CellValue::text("员工编号"),
                        CellValue::text("姓名"),
                        CellValue::text("部门"),
                    ],
                    vec![
                        CellValue::text("E001"),
                        CellValue::text("张三"),
                        CellValue::text("技术部"),
                    ],
                    vec![
                        CellValue::text("E002"),
                        CellValue::text("李四"),
                        CellValue::text("产品部"),
                    ],
                ],
            ),
            WriteRange::new(
                "Sheet2!A1:B2",
                vec![
                    vec![CellValue::text("项目名称"), CellValue::text("负责人")],
                    vec![CellValue::text("项目A"), CellValue::text("张三")],
                ],
            ),
        ],
    );

    match batch_service.write_multiple_ranges(request, None).await {
        Ok(response) => {
            println!("✅ 批量写入成功!");
            if let Some(data) = &response.data {
                println!("电子表格令牌: {}", data.spreadsheet_token);
                println!("写入范围数: {}", data.total_updated_ranges);
                println!("总单元格数: {}", data.total_updated_cells);
                println!("表格版本: {}", data.revision);
            }
        }
        Err(error) => {
            println!("❌ 批量写入失败: {}", error.user_friendly_message());
        }
    }

    // 示例2: 使用Builder模式批量写入多个范围
    println!("\n=== 示例2: 使用Builder模式批量写入多个范围 ===");
    let builder_request = WriteMultipleRangesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .add_range(
            "销售数据!A1:F5",
            vec![
                vec![
                    CellValue::text("产品"),
                    CellValue::text("Q1销量"),
                    CellValue::text("Q2销量"),
                    CellValue::text("Q3销量"),
                    CellValue::text("Q4销量"),
                    CellValue::text("总计"),
                ],
                vec![
                    CellValue::text("产品A"),
                    CellValue::number(100),
                    CellValue::number(120),
                    CellValue::number(110),
                    CellValue::number(130),
                    CellValue::formula("=SUM(B2:E2)"),
                ],
                vec![
                    CellValue::text("产品B"),
                    CellValue::number(80),
                    CellValue::number(90),
                    CellValue::number(95),
                    CellValue::number(100),
                    CellValue::formula("=SUM(B3:E3)"),
                ],
                vec![
                    CellValue::text("产品C"),
                    CellValue::number(60),
                    CellValue::number(75),
                    CellValue::number(85),
                    CellValue::number(90),
                    CellValue::formula("=SUM(B4:E4)"),
                ],
                vec![
                    CellValue::text("总计"),
                    CellValue::formula("=SUM(B2:B4)"),
                    CellValue::formula("=SUM(C2:C4)"),
                    CellValue::formula("=SUM(D2:D4)"),
                    CellValue::formula("=SUM(E2:E4)"),
                    CellValue::formula("=SUM(F2:F4)"),
                ],
            ],
        )
        .add_range(
            "库存数据!A1:D3",
            vec![
                vec![
                    CellValue::text("产品"),
                    CellValue::text("当前库存"),
                    CellValue::text("安全库存"),
                    CellValue::text("库存状态"),
                ],
                vec![
                    CellValue::text("产品A"),
                    CellValue::number(50),
                    CellValue::number(30),
                    CellValue::text("充足"),
                ],
                vec![
                    CellValue::text("产品B"),
                    CellValue::number(25),
                    CellValue::number(40),
                    CellValue::text("不足"),
                ],
            ],
        )
        .value_render_option("FormattedValue")
        .date_time_render_option("FormattedString")
        .user_id_type("open_id")
        .build();

    match batch_service
        .write_multiple_ranges(builder_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ Builder模式批量写入成功!");
            if let Some(data) = &response.data {
                println!("写入了 {} 个范围", data.total_updated_ranges);
                println!("总计 {} 个单元格", data.total_updated_cells);
            }
        }
        Err(error) => {
            println!(
                "❌ Builder模式批量写入失败: {}",
                error.user_friendly_message()
            );
        }
    }

    // 示例3: 从元组向量批量写入
    println!("\n=== 示例3: 从元组向量批量写入 ===");
    let ranges_data = vec![
        (
            "财务数据!A1:C4",
            vec![
                vec![
                    CellValue::text("科目"),
                    CellValue::text("预算"),
                    CellValue::text("实际"),
                ],
                vec![
                    CellValue::text("收入"),
                    CellValue::number(100000),
                    CellValue::number(120000),
                ],
                vec![
                    CellValue::text("成本"),
                    CellValue::number(60000),
                    CellValue::number(55000),
                ],
                vec![
                    CellValue::text("利润"),
                    CellValue::number(40000),
                    CellValue::number(65000),
                ],
            ],
        ),
        (
            "人员数据!A1:B3",
            vec![
                vec![CellValue::text("部门"), CellValue::text("人数")],
                vec![CellValue::text("技术部"), CellValue::number(15)],
                vec![CellValue::text("产品部"), CellValue::number(8)],
            ],
        ),
    ];

    match batch_service
        .write_ranges_from_vec("shtcnmBA*****yGehy8", ranges_data, None)
        .await
    {
        Ok(response) => {
            println!("✅ 元组向量批量写入成功!");
            if let Some(data) = &response.data {
                println!("写入范围数: {}", data.total_updated_ranges);
                println!("总计单元格数: {}", data.total_updated_cells);
            }
        }
        Err(error) => {
            println!("❌ 元组向量批量写入失败: {}", error.user_friendly_message());
        }
    }

    // 示例4: 便捷方法写入单个范围
    println!("\n=== 示例4: 便捷方法写入单个范围 ===");
    match batch_service
        .write_single_range(
            "shtcnmBA*****yGehy8",
            "汇总数据!A1:E1",
            vec![vec![
                CellValue::text("总员工数"),
                CellValue::number(23),
                CellValue::text("总项目数"),
                CellValue::number(5),
                CellValue::text("最后更新"),
            ]],
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✅ 单个范围写入成功!");
            if let Some(data) = &response.data {
                if let Some(updated_range) = data.updated_ranges.first() {
                    println!("范围: {}", updated_range.range);
                    println!("更新单元格数: {}", updated_range.updated_cells);
                }
            }
        }
        Err(error) => {
            println!("❌ 单个范围写入失败: {}", error.user_friendly_message());
        }
    }

    // 示例5: 处理不同数据类型
    println!("\n=== 示例5: 处理不同数据类型 ===");
    let mixed_data_request = WriteMultipleRangesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .add_range(
            "数据类型测试!A1:F2",
            vec![
                vec![
                    CellValue::text("文本类型"),
                    CellValue::number(123.45),
                    CellValue::boolean(true),
                    CellValue::formula("=SUM(B2:B5)"),
                    CellValue::Blank,
                    CellValue::Error("#N/A".to_string()),
                ],
                vec![
                    CellValue::text("字符串值"),
                    CellValue::number(678.90),
                    CellValue::boolean(false),
                    CellValue::formula("=AVERAGE(B2:B5)"),
                    CellValue::text(""),
                    CellValue::Error("#REF!".to_string()),
                ],
            ],
        )
        .build();

    match batch_service
        .write_multiple_ranges(mixed_data_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 混合数据类型写入成功!");
        }
        Err(error) => {
            println!("❌ 混合数据类型写入失败: {}", error.user_friendly_message());
        }
    }

    // 示例6: 动态添加范围
    println!("\n=== 示例6: 动态添加范围 ===");
    let mut dynamic_request = WriteMultipleRangesRequest::new(
        "shtcnmBA*****yGehy8",
        vec![WriteRange::new(
            "基础数据!A1:C2",
            vec![
                vec![
                    CellValue::text("项目"),
                    CellValue::text("状态"),
                    CellValue::text("进度"),
                ],
                vec![
                    CellValue::text("项目1"),
                    CellValue::text("进行中"),
                    CellValue::number(75),
                ],
            ],
        )],
    );

    // 根据条件动态添加范围
    let additional_ranges = vec![
        WriteRange::new(
            "扩展数据1!A1:B3",
            vec![
                vec![CellValue::text("任务"), CellValue::text("负责人")],
                vec![CellValue::text("任务A"), CellValue::text("张三")],
                vec![CellValue::text("任务B"), CellValue::text("李四")],
            ],
        ),
        WriteRange::new(
            "扩展数据2!A1:C2",
            vec![
                vec![
                    CellValue::text("里程碑"),
                    CellValue::text("截止日期"),
                    CellValue::text("完成状态"),
                ],
                vec![
                    CellValue::text("阶段1"),
                    CellValue::text("2024-03-31"),
                    CellValue::text("已完成"),
                ],
            ],
        ),
    ];

    for range in additional_ranges {
        dynamic_request = dynamic_request.add_range(range);
    }

    println!("最终范围数量: {}", dynamic_request.range_count());
    println!("总单元格数: {}", dynamic_request.total_cell_count());

    match batch_service
        .write_multiple_ranges(dynamic_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 动态范围写入成功!");
        }
        Err(error) => {
            println!("❌ 动态范围写入失败: {}", error.user_friendly_message());
        }
    }

    // 示例7: 不同渲染选项对比
    println!("\n=== 示例7: 不同渲染选项对比 ===");

    // 默认渲染
    let default_request = WriteMultipleRangesRequest::new(
        "shtcnmBA*****yGehy8",
        vec![WriteRange::new(
            "默认渲染!A1:B2",
            vec![
                vec![CellValue::text("测试"), CellValue::number(123)],
                vec![CellValue::formula("=A1*2"), CellValue::boolean(true)],
            ],
        )],
    );

    // 格式化值渲染
    let formatted_request = WriteMultipleRangesRequest::new(
        "shtcnmBA*****yGehy8",
        vec![WriteRange::new(
            "格式化渲染!A1:B2",
            vec![
                vec![CellValue::text("测试"), CellValue::number(123)],
                vec![CellValue::formula("=A1*2"), CellValue::boolean(true)],
            ],
        )],
    )
    .value_render_option("FormattedValue");

    // 公式渲染
    let formula_request = WriteMultipleRangesRequest::new(
        "shtcnmBA*****yGehy8",
        vec![WriteRange::new(
            "公式渲染!A1:B2",
            vec![
                vec![CellValue::text("测试"), CellValue::number(123)],
                vec![CellValue::formula("=A1*2"), CellValue::boolean(true)],
            ],
        )],
    )
    .value_render_option("Formula");

    println!("已创建不同渲染选项的请求，可根据需要选择合适的渲染方式");

    // 示例8: 大规模数据写入
    println!("\n=== 示例8: 大规模数据写入 ===");

    // 生成大量数据用于性能测试
    let mut large_data = Vec::new();
    for i in 1..=50 {
        let row = vec![
            CellValue::text(format!("项目{}", i)),
            CellValue::number((i as f64) * 100.0),
            CellValue::text(if i % 2 == 0 { "已完成" } else { "进行中" }),
            CellValue::number((i as f64) * 0.8),
            CellValue::text(format!(
                "负责人{}",
                if i % 3 == 0 {
                    "张三"
                } else if i % 3 == 1 {
                    "李四"
                } else {
                    "王五"
                }
            )),
        ];
        large_data.push(row);
    }

    let large_request = WriteMultipleRangesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .add_range("大数据表!A1:E50", large_data)
        .value_render_option("UnformattedValue") // 使用未格式化值以提高性能
        .build();

    println!("准备写入 {} 行 {} 列的数据", 50, 5);
    println!("总计 {} 个单元格", large_request.total_cell_count());

    match batch_service
        .write_multiple_ranges(large_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 大规模写入成功!");
            if let Some(data) = &response.data {
                println!("写入范围数: {}", data.total_updated_ranges);
                println!("总单元格数: {}", data.total_updated_cells);
                println!("表格版本: {}", data.revision);
            }
        }
        Err(error) => {
            println!("❌ 大规模写入失败: {}", error.user_friendly_message());
        }
    }

    // 示例9: 错误处理演示
    println!("\n=== 示例9: 错误处理演示 ===");

    // 测试无效范围格式
    let invalid_range_request = WriteMultipleRangesRequest::new(
        "shtcnmBA*****yGehy8",
        vec![WriteRange::new(
            "InvalidRangeWithoutSheet",
            vec![vec![CellValue::text("测试")]],
        )],
    );

    match batch_service
        .write_multiple_ranges(invalid_range_request, None)
        .await
    {
        Ok(_) => {
            println!("意外成功，应该失败");
        }
        Err(error) => {
            println!("✅ 正确捕获无效范围错误: {}", error.user_friendly_message());
        }
    }

    // 测试空电子表格令牌
    let empty_token_request = WriteMultipleRangesRequest::new(
        "",
        vec![WriteRange::new(
            "Sheet1!A1:B2",
            vec![vec![CellValue::text("测试")]],
        )],
    );

    match batch_service
        .write_multiple_ranges(empty_token_request, None)
        .await
    {
        Ok(_) => {
            println!("意外成功，应该失败");
        }
        Err(error) => {
            println!("✅ 正确捕获空令牌错误: {}", error.user_friendly_message());
        }
    }

    // 测试数据格式不一致
    let inconsistent_data_request = WriteMultipleRangesRequest::new(
        "shtcnmBA*****yGehy8",
        vec![WriteRange::new(
            "Sheet1!A1:B2",
            vec![
                vec![CellValue::text("A1"), CellValue::text("B1")],
                vec![CellValue::text("A2")], // 缺少B2
            ],
        )],
    );

    match batch_service
        .write_multiple_ranges(inconsistent_data_request, None)
        .await
    {
        Ok(_) => {
            println!("意外成功，应该失败");
        }
        Err(error) => {
            println!(
                "✅ 正确捕获数据格式不一致错误: {}",
                error.user_friendly_message()
            );
        }
    }

    // 示例10: Unicode和中文支持
    println!("\n=== 示例10: Unicode和中文支持 ===");
    let unicode_request = WriteMultipleRangesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .add_range(
            "中文工作表!A1:D4",
            vec![
                vec![
                    CellValue::text("员工姓名"),
                    CellValue::text("职位"),
                    CellValue::text("部门"),
                    CellValue::text("入职日期"),
                ],
                vec![
                    CellValue::text("张三"),
                    CellValue::text("高级工程师"),
                    CellValue::text("技术研发部🚀"),
                    CellValue::text("2023-01-15"),
                ],
                vec![
                    CellValue::text("李四"),
                    CellValue::text("产品经理"),
                    CellValue::text("产品设计部✨"),
                    CellValue::text("2022-08-20"),
                ],
                vec![
                    CellValue::text("王五"),
                    CellValue::text("UI设计师"),
                    CellValue::text("用户体验设计部🎨"),
                    CellValue::text("2023-03-10"),
                ],
            ],
        )
        .add_range(
            "emoji测试!A1:B2",
            vec![
                vec![CellValue::text("项目状态"), CellValue::text("心情指数")],
                vec![CellValue::text("进行中🔥"), CellValue::text("很好😊")],
            ],
        )
        .build();

    match batch_service
        .write_multiple_ranges(unicode_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ Unicode和中文数据写入成功!");
            if let Some(data) = &response.data {
                println!("写入范围数: {}", data.total_updated_ranges);
                println!("总单元格数: {}", data.total_updated_cells);
            }
        }
        Err(error) => {
            println!("❌ Unicode数据写入失败: {}", error.user_friendly_message());
        }
    }

    println!("\n=== 所有示例执行完成 ===");
    println!("注意：当前实现返回模拟数据，实际使用时需要配置有效的访问令牌");
    Ok(())
}
