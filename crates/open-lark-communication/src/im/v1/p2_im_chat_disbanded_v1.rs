use serde::{Deserialize, Serialize },

use open_lark_core::event::{context::EventHeader, dispatcher::EventHandler};
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImChatDisbandedV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ImChatDisbandedV1Data,
}
pub(crate) struct P2ImChatDisbandedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatDisbandedV1) + 'static,
{
    f: F,
impl<F> EventHandler for P2ImChatDisbandedV1ProcessorImpl<F>
    F: Fn(P2ImChatDisbandedV1) + 'static + Sync + Send,
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let message: P2ImChatDisbandedV1 = serde_json::from_slice(payload)?;
        (self.f)(message);
        Ok(())
    }
