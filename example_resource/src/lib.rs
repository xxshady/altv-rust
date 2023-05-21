use altv_sdk::ffi as sdk;

use altv::prelude::*;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    macro_rules! check_mvalue_serde {
        ($expected_type:ty, $expected_sdk_type:expr, $input:expr) => {{
            println!("checking type: {}", stringify!($expected_type));

            let mvalue = altv::__mvalue::to_mvalue(&$input).unwrap();
            let const_mvalue = mvalue.clone().into_const();

            let sdk_type =
                altv_sdk::MValueType::try_from(sdk::read_mvalue_type(const_mvalue.get())).unwrap();
            println!(
                "mvalue type: {sdk_type:?}, expected: {:?}",
                $expected_sdk_type
            );
            assert_eq!(sdk_type, $expected_sdk_type);

            let deserialized: $expected_type = altv::__mvalue::from_mvalue(mvalue).unwrap();
            println!("deserialized: {deserialized:?}");
        }};
    }

    // dbg!(f32::MAX);
    // dbg!(f32::MIN);
    // unsafe {
    //     check_mvalue_serde!(
    //         altv::Vector3,
    //         altv_sdk::MValueType::Vector3,
    //         altv::Vector3::new(f32::MAX, f32::MIN, f32::MIN)
    //     );
    // }

    dbg!(f32::MAX);
    dbg!(f32::MIN);
    unsafe {
        check_mvalue_serde!(
            altv::Vector2,
            altv_sdk::MValueType::Vector2,
            altv::Vector2::new(f32::MAX, f32::MIN)
        );
    }

    let args: altv::DynMValueArgs = &[&1, &true];
    let player = altv::Player::all()[0].clone();
    player.emit("test", args);

    let mut vec_args: Vec<altv::DynMValue> = vec![];

    for _ in 0..=5 {
        vec_args.push(&123);
    }

    player.emit("test", &vec_args);
}
