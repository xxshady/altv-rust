// auto-generated from build.rs

use primitive_enum::primitive_enum;
primitive_enum! { PlayerConnectDeniedReason u8 ;
    WrongVersion,
    WrongBranch,
    DebugNotAllowed,
    WrongPassword,
    WrongCdnUrl,
}