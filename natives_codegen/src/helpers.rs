use crate::parser::NativeType;

pub(crate) enum ValuePos {
    GuestParam,
    HostParam,
    GuestResult,
}

pub(crate) fn native_type_to_rust(
    native_type: NativeType,
    pos: ValuePos,
    r#ref: bool,
) -> &'static str {
    match native_type {
        NativeType::I32
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
        | NativeType::Pickup => "i32",

        NativeType::Hash
        | NativeType::Entity
        | NativeType::Object
        | NativeType::Ped
        | NativeType::Player
        | NativeType::ScrHandle
        | NativeType::Vehicle
        | NativeType::Train => "u32",

        NativeType::F32 => "f32",
        NativeType::String => match pos {
            ValuePos::GuestParam => "Option<&String>",
            ValuePos::HostParam => "Option<String>",
            ValuePos::GuestResult => "Option<String>",
        },
        NativeType::Vector3 => {
            // TODO: should it also be option?
            match pos {
                ValuePos::GuestParam => "Option<&shared::Vector3>",
                ValuePos::HostParam => "Option<shared::Vector3>",
                ValuePos::GuestResult => "shared::Vector3",
            }
        }
        NativeType::Void => "()",
        NativeType::Boolean => "bool",

        // Any ref is used for memory buffers
        NativeType::MemoryBuffer => unreachable!(),
        NativeType::Any => {
            if r#ref {
                "shared::MemoryBufferId"
            } else {
                "i32"
            }
        }
    }
}

pub(crate) const NATIVE_RETURN_IDENT: &str = "native_return";

pub(crate) fn internal_name_of(native_name: &str) -> String {
    format!("native_{native_name}")
}

pub(crate) fn ascii_camel_or_pascal_to_snake_case(str: &str) -> String {
    let mut current_part = String::new();
    let mut parts = vec![];
    let mut add_part = |current_part: &mut String| {
        parts.push(std::mem::take(current_part));
    };

    for c in str.chars() {
        if (c.is_uppercase() || c == '_') && current_part.len() > 1 {
            if c != '_' {
                current_part.push('_');
            }
            add_part(&mut current_part);
        }
        current_part.push(c);
    }
    add_part(&mut current_part);

    parts
        .into_iter()
        .map(|part| part.to_lowercase())
        .collect::<Vec<_>>()
        .join("")
}

pub(crate) fn ascii_snake_to_pascal_case(str: &str) -> String {
    let mut current_part = String::new();
    let mut parts = vec![];
    let mut add_part = |current_part: &mut String| {
        parts.push(std::mem::take(current_part));
    };

    for c in str.chars() {
        if (c.is_uppercase() || c == '_') && current_part.len() > 1 {
            add_part(&mut current_part);
        }

        if c == '_' {
            continue;
        }
        current_part.push(c);
    }
    add_part(&mut current_part);

    parts
        .into_iter()
        .map(|part| {
            let mut chars = part.chars();
            let first_char = chars.next().unwrap().to_ascii_uppercase();
            format!("{first_char}{}", chars.collect::<String>())
        })
        .collect::<Vec<_>>()
        .join("")
}
