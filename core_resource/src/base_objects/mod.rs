mod base_impl;
pub(crate) mod extra_pools;
pub(crate) mod inherit_ptrs;
mod manager;
mod objects;
mod pool_funcs;

pub(crate) use base_impl::base_ptr::BasePtr;
pub use base_impl::inherit_ptrs::BaseObjectInheritPtrs;
pub use base_impl::valid::ValidBaseObject;
pub use base_impl::wrapper::BaseObjectWrapper;
pub(crate) use base_impl::wrapper::BaseObjectWrapperRc;
pub(crate) use base_impl::BaseObjectContainer;

pub(crate) use manager::BaseObjectManager;
pub(crate) use objects::Store;

pub use objects::AnyBaseObject;
pub(crate) use objects::PendingDestroyOrCreation;

pub(crate) use objects::blip;
pub(crate) use objects::checkpoint;
pub(crate) use objects::col_shape;
pub(crate) use objects::connection_info;
pub(crate) use objects::marker;
pub(crate) use objects::network_object;
pub(crate) use objects::ped;
pub(crate) use objects::player;
pub(crate) use objects::vehicle;
pub(crate) use objects::virtual_entity;
pub(crate) use objects::virtual_entity_group;
pub(crate) use objects::voice_channel;

pub use pool_funcs::BaseObjectPoolFuncs;

pub type BaseObjectId = u32;
