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

    println!("=== 飞书视频会议 API v1 示例 ===");

    // 1. 预约会议
    println!("\n1. 预约会议");
    let reserve_req = open_lark::service::vc::v1::reserve::ApplyReserveRequest {
        topic: "Rust SDK 测试会议".to_string(),
        start_time: "2024-12-26T14:00:00Z".to_string(),
        end_time: "2024-12-26T15:00:00Z".to_string(),
        host_user_id: "user_123".to_string(),
        password: Some("123456".to_string()),
        participants: Some(vec!["user_456".to_string(), "user_789".to_string()]),
        room_id: None,
    };

    match client.vc.v1.reserve.apply(reserve_req, None, None).await {
        Ok(response) => {
            println!("预约成功: {response:?}");
            let reserve_id = &response.data.as_ref().unwrap().reserve.id;

            // 2. 获取预约详情
            println!("\n2. 获取预约详情");
            match client.vc.v1.reserve.get(reserve_id, None, None).await {
                Ok(get_response) => println!("预约详情: {get_response:?}"),
                Err(e) => eprintln!("获取预约详情失败: {e:?}"),
            }

            // 3. 获取活跃会议
            println!("\n3. 获取活跃会议");
            match client
                .vc
                .v1
                .reserve
                .get_active_meeting(reserve_id, None, None)
                .await
            {
                Ok(meeting_response) => {
                    println!("活跃会议: {meeting_response:?}");
                    let meeting_id = &meeting_response.data.as_ref().unwrap().meeting.id;

                    // 4. 邀请参会人
                    println!("\n4. 邀请参会人");
                    let invite_req = open_lark::service::vc::v1::meeting::InviteMeetingRequest {
                        invitees: vec!["user_abc".to_string(), "user_def".to_string()],
                    };

                    match client
                        .vc
                        .v1
                        .meeting
                        .invite(meeting_id, invite_req, None, None)
                        .await
                    {
                        Ok(invite_response) => println!("邀请结果: {invite_response:?}"),
                        Err(e) => eprintln!("邀请失败: {e:?}"),
                    }

                    // 5. 开始录制
                    println!("\n5. 开始录制");
                    let start_recording_req =
                        open_lark::service::vc::v1::recording::StartRecordingRequest {
                            title: Some("测试录制".to_string()),
                        };

                    match client
                        .vc
                        .v1
                        .recording
                        .start(meeting_id, start_recording_req, None, None)
                        .await
                    {
                        Ok(recording_response) => {
                            println!("录制开始: {recording_response:?}");

                            // 6. 停止录制
                            println!("\n6. 停止录制");
                            match client.vc.v1.recording.stop(meeting_id, None, None).await {
                                Ok(stop_response) => println!("录制停止: {stop_response:?}"),
                                Err(e) => eprintln!("停止录制失败: {e:?}"),
                            }
                        }
                        Err(e) => eprintln!("开始录制失败: {e:?}"),
                    }
                }
                Err(e) => eprintln!("获取活跃会议失败: {e:?}"),
            }
        }
        Err(e) => eprintln!("预约会议失败: {e:?}"),
    }

    // 7. 会议室管理
    println!("\n7. 会议室管理");

    // 创建会议室
    let create_room_req = open_lark::service::vc::v1::room::CreateRoomRequest {
        name: "Rust SDK 测试会议室".to_string(),
        description: Some("用于测试的会议室".to_string()),
        capacity: Some(20),
        location: Some("北京市朝阳区".to_string()),
    };

    match client.vc.v1.room.create(create_room_req, None, None).await {
        Ok(room_response) => {
            println!("会议室创建成功: {room_response:?}");
            let room_id = &room_response.data.as_ref().unwrap().room.room_id;

            // 获取会议室详情
            match client.vc.v1.room.get(room_id, None, None, None).await {
                Ok(get_room_response) => println!("会议室详情: {get_room_response:?}"),
                Err(e) => eprintln!("获取会议室详情失败: {e:?}"),
            }
        }
        Err(e) => eprintln!("创建会议室失败: {e:?}"),
    }

    // 8. 搜索会议室
    println!("\n8. 搜索会议室");
    let search_params = open_lark::service::vc::v1::room::SearchRoomsParams {
        keyword: Some("测试".to_string()),
        room_ids: None,
        page_size: Some(10),
        page_token: None,
        room_id_type: None,
        user_id_type: None,
    };
    match client.vc.v1.room.search(Some(search_params), None).await {
        Ok(search_response) => println!("搜索结果: {search_response:?}"),
        Err(e) => eprintln!("搜索会议室失败: {e:?}"),
    }

    // 9. 获取会议列表
    println!("\n9. 获取会议列表");
    let list_params = open_lark::service::vc::v1::meeting::ListMeetingsByNoParams {
        page_size: Some(10),
        page_token: None,
        user_id_type: None,
    };
    match client
        .vc
        .v1
        .meeting
        .list_by_no(
            "123456789",            // meeting_no
            "2024-12-25T00:00:00Z", // start_time
            "2024-12-27T23:59:59Z", // end_time
            Some(list_params),      // params
            None,                   // option
        )
        .await
    {
        Ok(list_response) => println!("会议列表: {list_response:?}"),
        Err(e) => eprintln!("获取会议列表失败: {e:?}"),
    }

    println!("\n=== VC API v1 示例执行完成 ===");
    Ok(())
}
