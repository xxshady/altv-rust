use altv::prelude::*;

use altv_sdk::ffi as sdk;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    use autocxx::prelude::*;
    macro_rules! check_mvalue_serde {
        ($expected_type:ty, $expected_sdk_type:expr, $input:expr) => {{
            println!("checking type: {}", stringify!($expected_type));

            let mvalue = altv::__mvalue::to_mvalue(&$input).unwrap();
            let const_mvalue =
                sdk::convert_mvalue_mut_wrapper_to_const(mvalue.0).within_unique_ptr();

            let sdk_type =
                altv_sdk::MValueType::try_from(sdk::read_mvalue_type(&const_mvalue)).unwrap();
            println!(
                "mvalue type: {sdk_type:?}, expected: {:?}",
                $expected_sdk_type
            );
            assert_eq!(sdk_type, $expected_sdk_type);

            let deserialized: $expected_type = altv::__mvalue::from_mvalue(const_mvalue).unwrap();
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

    // let byte_array = altv::Bytes::new(&[1, 2, 3]);
    // unsafe { check_mvalue_serde!(altv::ByteBuf, altv_sdk::MValueType::ByteArray, byte_array) }

    // let rgba = altv::RGBA::new(1, 2, 3, 4);
    // unsafe { check_mvalue_serde!(altv::RGBA, altv_sdk::MValueType::Rgba, rgba) }

    // let veh = altv::Vehicle::new("sultan2", 0, 0).unwrap();
    // let blip = altv::Blip::new_point(0);
    // // veh.destroy().unwrap();

    // unsafe {
    //     dbg!();
    //     let mvalue = altv::__mvalue::to_mvalue(&blip).unwrap();
    //     dbg!();
    //     let const_mvalue = sdk::convert_mvalue_mut_wrapper_to_const(mvalue.0).within_unique_ptr();
    //     dbg!();
    //     println!(
    //         "mvalue type: {:?}",
    //         altv_sdk::MValueType::try_from(sdk::read_mvalue_type(&const_mvalue))
    //     );

    //     // veh.destroy().unwrap();

    //     let deserialized: altv::BlipContainer = altv::__mvalue::from_mvalue(const_mvalue).unwrap();
    //     deserialized.set_alpha(100).unwrap();
    //     println!(
    //         "deserialized: {deserialized:?} {:?} {:?}",
    //         deserialized.alpha().unwrap(),
    //         deserialized.blip_type().unwrap()
    //     );
    // }
}
