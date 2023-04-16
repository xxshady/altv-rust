use crate::{sdk, VoidResult};

#[derive(Debug)]
pub struct CancellableEvent {
    base_event: altv_sdk::CEventPtr,
}

impl CancellableEvent {
    pub fn new(base_event: altv_sdk::CEventPtr) -> Self {
        Self {
            base_event
        }
    }

    pub fn cancel(&self) -> VoidResult {
        if unsafe { sdk::CEvent::WasCancelled(self.base_event) } {
            anyhow::bail!("Event cannot be cancelled multiple times")
        } else {
            unsafe { sdk::CEvent::Cancel(self.base_event) }
            Ok(())
        }
    }
}
