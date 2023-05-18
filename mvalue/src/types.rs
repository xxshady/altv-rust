use autocxx::prelude::UniquePtr;

pub(crate) type RawMValue = UniquePtr<altv_sdk::ffi::MValueWrapper>;
pub(crate) type RawMutMValue = UniquePtr<altv_sdk::ffi::MValueMutWrapper>;
