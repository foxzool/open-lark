use crate::core::error::LarkAPIError;
use reqwest::{multipart, RequestBuilder};
use serde_json::Value;

/// 专门构建 multipart/form-data 请求
pub struct MultipartBuilder;

impl MultipartBuilder {
    /// 构建包含文件的 multipart 表单
    pub fn build_multipart(
        req_builder: RequestBuilder,
        body: &[u8],
        file_data: &[u8],
    ) -> Result<RequestBuilder, LarkAPIError> {
        let json_value = serde_json::from_slice::<Value>(body)?;
        
        let form_obj = json_value
            .as_object()
            .ok_or_else(|| LarkAPIError::BadRequest("Invalid form data".to_string()))?;

        let mut form = multipart::Form::new();

        // 处理文件部分
        form = Self::add_file_part(form, form_obj, file_data)?;

        // 处理其他表单字段
        form = Self::add_form_fields(form, form_obj)?;

        // 设置编码
        form = form.percent_encode_noop();

        Ok(req_builder.multipart(form))
    }

    /// 添加文件部分到表单
    fn add_file_part(
        mut form: multipart::Form,
        form_obj: &serde_json::Map<String, Value>,
        file_data: &[u8],
    ) -> Result<multipart::Form, LarkAPIError> {
        let file_name = form_obj
            .get("file_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| LarkAPIError::BadRequest("Missing file_name in form data".to_string()))?
            .to_string();

        let file_part = multipart::Part::bytes(file_data.to_vec())
            .file_name(file_name);
        
        form = form.part("file", file_part);
        Ok(form)
    }

    /// 添加普通表单字段
    fn add_form_fields(
        mut form: multipart::Form,
        form_obj: &serde_json::Map<String, Value>,
    ) -> Result<multipart::Form, LarkAPIError> {
        for (key, value) in form_obj.iter() {
            // 跳过 file_name 字段和 null 值
            if key == "file_name" || value == &Value::Null {
                continue;
            }

            form = match value {
                Value::String(s) => form.text(key.to_string(), s.to_string()),
                Value::Number(n) => form.text(key.to_string(), n.to_string()),
                Value::Bool(b) => form.text(key.to_string(), b.to_string()),
                Value::Array(arr) => {
                    // 对于数组，转换为 JSON 字符串
                    form.text(key.to_string(), serde_json::to_string(arr)?)
                }
                Value::Object(obj) => {
                    // 对于对象，转换为 JSON 字符串
                    form.text(key.to_string(), serde_json::to_string(obj)?)
                }
                Value::Null => continue,
            };
        }

        Ok(form)
    }
}