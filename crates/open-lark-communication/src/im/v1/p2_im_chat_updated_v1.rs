use serde::{Deserialize, Serialize },

use open_lark_core::event::{context::EventHeader, dispatcher::EventHandler};
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImChatUpdatedV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ImChatUpdatedV1Data,
}
pub(crate) struct P2ImChatUpdatedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatUpdatedV1) + 'static,
{
    f: F,
impl<F> EventHandler for P2ImChatUpdatedV1ProcessorImpl<F>
    F: Fn(P2ImChatUpdatedV1) + 'static + Sync + Send,
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let message: P2ImChatUpdatedV1 = serde_json::from_slice(payload)?;
        (self.f)(message);
        Ok(())
    }
