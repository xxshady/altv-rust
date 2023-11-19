use crate::parser::NativeType;

pub(crate) enum ValuePos {
    GuestParam,
    HostParam,
    GuestResult,
}

pub(crate) fn native_type_to_rust(native_type: NativeType, pos: ValuePos) -> &'static str {
    match native_type {
        NativeType::I32
        | NativeType::Any
        | NativeType::MemoryBuffer // TODO: implement memory buffer
        | NativeType::Interior
        | NativeType::Cam
        | NativeType::FireId
        | NativeType::Blip
        | NativeType::CarGenerator
        | NativeType::Group
        | NativeType::Weapon
        | NativeType::Texture
        | NativeType::TextureDict
        | NativeType::CoverPoint
        | NativeType::TaskSequence
        | NativeType::ColourIndex
        | NativeType::Sphere
        | NativeType::Pickup
        => "i32",

        NativeType::Hash
        | NativeType::Entity
        | NativeType::Object
        | NativeType::Ped
        | NativeType::Player
        | NativeType::ScrHandle
        | NativeType::Vehicle
        | NativeType::Train
         => "u32",

        NativeType::F32 => "f32",
        NativeType::String => {
            match pos {
                ValuePos::GuestParam => "Option<&String>",
                ValuePos::HostParam => "Option<String>",
                ValuePos::GuestResult => "Option<String>",
            }
        },
        NativeType::Vector3 => {
            // TODO: should it also be option?
            match pos {
                ValuePos::GuestParam => "Option<&shared::Vector3>",
                ValuePos::HostParam => "Option<shared::Vector3>",
                ValuePos::GuestResult => "shared::Vector3",
            }
        },
        NativeType::Void => "()",
        NativeType::Boolean => "bool",
    }
}

pub(crate) const NATIVE_RETURN_IDENT: &str = "native_return";

pub(crate) fn internal_name_of(native_name: &str) -> String {
    format!("native_{native_name}")
}
