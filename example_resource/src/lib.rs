use altv::{
    prelude::*,
    serde::{Deserialize, Serialize},
};
#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "0");

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(crate = "altv::serde")]
    struct NewType {
        a: i32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(crate = "altv::serde")]
    struct NewType2 {
        a: bool,
    }

    let mvalue = altv::mvalue::to_mvalue(&altv::VoiceChannel::new_spatial(0.0)?)?;

    let value: altv::VoiceChannelContainer = altv::mvalue::from_mvalue(&mvalue.into_const())?;
    dbg!(value);

    Ok(())
}
