use dotenvy::dotenv;
use open_lark::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("Missing APP_ID environment variable");
    let app_secret = env::var("APP_SECRET").expect("Missing APP_SECRET environment variable");

    // 创建 Lark 客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("=== 飞书妙记 API v1 示例 ===");

    // 示例妙记token，实际使用时需要从妙记中获取
    let minute_token = "minute_token_example";

    // 1. 获取妙记信息
    println!("\n1. 获取妙记信息");
    match client.minutes.v1.minute.get(minute_token, None, None).await {
        Ok(response) => {
            println!("妙记信息: {response:?}");
            if let Some(minute) = &response.data {
                println!("妙记标题: {:?}", minute.minute.title);
                println!("创建时间: {:?}", minute.minute.create_time);
                println!("状态: {:?}", minute.minute.status);
            }
        }
        Err(e) => eprintln!("获取妙记信息失败: {e:?}"),
    }

    // 2. 下载妙记音视频文件
    println!("\n2. 下载妙记音视频文件");
    match client.minutes.v1.media.get(minute_token, None, None).await {
        Ok(response) => {
            println!("音视频文件信息: {response:?}");
            if let Some(media) = &response.data {
                println!("文件名: {:?}", media.media.filename);
                println!("文件大小: {:?}", media.media.file_size);
                println!("文件类型: {:?}", media.media.file_type);
                println!("下载URL: {:?}", media.media.download_url);
                println!("有效期: {:?}", media.media.expires_time);
            }
        }
        Err(e) => eprintln!("下载音视频文件失败: {e:?}"),
    }

    // 3. 导出妙记文字记录
    println!("\n3. 导出妙记文字记录");
    match client
        .minutes
        .v1
        .transcript
        .get(minute_token, None, None)
        .await
    {
        Ok(response) => {
            println!("文字记录信息: {response:?}");
            if let Some(transcript) = &response.data {
                println!(
                    "转录内容预览: {}",
                    transcript
                        .transcript
                        .content
                        .chars()
                        .take(100)
                        .collect::<String>()
                );
                println!("语言: {:?}", transcript.transcript.language);
                println!("格式: {:?}", transcript.transcript.format);
                println!("创建时间: {:?}", transcript.transcript.create_time);
            }
        }
        Err(e) => eprintln!("导出文字记录失败: {e:?}"),
    }

    // 4. 获取妙记统计数据
    println!("\n4. 获取妙记统计数据");
    match client
        .minutes
        .v1
        .statistics
        .get(minute_token, None, None)
        .await
    {
        Ok(response) => {
            println!("统计数据: {response:?}");
            if let Some(stats) = &response.data {
                println!("会议时长: {:?} 秒", stats.statistics.duration);
                println!("参会人数: {:?}", stats.statistics.participant_count);
                println!("发言次数: {:?}", stats.statistics.speech_count);
                println!("发言时长: {:?} 秒", stats.statistics.speech_duration);
                println!("静音时长: {:?} 秒", stats.statistics.mute_duration);

                if let Some(keywords) = &stats.statistics.keywords {
                    println!("关键词统计:");
                    for keyword in keywords.iter().take(5) {
                        println!("  - {}: {} 次", keyword.keyword, keyword.count);
                    }
                }
            }
        }
        Err(e) => eprintln!("获取统计数据失败: {e:?}"),
    }

    // 5. 演示批量获取多个妙记信息
    println!("\n5. 批量获取妙记信息示例");
    let minute_tokens = vec!["minute_token_1", "minute_token_2", "minute_token_3"];

    for token in minute_tokens {
        println!("\n处理妙记: {token}");

        // 并发获取妙记信息和统计数据
        let minute_future = client.minutes.v1.minute.get(token, None, None);
        let stats_future = client.minutes.v1.statistics.get(token, None, None);

        match tokio::try_join!(minute_future, stats_future) {
            Ok((minute_response, stats_response)) => {
                if let (Some(minute), Some(stats)) = (&minute_response.data, &stats_response.data) {
                    println!("  标题: {:?}", minute.minute.title);
                    println!("  时长: {:?} 秒", stats.statistics.duration);
                    println!("  参会人数: {:?}", stats.statistics.participant_count);
                }
            }
            Err(e) => eprintln!("  批量获取失败: {e:?}"),
        }
    }

    println!("\n=== Minutes API v1 示例执行完成 ===");
    Ok(())
}
