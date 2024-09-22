use super::scope::Scope;

pub trait AttachedToScope<'scope> {
  fn attached_to_scope(&'scope self) -> &'scope dyn Scope;
}
