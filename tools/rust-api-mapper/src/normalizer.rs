//! URL参数标准化器
//!
//! 智能处理URL参数标准化和映射，建立变量名到URL参数的准确映射。
//! 支持多种命名约定（snake_case、camelCase等）和飞书API专用映射。

use regex::Regex;
use std::collections::HashMap;
use anyhow::{Result, Context};

/// URL参数标准化器
pub struct ParameterNormalizer {
    /// 参数名映射表（snake_case -> camelCase）
    parameter_mappings: HashMap<String, String>,
    /// base_url清理模式
    base_url_patterns: Vec<Regex>,
    /// 参数占位符模式
    placeholder_pattern: Regex,
}

impl ParameterNormalizer {
    /// 创建新的参数标准化器
    pub fn new() -> Result<Self> {
        let mut parameter_mappings = HashMap::new();

        // 飞书API专用映射
        let mappings = [
            // 电子表格相关
            ("spreadsheet_token", "spreadsheetToken"),
            ("sheet_id", "sheetId"),
            ("sheet_token", "sheetToken"),
            ("range_id", "rangeId"),
            ("cell_id", "cellId"),
            ("row_id", "rowId"),
            ("column_id", "columnId"),
            ("image_key", "imageKey"),

            // 用户相关
            ("user_id", "userId"),
            ("user_id_type", "userIdType"),
            ("union_id", "unionId"),
            ("open_id", "openId"),

            // 组织架构相关
            ("department_id", "departmentId"),
            ("department_id_type", "departmentIdType"),
            ("user_group_id", "userGroupId"),
            ("department_key", "departmentKey"),
            ("department_key_type", "departmentKeyType"),

            // 应用相关
            ("app_id", "appId"),
            ("app_ticket", "appTicket"),
            ("tenant_key", "tenantKey"),

            // 文档相关
            ("document_id", "documentId"),
            ("version_id", "versionId"),
            ("file_key", "fileKey"),
            ("block_id", "blockId"),
            ("root_id", "rootId"),
            ("drive_id", "driveId"),

            // 审批相关
            ("approval_id", "approvalId"),
            ("definition_id", "definitionId"),
            ("instance_id", "instanceId"),
            ("instance_code", "instanceCode"),
            ("node_id", "nodeId"),
            ("task_id", "taskId"),
            ("template_id", "templateId"),

            // 日历相关
            ("schedule_id", "scheduleId"),
            ("calendar_id", "calendarId"),
            ("meeting_id", "meetingId"),
            ("event_id", "eventId"),

            // 通讯相关
            ("chat_id", "chatId"),
            ("message_id", "messageId"),

            // 通用模式
            ("uuid", "uuid"),
            ("id", "id"),
            ("token", "token"),
            ("key", "key"),
            ("code", "code"),
            ("type", "type"),
            ("name", "name"),
            ("title", "title"),
            ("desc", "desc"),
            ("status", "status"),
            ("state", "state"),
            ("level", "level"),
            ("sort", "sort"),
            ("order", "order"),
            ("page", "page"),
            ("size", "size"),
            ("limit", "limit"),
            ("offset", "offset"),
            ("count", "count"),
            ("total", "total"),

            // 时间相关
            ("start_time", "startTime"),
            ("end_time", "endTime"),
            ("create_time", "createTime"),
            ("update_time", "updateTime"),
            ("delete_time", "deleteTime"),
            ("expire_time", "expireTime"),
            ("access_token", "accessToken"),
            ("refresh_token", "refreshToken"),

            // 客户端相关
            ("client_key", "clientKey"),
            ("client_secret", "clientSecret"),
        ];

        for (snake, camel) in mappings {
            parameter_mappings.insert(snake.to_string(), camel.to_string());
        }

        let base_url_patterns = vec![
            Regex::new(r"\{\s*base_url\s*\}\s*/")?,
            Regex::new(r"^\{\s*\}\s*/")?,
            Regex::new(r"\{\s*config\.base_url\s*\}\s*/")?,
            Regex::new(r"\{\s*self\.config\.base_url\s*\}\s*/")?,
        ];

        Ok(Self {
            parameter_mappings,
            base_url_patterns,
            placeholder_pattern: Regex::new(r"\{\s*([^}]*)\s*\}")?,
        })
    }

    /// 智能标准化URL，支持变量名映射
    pub fn normalize_url_with_variables(&self, url: &str, variables: &[String]) -> Result<String> {
        // 首先清理base_url占位符
        let mut url = self.clean_base_url_placeholders(url)?;

        // 处理现有的:parameter格式（API列表中的格式）
        url = self.normalize_colon_parameters(&url)?;

        // 如果有变量信息，进行智能映射
        if !variables.is_empty() {
            url = self.replace_with_variable_mapping(&url, variables)?;
        } else {
            url = self.replace_generic_parameters(&url)?;
        }

        // 清理格式
        url = self.cleanup_url_format(&url);

        Ok(url)
    }

    /// 标准化:parameter格式为统一格式
    fn normalize_colon_parameters(&self, url: &str) -> Result<String> {
        let colon_param_pattern = Regex::new(r":(\w+)")
            .with_context(|| "Failed to compile colon parameter pattern")?;

        let result = colon_param_pattern.replace_all(url, |caps: &regex::Captures| {
            if let Some(param_match) = caps.get(1) {
                let param_name = param_match.as_str();
                // 将:parameter转换为标准格式，保持参数名但用统一格式
                format!(":{}", param_name)
            } else {
                ":parameter".to_string()
            }
        });

        Ok(result.to_string())
    }

    /// 清理base_url占位符
    fn clean_base_url_placeholders(&self, url: &str) -> Result<String> {
        let mut cleaned = url.to_string();
        for pattern in &self.base_url_patterns {
            cleaned = pattern.replace_all(&cleaned, "/").to_string();
        }
        Ok(cleaned)
    }

    /// 使用变量名进行智能参数替换
    fn replace_with_variable_mapping(&self, url: &str, variables: &[String]) -> Result<String> {
        let mut result = url.to_string();
        let mut variable_iter = variables.iter().peekable();

        // 使用正则表达式替换所有参数占位符
        result = self.placeholder_pattern.replace_all(&result, |caps: &regex::Captures| {
            let placeholder_content = caps.get(1).map_or("", |m| m.as_str()).trim();

            if placeholder_content.is_empty() {
                // 空占位符，使用下一个变量
                if let Some(var) = variable_iter.next() {
                    let clean_var = self.clean_variable_name(var);
                    let param_name = self.get_parameter_name(&clean_var);
                    format!(":{}", param_name)
                } else {
                    ":parameter".to_string()
                }
            } else {
                // 有具体内容，直接处理
                let clean_var = self.clean_variable_name(placeholder_content);
                let param_name = self.get_parameter_name(&clean_var);
                format!(":{}", param_name)
            }
        }).to_string();

        Ok(result)
    }

    /// 通用参数替换（当没有变量信息时）
    fn replace_generic_parameters(&self, url: &str) -> Result<String> {
        let mut result = url.to_string();

        // 优先处理已知的命名参数
        for (snake, camel) in &self.parameter_mappings {
            let pattern = format!(r"\{{\s*{}\s*\}}", regex::escape(snake));
            let regex = Regex::new(&pattern)
                .with_context(|| format!("Failed to compile pattern for {}", snake))?;
            result = regex.replace_all(&result, format!(":{}", camel)).to_string();
        }

        // 处理其他命名参数
        result = self.placeholder_pattern.replace_all(&result, |caps: &regex::Captures| {
            if let Some(param_content) = caps.get(1) {
                let param_name = self.to_parameter_name(param_content.as_str());
                format!(":{}", param_name)
            } else {
                ":parameter".to_string()
            }
        }).to_string();

        Ok(result)
    }

    /// 清理变量名，移除常见的访问模式
    fn clean_variable_name(&self, var_name: &str) -> String {
        let mut clean = var_name.to_string();

        // 移除对象访问前缀
        let prefixes = [
            r"request\.",
            r"self\.",
            r"config\.",
            r"&request\.",
            r"&self\.",
            r"&config\.",
        ];

        for prefix in &prefixes {
            if let Ok(regex) = Regex::new(&format!("^{}", prefix)) {
                clean = regex.replace(&clean, "").to_string();
            }
        }

        // 移除引用符号
        clean = clean.trim_start_matches('&').trim().to_string();

        clean
    }

    /// 根据变量名获取参数名
    fn get_parameter_name(&self, var_name: &str) -> String {
        // 首先检查预定义映射
        if let Some(camel) = self.parameter_mappings.get(var_name) {
            return camel.clone();
        }

        // 检查常见模式
        if let Some(caps) = Regex::new(r"(\w+)_id").unwrap().captures(var_name) {
            return format!("{}Id", self.snake_to_camel(&caps[1]));
        }
        if let Some(caps) = Regex::new(r"(\w+)_type").unwrap().captures(var_name) {
            return format!("{}Type", self.snake_to_camel(&caps[1]));
        }
        if let Some(caps) = Regex::new(r"(\w+)_token").unwrap().captures(var_name) {
            return format!("{}Token", self.snake_to_camel(&caps[1]));
        }
        if let Some(caps) = Regex::new(r"(\w+)_key").unwrap().captures(var_name) {
            return format!("{}Key", self.snake_to_camel(&caps[1]));
        }
        if let Some(caps) = Regex::new(r"(\w+)_(at|time)").unwrap().captures(var_name) {
            let base = &caps[1];
            let suffix = &caps[2];
            return format!("{}{}", self.snake_to_camel(base), suffix.to_ascii_uppercase());
        }

        // 默认转换为camelCase
        self.snake_to_camel(var_name)
    }

    /// 将任意参数名转换为标准格式
    fn to_parameter_name(&self, param_name: &str) -> String {
        let clean = param_name.trim_matches('"').trim_matches('\'').trim();

        // 检查预定义映射
        if let Some(camel) = self.parameter_mappings.get(clean) {
            return camel.clone();
        }

        // 默认处理
        if clean.contains('_') {
            self.snake_to_camel(clean)
        } else {
            clean.to_string()
        }
    }

    /// 将snake_case转换为camelCase
    fn snake_to_camel(&self, snake_str: &str) -> String {
        let components: Vec<&str> = snake_str.split('_').collect();
        if components.is_empty() {
            return snake_str.to_string();
        }

        let mut result = components[0].to_string();
        for component in &components[1..] {
            if !component.is_empty() {
                let mut chars: Vec<char> = component.chars().collect();
                if let Some(first_char) = chars.get_mut(0) {
                    *first_char = first_char.to_ascii_uppercase();
                }
                result.push_str(&chars.into_iter().collect::<String>());
            }
        }

        result
    }

    
    /// 清理URL格式
    fn cleanup_url_format(&self, url: &str) -> String {
        let mut cleaned = url.to_string();

        // 移除重复斜杠
        while cleaned.contains("//") {
            cleaned = cleaned.replace("//", "/");
        }

        // 移除末尾斜杠（除非是根路径）
        if cleaned.ends_with('/') && cleaned.len() > 1 {
            cleaned.pop();
        }

        // 确保以/开头
        if !cleaned.starts_with('/') {
            cleaned = format!("/{}", cleaned);
        }

        cleaned
    }

    /// 创建反向映射（参数名 -> 变量名建议）
    pub fn create_reverse_mapping(&self, normalized_url: &str) -> HashMap<String, String> {
        let mut reverse_map = HashMap::new();

        // 从URL中提取参数
        let param_regex = Regex::new(r":(\w+)").unwrap();
        for caps in param_regex.captures_iter(normalized_url) {
            if let Some(param_match) = caps.get(1) {
                let param = param_match.as_str();

                // 查找对应的snake_case形式
                let snake_name = self.camel_to_snake(param);

                // 检查是否有更好的变量名建议
                let final_snake = self.parameter_mappings
                    .iter()
                    .find(|(_, camel)| **camel == param)
                    .map(|(snake, _)| snake.clone())
                    .unwrap_or(snake_name);

                reverse_map.insert(param.to_string(), final_snake);
            }
        }

        reverse_map
    }

    /// 将camelCase转换为snake_case
    fn camel_to_snake(&self, camel_str: &str) -> String {
        let mut result = String::new();
        let mut prev_was_upper = false;

        for (i, ch) in camel_str.chars().enumerate() {
            if ch.is_ascii_uppercase() {
                if i != 0 && !prev_was_upper {
                    result.push('_');
                }
                result.push(ch.to_ascii_lowercase());
                prev_was_upper = true;
            } else {
                result.push(ch);
                prev_was_upper = false;
            }
        }

        result
    }

    /// 验证两个URL是否匹配
    pub fn validate_url_mapping(&self, code_url: &str, api_url: &str) -> Result<(bool, String)> {
        // 标准化两个URL
        let normalized_code = self.normalize_url_with_variables(code_url, &[])?;
        let normalized_api = self.normalize_url_with_variables(api_url, &[])?;

        // 直接比较
        if normalized_code == normalized_api {
            return Ok((true, "完全匹配".to_string()));
        }

        // 检查参数模式是否匹配
        let code_pattern = self.create_url_pattern(&normalized_code);
        let api_pattern = self.create_url_pattern(&normalized_api);

        if code_pattern == api_pattern {
            return Ok((true, "参数模式匹配".to_string()));
        }

        // 检查是否是子路径关系
        if self.is_subpath_match(&normalized_code, &normalized_api) {
            return Ok((true, "子路径匹配".to_string()));
        }

        Ok((false, format!("不匹配: {} vs {}", normalized_code, normalized_api)))
    }

    /// 创建URL模式，将参数替换为通配符
    fn create_url_pattern(&self, url: &str) -> String {
        let param_regex = Regex::new(r":\w+").unwrap();
        param_regex.replace_all(url, "{param}").to_string()
    }

    /// 检查是否是子路径匹配
    fn is_subpath_match(&self, url1: &str, url2: &str) -> bool {
        let parts1: Vec<&str> = url1.trim_start_matches('/').split('/').collect();
        let parts2: Vec<&str> = url2.trim_start_matches('/').split('/').collect();

        if parts1.len() != parts2.len() {
            return false;
        }

        for (p1, p2) in parts1.iter().zip(parts2.iter()) {
            if p1.starts_with(':') && p2.starts_with(':') {
                continue;
            } else if p1.starts_with(':') || p2.starts_with(':') {
                continue;
            } else if p1 != p2 {
                return false;
            }
        }

        true
    }
}

impl Default for ParameterNormalizer {
    fn default() -> Self {
        Self::new().expect("Failed to create ParameterNormalizer")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_url_with_variables() {
        let normalizer = ParameterNormalizer::new().unwrap();

        // 测试电子表格令牌映射
        let url = "/open-apis/sheets/v2/spreadsheets/{}/merge_cells";
        let variables = vec!["spreadsheet_token".to_string()];
        let result = normalizer.normalize_url_with_variables(url, &variables).unwrap();
        assert_eq!(result, "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/merge_cells");

        // 测试无参数URL
        let url = "/open-apis/sheets/v3/spreadsheets";
        let result = normalizer.normalize_url_with_variables(url, &[]).unwrap();
        assert_eq!(result, "/open-apis/sheets/v3/spreadsheets");
    }

    #[test]
    fn test_snake_to_camel() {
        let normalizer = ParameterNormalizer::new().unwrap();

        assert_eq!(normalizer.snake_to_camel("spreadsheet_token"), "spreadsheetToken");
        assert_eq!(normalizer.snake_to_camel("user_id"), "userId");
        assert_eq!(normalizer.snake_to_camel("simple"), "simple");
    }

    #[test]
    fn test_camel_to_snake() {
        let normalizer = ParameterNormalizer::new().unwrap();

        assert_eq!(normalizer.camel_to_snake("spreadsheetToken"), "spreadsheet_token");
        assert_eq!(normalizer.camel_to_snake("userId"), "user_id");
        assert_eq!(normalizer.camel_to_snake("simple"), "simple");
    }

    #[test]
    fn test_clean_variable_name() {
        let normalizer = ParameterNormalizer::new().unwrap();

        assert_eq!(normalizer.clean_variable_name("request.spreadsheet_token"), "spreadsheet_token");
        assert_eq!(normalizer.clean_variable_name("&self.user_id"), "user_id");
        assert_eq!(normalizer.clean_variable_name("config.base_url"), "base_url");
    }

    #[test]
    fn test_url_validation() {
        let normalizer = ParameterNormalizer::new().unwrap();

        let code_url = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/merge_cells";
        let api_url = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/merge_cells";

        let (matches, reason) = normalizer.validate_url_mapping(code_url, api_url).unwrap();
        assert!(matches);
        assert_eq!(reason, "参数模式匹配");
    }
}