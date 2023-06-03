use super::BaseObjectId;

pub trait BaseObjectPoolFuncs<Container> {
    fn all() -> Vec<Container>;
    fn all_count() -> usize;
    fn get_by_id(id: BaseObjectId) -> Option<Container>;
}
