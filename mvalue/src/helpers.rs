#[macro_export]
macro_rules! serialize_simple {
    ($self:ident, $create_mvalue:expr) => {{
        use autocxx::prelude::*;
        $self.output = Some(MValue(unsafe { $create_mvalue }.within_unique_ptr()));
        Ok(())
    }};
}
