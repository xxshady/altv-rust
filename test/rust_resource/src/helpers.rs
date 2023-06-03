pub(crate) fn destroy_all_base_objects() {
    altv::base_object::all().iter().for_each(|v| {
        match v {
            altv::AnyBaseObject::Blip(o) => {
                o.destroy().unwrap();
            }
            altv::AnyBaseObject::Checkpoint(o) => {
                o.destroy().unwrap();
            }
            altv::AnyBaseObject::ColShape(o) => {
                o.destroy().unwrap();
            }
            altv::AnyBaseObject::ConnectionInfo(_) => {}
            altv::AnyBaseObject::Marker(o) => {
                o.destroy().unwrap();
            }
            altv::AnyBaseObject::NetworkObject(o) => {
                o.destroy().unwrap();
            }
            altv::AnyBaseObject::Vehicle(o) => {
                o.destroy().unwrap();
            }
            altv::AnyBaseObject::Player(_) => {}
            altv::AnyBaseObject::Ped(o) => {
                o.destroy().unwrap();
            }
            altv::AnyBaseObject::VirtualEntity(o) => {
                o.destroy().unwrap();
            }
            altv::AnyBaseObject::VirtualEntityGroup(_) => {}
            altv::AnyBaseObject::VoiceChannel(o) => {
                o.destroy().unwrap();
            }
        };
    })
}
