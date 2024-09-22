use super::base_object_type::BaseObjectType;

pub trait AsBaseObjectType: Copy {
  fn as_base_object_type() -> BaseObjectType;
}
