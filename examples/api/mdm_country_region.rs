use dotenvy::dotenv;
use open_lark::{prelude::*, service::mdm::country_region::*};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 环境变量未设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 环境变量未设置");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    // 分页查询所有国家/地区
    println!("=== 分页查询所有国家/地区 ===");
    let list_request = CountryRegionListRequest {
        page_size: Some(10),
        ..Default::default()
    };

    match client.mdm.country_region.list(list_request, None).await {
        Ok(response) => {
            if let Some(list_data) = response.data {
                println!("查询国家/地区列表成功：");
                for country in &list_data.country_regions.items {
                    println!("  - 主数据编码: {}", country.master_data_code);
                    println!("    名称: {}", country.name);
                    if let Some(name_en) = &country.name_en {
                        println!("    英文名称: {name_en}");
                    }
                    if let Some(country_code) = &country.country_code {
                        println!("    国家代码: {country_code}");
                    }
                    if let Some(iso_country_code) = &country.iso_country_code {
                        println!("    ISO 国家代码: {iso_country_code}");
                    }
                    if let Some(region_type) = &country.region_type {
                        println!("    区域类型: {region_type}");
                    }
                    if let Some(status) = &country.status {
                        println!("    状态: {status}");
                    }
                    if let Some(sort_order) = country.sort_order {
                        println!("    排序序号: {sort_order}");
                    }
                    println!();
                }

                if let Some(has_more) = list_data.country_regions.has_more {
                    if has_more {
                        println!("还有更多数据，可使用 page_token 继续查询");
                        if let Some(page_token) = &list_data.country_regions.page_token {
                            println!("下一页 token: {page_token}");
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("查询国家/地区列表失败: {e:?}");
        }
    }

    // 根据主数据编码批量查询国家/地区
    println!("=== 根据主数据编码批量查询国家/地区 ===");
    let get_request = CountryRegionGetRequest {
        master_data_codes: vec![
            "CN".to_string(),
            "US".to_string(),
            "JP".to_string(),
            "UK".to_string(),
        ],
    };

    match client.mdm.country_region.get(get_request, None).await {
        Ok(response) => {
            if let Some(get_data) = response.data {
                println!("批量查询国家/地区成功：");
                for country in &get_data.country_regions {
                    println!("  - 主数据编码: {}", country.master_data_code);
                    println!("    名称: {}", country.name);
                    if let Some(name_en) = &country.name_en {
                        println!("    英文名称: {name_en}");
                    }
                    if let Some(country_code) = &country.country_code {
                        println!("    国家代码: {country_code}");
                    }
                    if let Some(iso_country_code) = &country.iso_country_code {
                        println!("    ISO 国家代码: {iso_country_code}");
                    }
                    if let Some(description) = &country.description {
                        println!("    描述: {description}");
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("批量查询国家/地区失败: {e:?}");
        }
    }

    // 按状态筛选查询
    println!("=== 按状态筛选查询国家/地区 ===");
    let filtered_request = CountryRegionListRequest {
        page_size: Some(5),
        status: Some("active".to_string()),
        ..Default::default()
    };

    match client.mdm.country_region.list(filtered_request, None).await {
        Ok(response) => {
            if let Some(filtered_data) = response.data {
                println!("按状态筛选查询成功：");
                for country in &filtered_data.country_regions.items {
                    println!("  - 主数据编码: {}", country.master_data_code);
                    println!("    名称: {}", country.name);
                    if let Some(status) = &country.status {
                        println!("    状态: {status}");
                    }
                    if let Some(region_type) = &country.region_type {
                        println!("    区域类型: {region_type}");
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("按状态筛选查询失败: {e:?}");
        }
    }

    // 按名称模糊查询
    println!("=== 按名称模糊查询国家/地区 ===");
    let name_search_request = CountryRegionListRequest {
        page_size: Some(5),
        name: Some("中国".to_string()),
        ..Default::default()
    };

    match client
        .mdm
        .country_region
        .list(name_search_request, None)
        .await
    {
        Ok(response) => {
            if let Some(search_data) = response.data {
                println!("按名称模糊查询成功：");
                for country in &search_data.country_regions.items {
                    println!("  - 主数据编码: {}", country.master_data_code);
                    println!("    名称: {}", country.name);
                    if let Some(name_en) = &country.name_en {
                        println!("    英文名称: {name_en}");
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("按名称模糊查询失败: {e:?}");
        }
    }

    Ok(())
}
