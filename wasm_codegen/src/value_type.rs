type RustType = &'static str;

pub(crate) struct ParamType<'a>(pub(crate) &'a str);
pub(crate) struct ReturnType<'a>(pub(crate) &'a str);

macro_rules! value_type {
    ( $(
        $variant:ident
        rust: $rust:literal
        $( de: $rust_de:literal )?
        kind: $kind:ident
        repr: $repr:ident
        can_be_param: $can_be_param:expr
        ;
    )+ ) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum ValueType {
            $( $variant, )+
        }

        impl ValueType {
            pub fn kind(&self) -> ValueKind {
                match self {
                    $( Self::$variant => ValueKind::$kind, )+
                }
            }

            pub fn repr(&self) -> ValueRepr {
                match self {
                    $( Self::$variant => ValueRepr::$repr, )+
                }
            }

            pub fn rust(&self) -> RustType {
                match self {
                    $( Self::$variant => $rust, )+
                }
            }

            pub fn de(&self) -> Option<RustType> {
                match self {
                    $( $( Self::$variant => Some($rust_de), )? )+
                    _ => None,
                }
            }
        }

        // for parsing type names in wasm.interface
        impl<'a> From<ReturnType<'a>> for ValueType {
            fn from(value: ReturnType) -> Self {
                match value.0 {
                    $( stringify!($variant) => Self::$variant, )+
                    unknown => panic!("Unknown return type: {unknown:?}"),
                }
            }
        }

        impl<'a> From<ParamType<'a>> for ValueType {
            fn from(value: ParamType) -> Self {
                match value.0 {
                    $( stringify!($variant) if $can_be_param => Self::$variant, )+
                    unknown => panic!("Unknown OR unsupported param type: {unknown:?}"),
                }
            }
        }

        impl From<ValueType> for ValueKind {
            fn from(value: ValueType) -> Self {
                match value {
                    $( ValueType::$variant => ValueKind::$kind, )+
                }
            }
        }

        impl From<ValueType> for ValueRepr {
            fn from(value: ValueType) -> Self {
                match value {
                    $( ValueType::$variant => ValueRepr::$repr, )+
                }
            }
        }
    };
}

value_type!(
    // ---------- native ----------

    I8 rust: "i8" kind: Native repr: I32 can_be_param: true;
    I16 rust: "i16" kind: Native repr: I32 can_be_param: true;
    I32 rust: "i32" kind: Native repr: I32 can_be_param: true;
    U8 rust: "u8" kind: Native repr: U32 can_be_param: true;
    U16 rust: "u16" kind: Native repr: U32 can_be_param: true;
    U32 rust: "u32" kind: Native repr: U32 can_be_param: true;
    I64 rust: "i64" kind: Native repr: I64 can_be_param: true;
    U64 rust: "u64" kind: Native repr: U64 can_be_param: true;
    F32 rust: "f32" kind: Native repr: F32 can_be_param: true;
    F64 rust: "f64" kind: Native repr: F64 can_be_param: true;
    Bool rust: "bool" kind: Bool repr: I32 can_be_param: true;
    BaseObjectPtr rust: "altv_wasm_shared::BaseObjectPtr" kind: Native repr: U64 can_be_param: true;
    BaseObjectType rust: "altv_wasm_shared::BaseObjectTypeRaw" kind: Native repr: U32 can_be_param: true;

    // ---------- fat ptr ----------

    // String should only be used when in return of function
    String rust: "String" kind: String repr: FatPtr can_be_param: false;
    // Str should only be used when passing params to function
    // &String and not &str because it must be allocated in dynamic memory
    Str rust: "&String" de: "String" kind: String repr: FatPtr can_be_param: true;
    OptionBool rust: "Option<bool>" kind: FatPtr repr: FatPtr can_be_param: true;
    Vector3 rust: "altv_wasm_shared::Vector3" kind: FatPtr repr: FatPtr can_be_param: true;
);

#[derive(Debug)]
pub enum ValueKind {
    /// Custom type
    FatPtr,
    /// Small value that fits in wasm type
    /// (e.g. `bool` fits in `i32`)
    Native,
    /// Did not found better way for bool values :deadge:
    /// other than representing it as i32 and checking == 1
    Bool,
    /// Improved (de)serialization of strings
    String,
}

#[derive(Debug)]
pub enum ValueRepr {
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    FatPtr,
}

impl From<ValueRepr> for &str {
    fn from(value: ValueRepr) -> Self {
        match value {
            ValueRepr::I32 => "i32",
            ValueRepr::U32 => "u32",
            ValueRepr::I64 => "i64",
            ValueRepr::U64 => "u64",
            ValueRepr::F32 => "f32",
            ValueRepr::F64 => "f64",
            ValueRepr::FatPtr => "super::__shared::FatPtr",
        }
    }
}
