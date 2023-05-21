use std::collections::HashMap;

use altv_sdk::ffi as sdk;

use altv::{
    __serde::{Deserialize, Serialize},
    prelude::*,
};

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

            let deserialized: $expected_type = altv::__mvalue::from_mvalue(&const_mvalue).unwrap();
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

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(crate = "altv::__serde")]
    struct TestStruct {
        kek: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(crate = "altv::__serde")]
    struct TestTupleStruct(i32, bool);

    altv::events::on("test", |c| {
        dbg!(c);
        let first = altv::__mvalue::from_mvalue::<i32>(c.args.get(0).unwrap());
        let second = altv::__mvalue::from_mvalue::<TestStruct>(c.args.get(1).unwrap());
        let third = altv::__mvalue::from_mvalue::<TestTupleStruct>(c.args.get(2).unwrap());
        dbg!(first, second, third);

        Ok(())
    });

    let args: altv::DynMValueArgs = &[&1, &HashMap::from([("kek".to_string(), true)]), &(1, true)];
    altv::events::emit("test", args).unwrap();

    // TODO: test it
    altv::events::on_client("test", |c| {
        dbg!(c);
    });

    // let player = altv::Player::all()[0].clone();
    // player.emit("test", args);

    // let mut vec_args: Vec<altv::DynMValue> = vec![];

    // for _ in 0..=5 {
    //     vec_args.push(&123);
    // }

    // player.emit("test", &vec_args);
}
