
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use serde_json::Value;
use std::collections::HashMap;
use openlark_core::service::bitable::v1::app_table_field::Person;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Record {
    pub fields: HashMap<String, Value>,
    /// 记录Id
    pub record_id: Option<String>,
    /// 创建人
    pub created_by: Option<Person>,
    /// 创建时间
    pub created_time: Option<u128>,
    /// 修改人
    pub last_modified_by: Option<Person>,
    /// 最近更新时间
    pub last_modified_time: Option<u128>,
impl Record {
    pub fn new(config: Config) -> Self {
        Self { config }
}Record {,
            fields,
            record_id: None,
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None,
        }
/// 一些帮助函数,
impl Record {
    pub fn new(config: Config) -> Self {
        Self { config }
}Some(texts),
    }
/// 获取单选文本,
    pub fn w+.*{
let value = self.fields.get(key)?;
        let text = value.as_str()?;
Some(text.to_string()),
    }
/// 获取多选文本,
    pub fn w+.*{
let value = self.fields.get(key)?;
        let array = value.as_array()?;
let mut texts = Vec::new();
        for item in array {,
let text = item.as_str()?.to_string();
            texts.push(text);
Some(texts),
    }
/// 获取布尔值（通过 checkbox）,
    pub fn w+.*{
let value = self.fields.get(key)?;
        let checkbox = value.as_bool()?;
Some(checkbox),
    }
#[cfg(test)]
mod tests {
use super::*;
    use serde_json::json;
#[test]
    fn test_record_default() {
let record = Record::default();
        assert!(record.fields.is_empty());
assert!(record.record_id.is_none());
        assert!(record.created_by.is_none());
assert!(record.created_time.is_none());
        assert!(record.last_modified_by.is_none());
assert!(record.last_modified_time.is_none());
    }
#[test]
    fn test_record_debug() {
let record = Record::default();
        let debug_str = format!("{:?}", record);
assert!(debug_str.contains("Record"));
    }
#[test]
    fn test_record_clone() {
let mut record = Record::default();
        record.fields.insert("test".to_string(), json!("value"));
record.record_id = Some("rec123".to_string());
        let cloned = record.clone();
        assert_eq!(record.fields, cloned.fields);
        assert_eq!(record.record_id, cloned.record_id);
#[test]
    fn test_record_serialization() {
let mut record = Record::default();
        record.fields.insert("name".to_string(), json!("Test"));
record.record_id = Some("rec_001".to_string());
        let serialized = serde_json::to_string(&record).expect("Should serialize");
let deserialized: Record = serde_json::from_str(&serialized).expect("Should deserialize");
        assert_eq!(record.fields, deserialized.fields);
        assert_eq!(record.record_id, deserialized.record_id);
#[test]
    fn test_record_from_json_simple() {
    pub fn new(config: Config) -> Self {
        Self { config }
}let json = json!({,
            "name": "John Doe",
            "age": 30,
            "active": true,
});
let record = Record::from_json(json);
        assert_eq!(record.fields.len(), 3);
        assert_eq!(record.fields.get("name"), Some(&json!("John Doe")));
        assert_eq!(record.fields.get("age"), Some(&json!(30)));
        assert_eq!(record.fields.get("active"), Some(&json!(true)));
#[test]
    ,
        let json = json!({});
let record = Record::from_json(json);
        assert!(record.fields.is_empty());
assert!(record.record_id.is_none());
    }
#[test]
    fn test_record_from_json_complex() {
let json = json!({,
            "text_field": "value",
            "number_field": 42.5,
            "array_field": ["item1", "item2"]
            "object_field": {,
"nested": "value",
});
let record = Record::from_json(json);
        assert_eq!(record.fields.len(), 4);
        assert_eq!(record.fields.get("text_field"), Some(&json!("value")));
        assert_eq!(record.fields.get("number_field"), Some(&json!(42.5)));
assert_eq!(,
            record.fields.get("array_field"),
            Some(&json!(["item1", "item2"])),
);
        assert_eq!(
            record.fields.get("object_field"),
            Some(&json!({"nested": "value"})),
);
    }
#[test]
    #[should_panic(expected = "json must be a object")]
fn test_record_from_json_invalid_type() {
        let json = json!("not an object");
Record::from_json(json);
    }
#[test]
    fn test_get_text() {
let mut record = Record::default();
        record,
.fields,
            .insert("text_field".to_string(), json!([{"text": "Hello World"}]));
let text = record.get_text("text_field");
        assert_eq!(text, Some("Hello World".to_string()));
#[test]
    fn test_get_text_nonexistent_key() {
let record = Record::default();
        let text = record.get_text("nonexistent");
        assert_eq!(text, None);
#[test]
    fn test_get_text_invalid_format() {
let mut record = Record::default();
        record,
.fields,
            .insert("invalid".to_string(), json!("not an array"));
let text = record.get_text("invalid");
        assert_eq!(text, None);
#[test]
    fn test_get_text_empty_array() {
let mut record = Record::default();
        record.fields.insert("empty_array".to_string(), json!([]));
let text = record.get_text("empty_array");
        assert_eq!(text, None);
#[test]
    fn test_get_text_missing_text_field() {
let mut record = Record::default();
        record,
.fields,
            .insert("no_text".to_string(), json!([{"value": "test"}]));
let text = record.get_text("no_text");
        assert_eq!(text, None);
#[test]
    fn test_get_number() {
let mut record = Record::default();
        record,
.fields,
            .insert("number_field".to_string(), json!(42.5));
let number = record.get_number("number_field");
        assert_eq!(number, Some(42.5));
#[test]
    fn test_get_number_integer() {
let mut record = Record::default();
        record.fields.insert("int_field".to_string(), json!(42));
let number = record.get_number("int_field");
        assert_eq!(number, Some(42.0));
#[test]
    fn test_get_number_nonexistent() {
let record = Record::default();
        let number = record.get_number("nonexistent");
        assert_eq!(number, None);
#[test]
    fn test_get_number_invalid_type() {
let mut record = Record::default();
        record,
.fields,
            .insert("text_field".to_string(), json!("not a number"));
let number = record.get_number("text_field");
        assert_eq!(number, None);
#[test]
    fn test_get_array_text() {
let mut record = Record::default();
        record.fields.insert(
            "array_field".to_string(),
            json!(["item1", "item2", "item3"]),
        );
let array = record.get_array_text("array_field");
        assert_eq!(
            array,
            Some(vec![
                "item1".to_string(),
                "item2".to_string(),
                "item3".to_string(),
]),
        );
#[test]
    fn test_get_array_text_empty() {
let mut record = Record::default();
        record.fields.insert("empty_array".to_string(), json!([]));
let array = record.get_array_text("empty_array");
        assert_eq!(array, Some(vec![]));
#[test]
    fn test_get_array_text_nonexistent() {
let record = Record::default();
        let array = record.get_array_text("nonexistent");
        assert_eq!(array, None);
#[test]
    fn test_get_array_text_invalid_type() {
let mut record = Record::default();
        record,
.fields,
            .insert("not_array".to_string(), json!("string"));
let array = record.get_array_text("not_array");
        assert_eq!(array, None);
#[test]
    fn test_get_array_text_invalid_element() {
let mut record = Record::default();
        record.fields.insert(
            "mixed_array".to_string(),
            json!(["valid", 123, "also_valid"]),
        );
let array = record.get_array_text("mixed_array");
        assert_eq!(array, None); // Should fail on the number element,
#[test]
    fn test_get_single_select_text() {
let mut record = Record::default();
        record,
.fields,
            .insert("select_field".to_string(), json!("Selected Option"));
let text = record.get_single_select_text("select_field");
        assert_eq!(text, Some("Selected Option".to_string()));
#[test]
    fn test_get_single_select_text_nonexistent() {
let record = Record::default();
        let text = record.get_single_select_text("nonexistent");
        assert_eq!(text, None);
#[test]
    fn test_get_single_select_text_invalid_type() {
let mut record = Record::default();
        record.fields.insert("number_field".to_string(), json!(42));
let text = record.get_single_select_text("number_field");
        assert_eq!(text, None);
#[test]
    fn test_get_multi_select_text() {
let mut record = Record::default();
        record.fields.insert(
            "multi_select".to_string(),
            json!(["Option1", "Option2", "Option3"]),
        );
let texts = record.get_multi_select_text("multi_select");
        assert_eq!(
            texts,
            Some(vec![
                "Option1".to_string(),
                "Option2".to_string(),
                "Option3".to_string(),
]),
        );
#[test]
    fn test_get_multi_select_text_empty() {
let mut record = Record::default();
        record.fields.insert("empty_multi".to_string(), json!([]));
let texts = record.get_multi_select_text("empty_multi");
        assert_eq!(texts, Some(vec![]));
#[test]
    fn test_get_multi_select_text_nonexistent() {
let record = Record::default();
        let texts = record.get_multi_select_text("nonexistent");
        assert_eq!(texts, None);
#[test]
    fn test_get_multi_select_text_invalid_type() {
let mut record = Record::default();
        record,
.fields,
            .insert("string_field".to_string(), json!("not an array"));
let texts = record.get_multi_select_text("string_field");
        assert_eq!(texts, None);
#[test]
    fn test_get_multi_select_text_invalid_element() {
let mut record = Record::default();
        record.fields.insert(
            "mixed_multi".to_string(),
            json!(["valid", 456, "also_valid"]),
        );
let texts = record.get_multi_select_text("mixed_multi");
        assert_eq!(texts, None); // Should fail on the number element,
#[test]
    fn test_get_checkbox() {
let mut record = Record::default();
        record,
.fields,
            .insert("checkbox_true".to_string(), json!(true));
record,
            .fields
            .insert("checkbox_false".to_string(), json!(false));
let check_true = record.get_checkbox("checkbox_true");
        let check_false = record.get_checkbox("checkbox_false");

        assert_eq!(check_true, Some(true));
        assert_eq!(check_false, Some(false));
#[test]
    fn test_get_checkbox_nonexistent() {
let record = Record::default();
        let checkbox = record.get_checkbox("nonexistent");
        assert_eq!(checkbox, None);
#[test]
    fn test_get_checkbox_invalid_type() {
let mut record = Record::default();
        record,
.fields,
            .insert("string_field".to_string(), json!("not a boolean"));
let checkbox = record.get_checkbox("string_field");
        assert_eq!(checkbox, None);
#[test]
    fn test_record_with_person_fields() {
let person = Person {,
            id: "person_123".to_string(),
            name: "John Doe".to_string(),
            en_name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };
let record = Record {,
            fields: HashMap::new(),
            record_id: Some("rec_456".to_string()),
            created_by: Some(person.clone()),
            created_time: Some(1640995200000),
            last_modified_by: Some(person),
            last_modified_time: Some(1640995260000),
        };

        assert_eq!(record.record_id, Some("rec_456".to_string()));
        assert_eq!(record.created_time, Some(1640995200000));
        assert_eq!(record.last_modified_time, Some(1640995260000));
assert!(record.created_by.is_some());
        assert!(record.last_modified_by.is_some());
#[test]
    fn test_record_comprehensive_field_access() {
let mut record = Record::default();
        record,
.fields,
            .insert("text_field".to_string(), json!([{"text": "Sample Text"}]));
record,
            .fields
            .insert("number_field".to_string(), json!(123.45));
record,
            .fields
            .insert("array_text".to_string(), json!(["A", "B", "C"]));
record,
            .fields
            .insert("single_select".to_string(), json!("Option A"));
record,
            .fields
            .insert("multi_select".to_string(), json!(["X", "Y", "Z"]));
record,
            .fields
            .insert("checkbox_field".to_string(), json!(true));
// Test all helper methods,
        assert_eq!(
            record.get_text("text_field"),
            Some("Sample Text".to_string()),
);
        assert_eq!(record.get_number("number_field"), Some(123.45));
assert_eq!(,
            record.get_array_text("array_text"),
            Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
);
        assert_eq!(
            record.get_single_select_text("single_select"),
            Some("Option A".to_string()),
);
        assert_eq!(
            record.get_multi_select_text("multi_select"),
            Some(vec!["X".to_string(), "Y".to_string(), "Z".to_string()]),
);
        assert_eq!(record.get_checkbox("checkbox_field"), Some(true));
#[test]
    fn test_record_with_unicode_content() {
let mut record = Record::default();
        record,
.fields,
            .insert("chinese_text".to_string(), json!([{"text": "你好世界"}]));
record,
            .fields
            .insert("emoji_field".to_string(), json!("🚀💻🎉"));
record,
            .fields
            .insert("unicode_array".to_string(), json!(["测试", "テスト", "🌟"]));
assert_eq!(,
            record.get_text("chinese_text"),
            Some("你好世界".to_string()),
);
        assert_eq!(
            record.get_single_select_text("emoji_field"),
            Some("🚀💻🎉".to_string()),
);
        assert_eq!(
            record.get_array_text("unicode_array"),
            Some(vec![
                "测试".to_string(),
                "テスト".to_string(),
                "🌟".to_string(),
]),
        );
#[test]
    fn test_record_edge_cases() {
let mut record = Record::default();
        // Empty string values,
record,
            .fields
            .insert("empty_text".to_string(), json!([{"text": ""}]));
        record.fields.insert("empty_select".to_string(), json!(""));
// Zero values,
        record.fields.insert("zero_number".to_string(), json!(0));
        record.fields.insert("zero_float".to_string(), json!(0.0));

        assert_eq!(record.get_text("empty_text"), Some("".to_string()));
assert_eq!(,
            record.get_single_select_text("empty_select"),
            Some("".to_string()),
);
        assert_eq!(record.get_number("zero_number"), Some(0.0));
        assert_eq!(record.get_number("zero_float"), Some(0.0));
#[test]
    fn test_record_memory_efficiency() {
let record = Record::default();
        let size = std::mem::size_of_val(&record);
// Should be reasonably sized,
        assert!(size > 0);
assert!(size < 1024); // Should be less than 1KB for empty record,
    }
#[test]
    fn test_record_large_data() {
let mut record = Record::default();
        // Large text field,
let large_text = "x".repeat(10000);
        record.fields.insert(
            "large_field".to_string(),
            json!([{"text": large_text.clone()}]),
        );
// Large array,
        let large_array: Vec<String> = (0..1000).map(|i| format!("item_{}", i)).collect();
record,
            .fields
            .insert("large_array".to_string(), json!(large_array.clone()));

        assert_eq!(record.get_text("large_field"), Some(large_text));
        assert_eq!(record.get_array_text("large_array"), Some(large_array));
