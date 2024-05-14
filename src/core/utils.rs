use crate::core::constants::VERSION;

pub fn user_agent() -> String {
    format!("open-lark/{}", VERSION)
}
