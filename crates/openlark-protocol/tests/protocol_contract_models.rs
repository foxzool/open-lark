//! WebSocket 协议消息结构的契约测试。
//!
//! 验证 protobuf 生成的 Header / Frame 结构体在 prost 编解码过程中的双向一致性。

use openlark_protocol::pbbp2::{Frame, Header};
use prost::Message;

fn assert_encode_decode_roundtrip<T>(original: &T)
where
    T: Message + PartialEq + std::fmt::Debug + Default,
{
    let bytes = original.encode_to_vec();
    let decoded = T::decode(bytes.as_slice()).expect("解码不应失败");
    assert_eq!(original, &decoded, "编解码往返后结构体应一致");
}

fn encode_contract<T>(value: &T, expected: &[u8])
where
    T: Message,
{
    let bytes = value.encode_to_vec();
    assert_eq!(bytes, expected, "编码结果应与预期字节一致");
}

fn decode_contract<T>(payload: &[u8]) -> T
where
    T: Message + Default,
{
    T::decode(payload).expect("解码不应失败")
}

#[test]
fn header_roundtrip() {
    let header = Header {
        key: "content-type".to_string(),
        value: "application/json".to_string(),
    };
    assert_encode_decode_roundtrip(&header);
}

#[test]
fn header_encode_decode_contract() {
    let header = Header {
        key: "x-request-id".to_string(),
        value: "req-abc-123".to_string(),
    };
    let bytes = header.encode_to_vec();
    let decoded: Header = decode_contract(&bytes);
    assert_eq!(decoded.key, "x-request-id");
    assert_eq!(decoded.value, "req-abc-123");
    encode_contract(&decoded, &bytes);
}

#[test]
fn header_empty_values() {
    let header = Header {
        key: String::new(),
        value: String::new(),
    };
    assert_encode_decode_roundtrip(&header);
}

#[test]
fn header_unicode_values() {
    let header = Header {
        key: "飞书头".to_string(),
        value: "值=测试&编码".to_string(),
    };
    assert_encode_decode_roundtrip(&header);
}

#[test]
fn frame_minimal_roundtrip() {
    let frame = Frame {
        seq_id: 1,
        log_id: 100,
        service: 1,
        method: 1,
        headers: vec![],
        payload_encoding: None,
        payload_type: None,
        payload: None,
        log_id_new: None,
    };
    assert_encode_decode_roundtrip(&frame);
}

#[test]
fn frame_full_fields_roundtrip() {
    let frame = Frame {
        seq_id: 42,
        log_id: 999,
        service: 5,
        method: 10,
        headers: vec![
            Header {
                key: "authorization".to_string(),
                value: "Bearer token_xxx".to_string(),
            },
            Header {
                key: "x-custom".to_string(),
                value: "自定义值".to_string(),
            },
        ],
        payload_encoding: Some("gzip".to_string()),
        payload_type: Some("application/json".to_string()),
        payload: Some(b"{\"event\":\"message\"}".to_vec()),
        log_id_new: Some("log-new-001".to_string()),
    };
    assert_encode_decode_roundtrip(&frame);
}

#[test]
fn frame_with_headers_contract() {
    let frame = Frame {
        seq_id: 100,
        log_id: 200,
        service: 3,
        method: 7,
        headers: vec![Header {
            key: "content-type".to_string(),
            value: "text/plain".to_string(),
        }],
        payload_encoding: None,
        payload_type: Some("text/plain".to_string()),
        payload: Some(b"hello".to_vec()),
        log_id_new: None,
    };
    let bytes = frame.encode_to_vec();
    let decoded: Frame = decode_contract(&bytes);
    assert_eq!(decoded.seq_id, 100);
    assert_eq!(decoded.log_id, 200);
    assert_eq!(decoded.service, 3);
    assert_eq!(decoded.method, 7);
    assert_eq!(decoded.headers.len(), 1);
    assert_eq!(decoded.headers[0].key, "content-type");
    assert_eq!(decoded.headers[0].value, "text/plain");
    assert_eq!(decoded.payload_type.as_deref(), Some("text/plain"));
    assert_eq!(decoded.payload.as_deref(), Some(b"hello".as_slice()));
    assert!(decoded.payload_encoding.is_none());
    assert!(decoded.log_id_new.is_none());
}

#[test]
fn frame_empty_payload_roundtrip() {
    let frame = Frame {
        seq_id: 0,
        log_id: 0,
        service: 0,
        method: 0,
        headers: vec![],
        payload_encoding: None,
        payload_type: None,
        payload: Some(vec![]),
        log_id_new: None,
    };
    assert_encode_decode_roundtrip(&frame);
}

#[test]
fn frame_large_seq_and_log_id() {
    let frame = Frame {
        seq_id: u64::MAX,
        log_id: u64::MAX,
        service: i32::MAX,
        method: i32::MIN,
        headers: vec![],
        payload_encoding: None,
        payload_type: None,
        payload: None,
        log_id_new: None,
    };
    let bytes = frame.encode_to_vec();
    let decoded: Frame = decode_contract(&bytes);
    assert_eq!(decoded.seq_id, u64::MAX);
    assert_eq!(decoded.log_id, u64::MAX);
    assert_eq!(decoded.service, i32::MAX);
    assert_eq!(decoded.method, i32::MIN);
}

#[test]
fn frame_binary_payload_roundtrip() {
    let binary_data: Vec<u8> = (0..=255).collect();
    let frame = Frame {
        seq_id: 1,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![],
        payload_encoding: Some("none".to_string()),
        payload_type: Some("application/octet-stream".to_string()),
        payload: Some(binary_data.clone()),
        log_id_new: None,
    };
    assert_encode_decode_roundtrip(&frame);
    let bytes = frame.encode_to_vec();
    let decoded: Frame = decode_contract(&bytes);
    assert_eq!(decoded.payload.as_deref(), Some(binary_data.as_slice()));
}

#[test]
fn frame_log_id_new_field() {
    let frame = Frame {
        seq_id: 1,
        log_id: 50,
        service: 1,
        method: 1,
        headers: vec![],
        payload_encoding: None,
        payload_type: None,
        payload: None,
        log_id_new: Some("log-id-new-uuid-xxx".to_string()),
    };
    let bytes = frame.encode_to_vec();
    let decoded: Frame = decode_contract(&bytes);
    assert_eq!(decoded.log_id_new.as_deref(), Some("log-id-new-uuid-xxx"));
}

#[test]
fn frame_multiple_headers_ordering() {
    let headers: Vec<Header> = (0..10)
        .map(|i| Header {
            key: format!("key-{i}"),
            value: format!("value-{i}"),
        })
        .collect();
    let frame = Frame {
        seq_id: 1,
        log_id: 1,
        service: 1,
        method: 1,
        headers: headers.clone(),
        payload_encoding: None,
        payload_type: None,
        payload: None,
        log_id_new: None,
    };
    let bytes = frame.encode_to_vec();
    let decoded: Frame = decode_contract(&bytes);
    assert_eq!(decoded.headers.len(), 10);
    for (i, h) in decoded.headers.iter().enumerate() {
        assert_eq!(h.key, format!("key-{i}"));
        assert_eq!(h.value, format!("value-{i}"));
    }
}
