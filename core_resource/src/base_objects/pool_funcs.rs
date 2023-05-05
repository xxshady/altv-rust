pub trait BaseObjectPoolFuncs<Container> {
    fn all() -> Vec<Container>;
    fn all_count() -> usize;
}
