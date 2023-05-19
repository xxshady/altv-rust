use altv::prelude::*;

use altv_sdk::ffi as sdk;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    // std::env::set_var("RUST_BACKTRACE", "full");

    use autocxx::prelude::*;
    macro_rules! check_mvalue_serde {
        ($expected_type:ty, $input:expr) => {{
            println!("checking type: {}", stringify!($expected_type));

            dbg!();
            let mvalue = altv::__mvalue::to_mvalue(&$input).unwrap();
            dbg!();
            let const_mvalue =
                sdk::convert_mvalue_mut_wrapper_to_const(mvalue.0).within_unique_ptr();
            dbg!();
            println!(
                "mvalue type: {:?}",
                altv_sdk::MValueType::try_from(sdk::read_mvalue_type(&const_mvalue))
            );

            // let deserialized: $expected_type = mvalue::from_mvalue(const_mvalue).unwrap();
            // println!("deserialized: {deserialized:?}");
        }};
    }

    let veh = altv::Vehicle::new("sultan2", 0, 0).unwrap();
    let blip = altv::Blip::new_point(0);
    // veh.destroy().unwrap();

    unsafe {
        dbg!();
        let mvalue = altv::__mvalue::to_mvalue(&blip).unwrap();
        dbg!();
        let const_mvalue = sdk::convert_mvalue_mut_wrapper_to_const(mvalue.0).within_unique_ptr();
        dbg!();
        println!(
            "mvalue type: {:?}",
            altv_sdk::MValueType::try_from(sdk::read_mvalue_type(&const_mvalue))
        );

        // veh.destroy().unwrap();

        let deserialized: altv::BlipContainer = altv::__mvalue::from_mvalue(const_mvalue).unwrap();
        deserialized.set_alpha(100).unwrap();
        println!(
            "deserialized: {deserialized:?} {:?} {:?}",
            deserialized.alpha().unwrap(),
            deserialized.blip_type().unwrap()
        );
    }
}
