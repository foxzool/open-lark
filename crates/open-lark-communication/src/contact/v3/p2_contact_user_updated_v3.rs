
use serde_json;use serde::{Deserialize, Serialize};
use open_lark_core::event::{context::EventHeader, dispatcher::EventHandler },
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ContactUserUpdatedV3 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ContactUserUpdatedV3Data,
}
pub(crate) struct P2ContactUserUpdatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactUserUpdatedV3) + 'static,
{
    f: F,
impl<F> EventHandler for P2ContactUserUpdatedV3ProcessorImpl<F>
    F: Fn(P2ContactUserUpdatedV3) + 'static + Sync + Send,
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2ContactUserUpdatedV3 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
