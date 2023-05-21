use autocxx::prelude::UniquePtr;

pub(crate) type RawConstMValue = UniquePtr<altv_sdk::ffi::ConstMValueWrapper>;
pub(crate) type RawMutMValue = UniquePtr<altv_sdk::ffi::MValueMutWrapper>;
