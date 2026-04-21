//! 消息内容类型单元测试
//!
//! 测试各种消息内容类型（文本、富文本、图片、卡片等）的构建和序列化

use rstest::*;
use serde_json::{json, Value};
use open_lark::service::im::v1::message::*;

/// 文本消息测试
#[cfg(test)]
mod message_text_tests {
    use super::*;

    #[test]
    fn test_message_text_basic() {
        let msg = MessageText::new("Hello World");
        assert_eq!(msg.msg_type(), "text");
        assert_eq!(msg.content(), r#"{"text":"Hello World"}"#);
    }

    #[test]
    fn test_message_text_empty() {
        let msg = MessageText::new("");
        assert_eq!(msg.content(), r#"{"text":""}"#);
    }

    #[test]
    fn test_message_text_add_text() {
        let msg = MessageText::new("Hello")
            .add_text(" World")
            .add_text("!")
            .build();
        assert_eq!(msg.content(), r#"{"text":"Hello World!"}"#);
    }

    #[test]
    fn test_message_text_text_line() {
        let msg = MessageText::new("Line 1")
            .text_line("")
            .text_line("Line 2")
            .build();
        assert_eq!(msg.content(), r#"{"text":"Line 1\n\nLine 2\n"}"#);
    }

    #[test]
    fn test_message_text_line() {
        let msg = MessageText::new("First")
            .line()
            .add_text("Second")
            .build();
        assert_eq!(msg.content(), r#"{"text":"First\nSecond"}"#);
    }

    #[test]
    fn test_message_text_at_user() {
        let msg = MessageText::new("Hello ")
            .at_user("ou_123456")
            .build();
        assert_eq!(msg.content(), r#"{"text":"Hello <at user_id=\"ou_123456\"></at>"}"#);
    }

    #[test]
    fn test_message_text_at_all() {
        let msg = MessageText::new("Attention ")
            .at_all()
            .build();
        assert_eq!(msg.content(), r#"{"text":"Attention <at user_id=\"all\">name=\"全体成员\"</at>"}"#);
    }

    #[test]
    fn test_message_text_complex() {
        let msg = MessageText::new("欢迎新成员 ")
            .at_user("ou_newcomer")
            .text_line("!")
            .add_text("大家好，我是 ")
            .at_user("ou_myself")
            .line()
            .add_text("请多关照！")
            .build();

        let expected = r#"{"text":"欢迎新成员 <at user_id=\"ou_newcomer\"></at>!\n大家好，我是 <at user_id=\"ou_myself\"></at>\n请多关照！"}"#;
        assert_eq!(msg.content(), expected);
    }

    #[rstest]
    #[case("ou_123456")]
    #[case("u_123456")]
    #[case("un_123456")]
    #[case("user@example.com")]
    fn test_message_text_at_different_user_ids(#[case] user_id: &str) {
        let msg = MessageText::new("Hello ")
            .at_user(user_id)
            .build();
        
        let expected = format!(r#"{{"text":"Hello <at user_id=\"{}\"></at>"}}"#, user_id);
        assert_eq!(msg.content(), expected);
    }

    #[test]
    fn test_message_text_special_characters() {
        let msg = MessageText::new("Special chars: 😀🎉💯\n\t\"'\\");
        let content_json: Value = serde_json::from_str(&msg.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["text"], "Special chars: 😀🎉💯\n\t\"'\\");
    }

    #[test]
    fn test_message_text_chaining() {
        let msg = MessageText::new("")
            .add_text("Hello")
            .line()
            .at_user("user1")
            .add_text(" and ")
            .at_user("user2")
            .line()
            .at_all()
            .build();

        let expected = r#"{"text":"Hello\n<at user_id=\"user1\"></at> and <at user_id=\"user2\"></at>\n<at user_id=\"all\">name=\"全体成员\"</at>"}"#;
        assert_eq!(msg.content(), expected);
    }

    #[test]
    fn test_message_text_very_long() {
        let long_text = "A".repeat(10000);
        let msg = MessageText::new(&long_text);
        let content_json: Value = serde_json::from_str(&msg.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["text"], long_text);
    }
}

/// 富文本消息测试
#[cfg(test)]
mod message_post_tests {
    use super::*;

    #[test]
    fn test_message_post_basic() {
        let post = MessagePost::new("zh_cn")
            .title("测试标题");
        
        assert_eq!(post.msg_type(), "post");
        let content = post.content();
        let content_json: Value = serde_json::from_str(&content).expect("JSON 反序列化失败");
        assert_eq!(content_json["post"]["zh_cn"]["title"], "测试标题");
    }

    #[test]
    fn test_message_post_with_content() {
        let post = MessagePost::new("zh_cn")
            .title("富文本消息")
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("普通文本")),
                MessagePostNode::A(ANode::new("链接文本", "https://example.com")),
            ]);

        let content_json: Value = serde_json::from_str(&post.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["post"]["zh_cn"]["title"], "富文本消息");
        
        let content_array = &content_json["post"]["zh_cn"]["content"][0];
        assert_eq!(content_array[0]["tag"], "text");
        assert_eq!(content_array[0]["text"], "普通文本");
        assert_eq!(content_array[1]["tag"], "a");
        assert_eq!(content_array[1]["text"], "链接文本");
        assert_eq!(content_array[1]["href"], "https://example.com");
    }

    #[test]
    fn test_message_post_multiple_paragraphs() {
        let post = MessagePost::new("zh_cn")
            .title("多段落消息")
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("第一段")),
            ])
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("第二段")),
                MessagePostNode::At(AtNode::new("ou_user123")),
            ]);

        let content_json: Value = serde_json::from_str(&post.content()).expect("JSON 反序列化失败");
        let content_array = &content_json["post"]["zh_cn"]["content"];
        assert_eq!(content_array.as_array().unwrap().len(), 2);
        assert_eq!(content_array[0][0]["text"], "第一段");
        assert_eq!(content_array[1][0]["text"], "第二段");
        assert_eq!(content_array[1][1]["user_id"], "ou_user123");
    }

    #[test]
    fn test_message_post_all_node_types() {
        let post = MessagePost::new("en_us")
            .title("All Node Types")
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("Plain text")),
                MessagePostNode::A(ANode::new("Link", "https://example.com")),
                MessagePostNode::At(AtNode::new("user123")),
                MessagePostNode::Img(ImgNode::new("img_key_123")),
                MessagePostNode::Media(MediaNode::new("media_key_123", Some("thumb_key_123"))),
                MessagePostNode::Emotion(EmotionNode::new("SMILE")),
            ]);

        let content_json: Value = serde_json::from_str(&post.content()).expect("JSON 反序列化失败");
        let content_array = &content_json["post"]["en_us"]["content"][0];
        
        assert_eq!(content_array[0]["tag"], "text");
        assert_eq!(content_array[1]["tag"], "a");
        assert_eq!(content_array[2]["tag"], "at");
        assert_eq!(content_array[3]["tag"], "img");
        assert_eq!(content_array[4]["tag"], "media");
        assert_eq!(content_array[5]["tag"], "emotion");
        
        assert_eq!(content_array[3]["image_key"], "img_key_123");
        assert_eq!(content_array[4]["file_key"], "media_key_123");
        assert_eq!(content_array[4]["image_key"], "thumb_key_123");
        assert_eq!(content_array[5]["emoji_type"], "SMILE");
    }

    #[rstest]
    #[case("zh_cn", "中文标题")]
    #[case("en_us", "English Title")]
    #[case("ja_jp", "日本語タイトル")]
    fn test_message_post_different_languages(#[case] lang: &str, #[case] title: &str) {
        let post = MessagePost::new(lang).title(title);
        let content_json: Value = serde_json::from_str(&post.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["post"][lang]["title"], title);
    }
}

/// 富文本节点测试
#[cfg(test)]
mod post_node_tests {
    use super::*;

    #[test]
    fn test_text_node_basic() {
        let node = TextNode::new("测试文本");
        let serialized = serde_json::to_string(&MessagePostNode::Text(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["tag"], "text");
        assert_eq!(parsed["text"], "测试文本");
        assert!(parsed.get("un_escape").is_none());
        assert!(parsed.get("style").is_none());
    }

    #[test]
    fn test_text_node_with_styles() {
        let node = TextNode::new("样式文本")
            .style(vec!["bold", "underline", "italic"])
            .un_escape(true);
        
        let serialized = serde_json::to_string(&MessagePostNode::Text(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["text"], "样式文本");
        assert_eq!(parsed["un_escape"], true);
        assert_eq!(parsed["style"][0], "bold");
        assert_eq!(parsed["style"][1], "underline");
        assert_eq!(parsed["style"][2], "italic");
    }

    #[test]
    fn test_a_node_basic() {
        let node = ANode::new("飞书官网", "https://www.feishu.cn");
        let serialized = serde_json::to_string(&MessagePostNode::A(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["tag"], "a");
        assert_eq!(parsed["text"], "飞书官网");
        assert_eq!(parsed["href"], "https://www.feishu.cn");
        assert!(parsed.get("style").is_none());
    }

    #[test]
    fn test_a_node_with_style() {
        let node = ANode::new("链接", "https://example.com")
            .style(vec!["bold", "lineThrough"]);
        
        let serialized = serde_json::to_string(&MessagePostNode::A(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["style"][0], "bold");
        assert_eq!(parsed["style"][1], "lineThrough");
    }

    #[test]
    fn test_at_node_user() {
        let node = AtNode::new("ou_user123");
        let serialized = serde_json::to_string(&MessagePostNode::At(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["tag"], "at");
        assert_eq!(parsed["user_id"], "ou_user123");
        assert!(parsed.get("style").is_none());
    }

    #[test]
    fn test_at_node_all() {
        let node = AtNode::new("all")
            .style(vec!["bold"]);
        
        let serialized = serde_json::to_string(&MessagePostNode::At(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["user_id"], "all");
        assert_eq!(parsed["style"][0], "bold");
    }

    #[test]
    fn test_img_node() {
        let node = ImgNode::new("img_v2_test_key");
        let serialized = serde_json::to_string(&MessagePostNode::Img(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["tag"], "img");
        assert_eq!(parsed["image_key"], "img_v2_test_key");
    }

    #[test]
    fn test_media_node_with_thumbnail() {
        let node = MediaNode::new("file_v2_video_key", Some("thumb_key_123"));
        let serialized = serde_json::to_string(&MessagePostNode::Media(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["tag"], "media");
        assert_eq!(parsed["file_key"], "file_v2_video_key");
        assert_eq!(parsed["image_key"], "thumb_key_123");
    }

    #[test]
    fn test_media_node_without_thumbnail() {
        let node = MediaNode::new("file_v2_video_key", None);
        let serialized = serde_json::to_string(&MessagePostNode::Media(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["file_key"], "file_v2_video_key");
        assert!(parsed.get("image_key").is_none());
    }

    #[test]
    fn test_emotion_node() {
        let node = EmotionNode::new("LAUGH");
        let serialized = serde_json::to_string(&MessagePostNode::Emotion(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["tag"], "emotion");
        assert_eq!(parsed["emoji_type"], "LAUGH");
    }

    #[rstest]
    #[case("SMILE")]
    #[case("HEART")]
    #[case("THUMBSUP")]
    #[case("CLAP")]
    #[case("LAUGH")]
    fn test_emotion_node_different_types(#[case] emoji_type: &str) {
        let node = EmotionNode::new(emoji_type);
        let serialized = serde_json::to_string(&MessagePostNode::Emotion(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["emoji_type"], emoji_type);
    }

    #[rstest]
    #[case("bold")]
    #[case("underline")]
    #[case("lineThrough")]
    #[case("italic")]
    fn test_text_node_individual_styles(#[case] style: &str) {
        let node = TextNode::new("样式文本").style(vec![style]);
        let serialized = serde_json::to_string(&MessagePostNode::Text(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["style"][0], style);
    }
}

/// 图片消息测试
#[cfg(test)]
mod message_image_tests {
    use super::*;

    #[test]
    fn test_message_image_basic() {
        let img = MessageImage {
            image_key: "img_v2_test_key_123".to_string(),
        };
        
        assert_eq!(img.msg_type(), "image");
        assert_eq!(img.content(), r#"{"image_key":"img_v2_test_key_123"}"#);
    }

    #[test]
    fn test_message_image_serialization() {
        let img = MessageImage {
            image_key: "img_complex_key_456".to_string(),
        };
        
        let content_json: Value = serde_json::from_str(&img.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["image_key"], "img_complex_key_456");
    }

    #[rstest]
    #[case("img_v2_short")]
    #[case("img_v2_very_long_key_name_with_special_characters_123456789")]
    #[case("img_123")]
    fn test_message_image_different_keys(#[case] image_key: &str) {
        let img = MessageImage {
            image_key: image_key.to_string(),
        };
        
        let expected = format!(r#"{{"image_key":"{}"}}"#, image_key);
        assert_eq!(img.content(), expected);
    }

    #[test]
    fn test_message_image_empty_key() {
        let img = MessageImage {
            image_key: "".to_string(),
        };
        
        assert_eq!(img.content(), r#"{"image_key":""}"#);
    }
}

/// 卡片模板消息测试
#[cfg(test)]
mod message_card_template_tests {
    use super::*;

    #[test]
    fn test_message_card_template_basic() {
        let template_vars = json!({
            "title": "测试卡片",
            "content": "这是一个测试卡片",
            "user_name": "张三"
        });
        
        let card = MessageCardTemplate::new("ctp_test_123", template_vars.clone());
        
        assert_eq!(card.msg_type(), "interactive");
        
        let content_json: Value = serde_json::from_str(&card.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["type"], "template");
        assert_eq!(content_json["data"]["template_id"], "ctp_test_123");
        assert_eq!(content_json["data"]["template_variable"]["title"], "测试卡片");
        assert_eq!(content_json["data"]["template_variable"]["user_name"], "张三");
    }

    #[test]
    fn test_message_card_template_complex_variables() {
        let template_vars = json!({
            "header": {
                "title": "复杂卡片",
                "subtitle": "多层级数据"
            },
            "body": [
                {"type": "text", "content": "第一行"},
                {"type": "text", "content": "第二行"}
            ],
            "actions": [
                {"type": "button", "text": "确认", "value": "confirm"},
                {"type": "button", "text": "取消", "value": "cancel"}
            ]
        });
        
        let card = MessageCardTemplate::new("ctp_complex_456", template_vars);
        let content_json: Value = serde_json::from_str(&card.content()).expect("JSON 反序列化失败");
        
        assert_eq!(content_json["data"]["template_variable"]["header"]["title"], "复杂卡片");
        assert_eq!(content_json["data"]["template_variable"]["body"][0]["content"], "第一行");
        assert_eq!(content_json["data"]["template_variable"]["actions"][1]["text"], "取消");
    }

    #[test]
    fn test_message_card_template_empty_variables() {
        let empty_vars = json!({});
        let card = MessageCardTemplate::new("ctp_empty_789", empty_vars);
        
        let content_json: Value = serde_json::from_str(&card.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["data"]["template_variable"], json!({}));
    }

    #[rstest]
    #[case("ctp_123")]
    #[case("template_id_456")]
    #[case("very_long_template_id_name_789")]
    fn test_message_card_template_different_ids(#[case] template_id: &str) {
        let vars = json!({"test": "value"});
        let card = MessageCardTemplate::new(template_id, vars);
        
        let content_json: Value = serde_json::from_str(&card.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["data"]["template_id"], template_id);
    }

    #[test]
    fn test_message_card_template_string_template_id() {
        let template_id = String::from("string_template_id");
        let vars = json!({"key": "value"});
        let card = MessageCardTemplate::new(template_id.clone(), vars);
        
        let content_json: Value = serde_json::from_str(&card.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["data"]["template_id"], template_id);
    }
}

/// SendMessageTrait 特征测试
#[cfg(test)]
mod send_message_trait_tests {
    use super::*;

    #[test]
    fn test_send_message_trait_text() {
        let text_msg = MessageText::new("Test message");
        assert_eq!(text_msg.msg_type(), "text");
        assert!(text_msg.content().contains("Test message"));
    }

    #[test]
    fn test_send_message_trait_post() {
        let post_msg = MessagePost::new("zh_cn").title("Post title");
        assert_eq!(post_msg.msg_type(), "post");
        assert!(post_msg.content().contains("Post title"));
    }

    #[test]
    fn test_send_message_trait_image() {
        let image_msg = MessageImage {
            image_key: "img_test_key".to_string(),
        };
        assert_eq!(image_msg.msg_type(), "image");
        assert!(image_msg.content().contains("img_test_key"));
    }

    #[test]
    fn test_send_message_trait_card_template() {
        let card_msg = MessageCardTemplate::new("template_123", json!({"test": true}));
        assert_eq!(card_msg.msg_type(), "interactive");
        assert!(card_msg.content().contains("template"));
    }

    // 测试多态性
    fn test_message_polymorphism(msg: &dyn SendMessageTrait) -> (String, String) {
        (msg.msg_type(), msg.content())
    }

    #[test]
    fn test_send_message_trait_polymorphism() {
        let text_msg = MessageText::new("Polymorphism test");
        let (msg_type, content) = test_message_polymorphism(&text_msg);
        assert_eq!(msg_type, "text");
        assert!(content.contains("Polymorphism test"));
        
        let image_msg = MessageImage {
            image_key: "poly_img_key".to_string(),
        };
        let (msg_type, content) = test_message_polymorphism(&image_msg);
        assert_eq!(msg_type, "image");
        assert!(content.contains("poly_img_key"));
    }
}

/// 边界条件和错误处理测试
#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_text_message_very_long_content() {
        let long_text = "A".repeat(100000);
        let msg = MessageText::new(&long_text);
        let content_json: Value = serde_json::from_str(&msg.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["text"].as_str().unwrap().len(), 100000);
    }

    #[test]
    fn test_text_message_special_characters() {
        let special_text = "🚀 Hello\n\tWorld \"quotes\" 'single' \\ backslash 中文 😀";
        let msg = MessageText::new(special_text);
        let content_json: Value = serde_json::from_str(&msg.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["text"], special_text);
    }

    #[test]
    fn test_text_message_null_characters() {
        let text_with_null = "Hello\0World";
        let msg = MessageText::new(text_with_null);
        // 确保可以正常序列化
        let _ = msg.content();
    }

    #[test]
    fn test_post_message_empty_title() {
        let post = MessagePost::new("zh_cn").title("");
        let content_json: Value = serde_json::from_str(&post.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["post"]["zh_cn"]["title"], "");
    }

    #[test]
    fn test_post_message_no_content_nodes() {
        let post = MessagePost::new("en_us").title("Empty content");
        let content_json: Value = serde_json::from_str(&post.content()).expect("JSON 反序列化失败");
        assert_eq!(content_json["post"]["en_us"]["content"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_image_message_empty_key() {
        let img = MessageImage {
            image_key: "".to_string(),
        };
        assert_eq!(img.content(), r#"{"image_key":""}"#);
    }

    #[test]
    fn test_card_template_null_variables() {
        let card = MessageCardTemplate::new("template_null", json!(null));
        let content_json: Value = serde_json::from_str(&card.content()).expect("JSON 反序列化失败");
        assert!(content_json["data"]["template_variable"].is_null());
    }

    #[test]
    fn test_text_node_empty_text() {
        let node = TextNode::new("");
        let serialized = serde_json::to_string(&MessagePostNode::Text(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        assert_eq!(parsed["text"], "");
    }

    #[test]
    fn test_a_node_invalid_url() {
        let node = ANode::new("Invalid Link", "not-a-url");
        let serialized = serde_json::to_string(&MessagePostNode::A(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        assert_eq!(parsed["href"], "not-a-url");
    }

    #[test]
    fn test_at_node_empty_user_id() {
        let node = AtNode::new("");
        let serialized = serde_json::to_string(&MessagePostNode::At(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        assert_eq!(parsed["user_id"], "");
    }

    #[test]
    fn test_style_array_empty() {
        let node = TextNode::new("Test").style(vec![]);
        let serialized = serde_json::to_string(&MessagePostNode::Text(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        assert_eq!(parsed["style"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_very_long_urls() {
        let long_url = format!("https://example.com/{}", "a".repeat(2000));
        let node = ANode::new("Long URL", &long_url);
        let serialized = serde_json::to_string(&MessagePostNode::A(node)).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        assert_eq!(parsed["href"], long_url);
    }
}

/// 性能测试
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_text_message_build_performance() {
        let start = Instant::now();
        
        for i in 0..1000 {
            let _msg = MessageText::new("Base text")
                .add_text(&format!(" iteration {}", i))
                .at_user(&format!("user_{}", i))
                .line()
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_post_message_build_performance() {
        let start = Instant::now();
        
        for i in 0..500 {
            let _msg = MessagePost::new("zh_cn")
                .title(&format!("Title {}", i))
                .append_content(vec![
                    MessagePostNode::Text(TextNode::new(&format!("Text {}", i))),
                    MessagePostNode::At(AtNode::new(&format!("user_{}", i))),
                ]);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200);
    }

    #[test]
    fn test_message_serialization_performance() {
        let messages: Vec<Box<dyn SendMessageTrait>> = vec![
            Box::new(MessageText::new("Performance test message")),
            Box::new(MessageImage {
                image_key: "perf_img_key".to_string(),
            }),
            Box::new(MessagePost::new("zh_cn").title("Performance post")),
        ];
        
        let start = Instant::now();
        
        for _ in 0..1000 {
            for msg in &messages {
                let _ = msg.content();
            }
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 300);
    }
}

/// 属性测试（Property Testing）
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_text_message_any_string(text in "\\PC{0,1000}") {
            let msg = MessageText::new(&text);
            let content = msg.content();
            // 确保总能成功序列化
            let _: Value = serde_json::from_str(&content).expect("JSON 反序列化失败");
        }

        #[test] 
        fn test_image_message_any_key(key in "\\PC{1,100}") {
            let img = MessageImage {
                image_key: key.clone(),
            };
            let content = img.content();
            let parsed: Value = serde_json::from_str(&content).expect("JSON 反序列化失败");
            prop_assert_eq!(parsed["image_key"], key);
        }

        #[test]
        fn test_text_node_with_random_styles(
            text in "\\PC{1,50}",
            styles in prop::collection::vec("bold|underline|italic|lineThrough", 0..4)
        ) {
            let node = TextNode::new(&text).style(styles.clone());
            let serialized = serde_json::to_string(&MessagePostNode::Text(node)).unwrap();
            let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
            
            prop_assert_eq!(parsed["text"], text);
            if !styles.is_empty() {
                prop_assert_eq!(parsed["style"].as_array().unwrap().len(), styles.len());
            }
        }

        #[test]
        fn test_at_node_user_ids(user_id in "\\PC{1,50}") {
            let node = AtNode::new(&user_id);
            let serialized = serde_json::to_string(&MessagePostNode::At(node)).unwrap();
            let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
            
            prop_assert_eq!(parsed["user_id"], user_id);
        }
    }
}
