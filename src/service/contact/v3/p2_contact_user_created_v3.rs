use serde::{Deserialize, Serialize};
use openlark_core::event::EventHandler;

#[derive(Debug, Deserialize, Serialize)]
pub struct P2ContactUserCreatedV3;

#[derive(Debug, Deserialize, Serialize)]
pub struct P2ContactUserCreatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactUserCreatedV3) + 'static + Sync + Send,
{
    handler: F,
}

impl<F> P2ContactUserCreatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactUserCreatedV3) + 'static + Sync + Send,
{
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

impl<F> EventHandler for P2ContactUserCreatedV3ProcessorImpl<F>
where
    F: Fn(P2ContactUserCreatedV3) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2ContactUserCreatedV3 = serde_json::from_slice(payload)?;
        (self.handler)(event);
        Ok(())
    }
}