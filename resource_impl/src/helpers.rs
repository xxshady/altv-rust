#[macro_export]
macro_rules! get_entity_by_id {
    ($wrapper: path, $entity_id: ident) => {
        $crate::resource_impl::RESOURCE_IMPL_INSTANCE.with(|instance| {
            let instance = instance.borrow();
            let entities = instance.borrow_entities();
            let result = entities.get_by_id($entity_id);

            match result {
                Some(_wrapper @ $wrapper(entity)) => Some(Rc::clone(entity)),
                None | Some(_) => None,
            }
        })
    };
}
