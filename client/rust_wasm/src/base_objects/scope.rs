use super::{
  as_base_object_type::AsBaseObjectType, handle::BaseObjectHandle, instance::BaseObject,
  scoped_instance::ScopedBaseObject,
};

/// Scope where "borrowed" base objects (for example Player or Vehicle) are guaranteed to be valid
///
/// # Example
///
/// ```
/// // `context` implements Scope
/// altv::set_timeout(|context| {
///   // `streamed_in` needs scope to guarantee that
///   // returned players will not be used after destroy
///   let players = altv::Player::streamed_in(context);
///
///   altv::spawn_local(async {
///     dbg!(players); // error, `context` is dead at this moment
///   });
/// });
/// ```
pub trait Scope {}

pub fn attach_base_object<'scope, T: AsBaseObjectType>(
  scope: &'scope dyn Scope,
  base_object: BaseObject<T>,
) -> ScopedBaseObject<'scope, T> {
  ScopedBaseObject::new(scope, base_object)
}

#[derive(Default)]
pub struct DefaultScope {}

impl Scope for DefaultScope {}

/// Immediately called closure with new scope
pub fn new_scope<R>(use_scope: impl for<'scope> FnOnce(&'scope DefaultScope) -> R) -> R {
  use_scope(&DefaultScope::default())
}
