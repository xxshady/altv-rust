use crate::{
    sdk::{self, CCancellableEvent},
    VoidResult,
};

#[derive(Debug)]
pub struct CancellableEvent {
    cancellable_ptr: *const sdk::alt::CCancellableEvent,
}

impl CancellableEvent {
    pub fn new(base_event: altv_sdk::CEventPtr) -> Self {
        let ptr = unsafe { sdk::events::to_cancellable(base_event) };
        assert!(!ptr.is_null());

        Self {
            cancellable_ptr: ptr,
        }
    }

    pub fn cancel(&self) -> VoidResult {
        if unsafe { CCancellableEvent::WasCancelled(self.cancellable_ptr) } {
            anyhow::bail!("Event cannot be cancelled multiple times")
        } else {
            unsafe { CCancellableEvent::Cancel(self.cancellable_ptr) }
            Ok(())
        }
    }
}
