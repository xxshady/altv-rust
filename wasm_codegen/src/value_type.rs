type RustType = &'static str;

macro_rules! value_type {
    ( $( $variant:ident rust: $rust:literal kind: $kind:ident repr: $repr:ident; )+ ) => {
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
        }

        // for parsing type names in wasm.interface
        impl From<&str> for ValueType {
            fn from(value: &str) -> Self {
                match value {
                    $( stringify!($variant) => Self::$variant, )+
                    unknown => panic!("Unknown value type: {unknown:?}"),
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
    String rust: "String" kind: FatPtr repr: FatPtr;
    I8 rust: "i8" kind: Native repr: I32;
    I16 rust: "i16" kind: Native repr: I32;
    I32 rust: "i32" kind: Native repr: I32;
    U8 rust: "u8" kind: Native repr: U32;
    U16 rust: "u16" kind: Native repr: U32;
    U32 rust: "u32" kind: Native repr: U32;
    I64 rust: "i64" kind: Native repr: I64;
    U64 rust: "u64" kind: Native repr: U64;
    F32 rust: "f32" kind: Native repr: F32;
    F64 rust: "f64" kind: Native repr: F64;
    Bool rust: "bool" kind: Bool repr: I32;
    OptionBool rust: "Option<bool>" kind: FatPtr repr: FatPtr;
    BaseObjectPtr rust: "altv_wasm_shared::BaseObjectPtr" kind: Native repr: U64;
    BaseObjectType rust: "altv_wasm_shared::BaseObjectTypeRaw" kind: Native repr: U32;
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
