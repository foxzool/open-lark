use super::LarkWsClient;
use lark_websocket_protobuf::pbbp2::{Frame, Header};

#[tokio::test]
async fn test_single_package_payload_preservation() {
    let mut client = LarkWsClient::new_for_test();

    // 创建单包消息的 frame
    let test_payload = b"test payload data".to_vec();
    let frame = Frame {
        seq_id: 1,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "test_msg_001".to_string(),
            },
            // 没有 sum 和 seq 头部，应该默认为单包
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(test_payload.clone()),
        log_id_new: None,
    };

    // 处理单包消息
    let result = client.process_frame_packages(frame).await;

    // 验证结果
    assert!(result.is_some());
    let processed_frame = result.unwrap();
    assert!(processed_frame.payload.is_some());
    assert_eq!(processed_frame.payload.unwrap(), test_payload);
}

#[tokio::test]
async fn test_multi_package_payload_combination() {
    let mut client = LarkWsClient::new_for_test();

    let part1 = b"Hello ".to_vec();
    let part2 = b"World!".to_vec();
    let combined = b"Hello World!".to_vec();

    // 创建第一个包
    let frame1 = Frame {
        seq_id: 1,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "test_msg_002".to_string(),
            },
            Header {
                key: "sum".to_string(),
                value: "2".to_string(),
            },
            Header {
                key: "seq".to_string(),
                value: "0".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(part1),
        log_id_new: None,
    };

    // 处理第一个包 - 应该返回 None 因为还没收齐
    let result1 = client.process_frame_packages(frame1).await;
    assert!(result1.is_none());

    // 创建第二个包
    let frame2 = Frame {
        seq_id: 2,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "test_msg_002".to_string(),
            },
            Header {
                key: "sum".to_string(),
                value: "2".to_string(),
            },
            Header {
                key: "seq".to_string(),
                value: "1".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(part2),
        log_id_new: None,
    };

    // 处理第二个包 - 应该返回组合后的完整消息
    let result2 = client.process_frame_packages(frame2).await;
    assert!(result2.is_some());
    let processed_frame = result2.unwrap();
    assert!(processed_frame.payload.is_some());
    assert_eq!(processed_frame.payload.unwrap(), combined);
}
