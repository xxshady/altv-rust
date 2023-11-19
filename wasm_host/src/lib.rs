pub mod gen;

impl<T> std::fmt::Debug for gen::exports::Exports<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Exports {{}}")
    }
}
