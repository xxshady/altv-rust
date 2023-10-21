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

pub(crate) use objects::{
    blip, checkpoint, marker, object, ped, player, vehicle, virtual_entity, virtual_entity_group,
    voice_channel, connection_info, col_shape_circle, col_shape_cuboid, col_shape_cylinder,
    col_shape_poly, col_shape_rect, col_shape_sphere,
};

pub use pool_funcs::BaseObjectPoolFuncs;

pub type BaseObjectId = u32;
