#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Chats Service Shim,
//!,
//! Temporary shim implementation for ChatsService during migration period.,
use crate::core::config::Config;
/// 群聊服务,
pub struct ChatsService {
    pub config: Config,
impl ChatsService {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl Clone for ChatsService {,
    fn clone(&self) -> Self {
Self {
            config: self.config.clone()}
impl std::fmt::Debug for ChatsService {,
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {,
f.debug_struct()
            .field()
.finish()}
