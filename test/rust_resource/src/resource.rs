use altv::events::{on_resource_stop, on_resource_start};

pub(crate) fn test_resource() {
    on_resource_start(|_| {
        dbg!(altv::Resource::current().valid());
        assert!(altv::Resource::current().valid());
    });

    on_resource_stop(|_| {
        dbg!(altv::Resource::current().valid());
        assert!(!altv::Resource::current().valid());
    });
}
