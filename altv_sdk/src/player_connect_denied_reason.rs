// auto-generated from build.rs

#![allow(non_camel_case_types)]
use primitive_enum::primitive_enum;
primitive_enum! { PlayerConnectDeniedReason u8 ;
    WrongVersion,
    WrongBranch,
    DebugNotAllowed,
    WrongPassword,
    WrongCdnUrl,
}