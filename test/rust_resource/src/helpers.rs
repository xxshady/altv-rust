pub(crate) fn destroy_all_base_objects() {
    altv::base_object::all().iter().for_each(|v| {
        match v {
            altv::AnyBaseObject::Blip(o) => {
                o.destroy().unwrap();
            }
            altv::AnyBaseObject::Checkpoint(o) => {
                o.destroy().unwrap();
            }
            altv::AnyBaseObject::ColShapeCircle(o) => {
                o.destroy().unwrap();
            }
            altv::AnyBaseObject::ColShapeCuboid(o) => {
                // o.destroy().unwrap();
            }
            altv::AnyBaseObject::ColShapeCylinder(o) => {
                // o.destroy().unwrap();
            }
            altv::AnyBaseObject::ColShapePoly(o) => {
                // o.destroy().unwrap();
            }
            altv::AnyBaseObject::ColShapeRect(o) => {
                // o.destroy().unwrap();
            }
            altv::AnyBaseObject::ColShapeSphere(o) => {
                // o.destroy().unwrap();
            }
            altv::AnyBaseObject::ConnectionInfo(_) => {}
            altv::AnyBaseObject::Marker(o) => {
                o.destroy().unwrap();
            }
            altv::AnyBaseObject::Object(o) => {
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
