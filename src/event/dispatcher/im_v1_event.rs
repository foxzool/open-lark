use crate::{
    event::dispatcher::EventDispatcherHandlerBuilder,
    service::im::v1::p2_im_message_receive_v1::{
        P2ImMessageReceiveV1, P2ImMessageReceiveV1ProcessorImpl,
    },
};

impl EventDispatcherHandlerBuilder {
    pub fn register_p2_im_message_receive_v1<F>(mut self, f: F) -> Self
    where
        F: Fn(P2ImMessageReceiveV1) + 'static + Sync + Send,
    {
        let key = "p2.im.message.receive_v1".to_string();
        if self.processor_map.contains_key(&key) {
            panic!("processor already registered, type: {}", key);
        }
        let processor = P2ImMessageReceiveV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        self
    }
}
