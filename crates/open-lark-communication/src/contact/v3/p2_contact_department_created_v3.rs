use serde::{Deserialize, Serialize },
use serde_json;
use open_lark_core::event::{context::EventHeader, dispatcher::EventHandler};
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ContactDepartmentCreatedV3 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ContactDepartmentCreatedV3Data,
}
pub(crate) struct P2ContactDepartmentCreatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactDepartmentCreatedV3) + 'static,
{
    f: F,
impl<F> EventHandler for P2ContactDepartmentCreatedV3ProcessorImpl<F>
    F: Fn(P2ContactDepartmentCreatedV3) + 'static + Sync + Send,
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2ContactDepartmentCreatedV3 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
