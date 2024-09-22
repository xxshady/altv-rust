use altv::mvalue::{from_mvalue, to_mvalue};

macro_rules! to_and_from {
    (@internal
        $(
            $deserialize_type:ty: $( $serialize:expr $( => $eq:expr )? ),+ ;
        )+
    ) => {
        $( $(
            altv::log!(
                "ser: {} expected: {}",
                stringify!($serialize),
                stringify!($deserialize_type)
            );

            let serialized_value = $serialize;

            let mvalue = to_mvalue(&serialized_value).unwrap();
            let deserialized: $deserialize_type = from_mvalue(&mvalue.into_const()).unwrap();
            altv::log!(
                "de:  {:?}, expected: {:?}",
                deserialized,
                serialized_value
            );

            #[allow(clippy::redundant_closure_call)]
            $( $eq(deserialized, serialized_value); )?
        )+ )+
    };

    (@assert_eq $(
        $deserialize_type:ty: $( $serialize:expr ),+ ;
    )+) => {
        to_and_from!(@internal
            $(
                $deserialize_type: $( $serialize => |a, b| { assert_eq!(a, b) } ),+ ;
            )+
        );
    };

    (@custom_eq $(
        @eq $custom_eq:expr, $deserialize_type:ty: $( $serialize:expr ),+ ;
    )+) => {
        to_and_from!(@internal
            $(
                $deserialize_type: $( $serialize => $custom_eq ),+ ;
            )+
        );
    };
}

pub(crate) fn test_mvalue() {
  #[derive(altv::serde::Serialize, altv::serde::Deserialize, Debug, PartialEq)]
  #[serde(crate = "altv::serde")]
  #[repr(u32)]
  enum TestEnum {
    Unit,
    Newtype(i32),
    Tuple(i32, bool, String),
    Struct { a: i32, b: bool },
  }

  #[derive(altv::serde::Serialize, altv::serde::Deserialize, Debug, PartialEq)]
  #[serde(crate = "altv::serde")]
  #[serde(untagged)]
  #[repr(u32)]
  enum TestEnumUntagged {
    Unit,
    Newtype(i32),
    Tuple(i32, bool, String),
    Struct { a: i32, b: bool },
  }

  to_and_from!(@assert_eq
      bool: true, false;

      i8: i8::MAX, i8::MIN;
      i16: i16::MAX, i16::MIN;
      i32: i32::MAX, i32::MIN;
      i64: i64::MAX, i64::MIN;

      u8: u8::MAX, u8::MIN;
      u16: u16::MAX, u16::MIN;
      u32: u32::MAX, u32::MIN;
      u64: u64::MAX, u64::MIN;

      f32: f32::MAX, f32::MIN;
      f64: f64::MAX, f64::MIN;

      Option<bool>: Some(true), Some(false), Option::<bool>::None;

      String: "123456789AWjdkrOtignmzxcnb;l^)(%$*&@#^!$#)(%$?>0Да".to_string(), "123456789AWjdkrOtignmzxcnb;l^)(%$*&@#^!$#)(%$?>0Да";

      Vec<i32>: vec![1, 2, 3];
      Vec<bool>: vec![true, false, true];
      Vec<String>: vec!["123456789AWjdkrOtignmzxcnb;l^)(%$*&@#^!$#)(%$?>0Да".to_string()];

      altv::ByteBuf: altv::ByteBuf::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, u8::MAX]);

      altv::Rgba: altv::Rgba::new(u8::MAX, u8::MAX, u8::MAX, u8::MAX);

      altv::Vector3: altv::Vector3::new(f32::MAX, f32::MIN, 123);
      altv::Vector2: altv::Vector2::new(f32::MAX, f32::MIN);
      altv::Vector2: altv::Vector2::new(f32::MAX, 123);

      altv::ColShapeContainer: altv::ColShape::new_circle(0, 10.0);

      TestEnum: TestEnum::Unit;
      TestEnum: TestEnum::Newtype(123);
      TestEnum: TestEnum::Tuple(
          i32::MAX,
          true,
          String::from("wdwdwddwdwdwddwdwdwddwdwdwddwdwdwddwdwdwddwdwdwdd"),
      );
      TestEnum: TestEnum::Struct { a: 123, b: true };

      TestEnumUntagged: TestEnumUntagged::Unit;
      TestEnumUntagged: TestEnumUntagged::Newtype(123);
      TestEnumUntagged: TestEnumUntagged::Tuple(
          i32::MAX,
          true,
          String::from("wdwdwddwdwdwddwdwdwddwdwdwddwdwdwddwdwdwddwdwdwdd"),
      );
      TestEnumUntagged: TestEnumUntagged::Struct { a: 123, b: true };

      // TODO:
      // altv::AnyBaseObject: altv::VoiceChannel::new_spatial(0.0).unwrap();
  );

  // unit variant
  {
    let s = to_mvalue(&(
      0, // TestEnum::Unit
      (),
    ))
    .unwrap();
    let d: TestEnum = from_mvalue(&s.into_const()).unwrap();
    assert_eq!(d, TestEnum::Unit);
  }
  {
    let s = to_mvalue(
      &0, // TestEnum::Unit
    )
    .unwrap();
    let d: TestEnum = from_mvalue(&s.into_const()).unwrap();
    assert_eq!(d, TestEnum::Unit);
  }

  // newtype variant
  {
    let s = to_mvalue(&(
      1, // TestEnum::Newtype
      123,
    ))
    .unwrap();
    let d: TestEnum = from_mvalue(&s.into_const()).unwrap();
    assert_eq!(d, TestEnum::Newtype(123));
  }

  // tuple variant
  {
    let s = to_mvalue(&(
      2, // TestEnum::Tuple
      (
        i32::MAX,
        true,
        String::from("wdwdwddwdwdwddwdwdwddwdwdwddwdwdwddwdwdwddwdwdwdd"),
      ),
    ))
    .unwrap();
    let d: TestEnum = from_mvalue(&s.into_const()).unwrap();
    assert_eq!(
      d,
      TestEnum::Tuple(
        i32::MAX,
        true,
        String::from("wdwdwddwdwdwddwdwdwddwdwdwddwdwdwddwdwdwddwdwdwdd"),
      )
    );
  }

  // struct variant
  {
    #[derive(altv::serde::Serialize)]
    #[serde(crate = "altv::serde")]
    struct TestStruct {
      a: i32,
      b: bool,
    }

    let s = to_mvalue(&(
      3, // TestEnum::Struct
      TestStruct { a: 123, b: true },
    ))
    .unwrap();
    let d: TestEnum = from_mvalue(&s.into_const()).unwrap();
    assert_eq!(d, TestEnum::Struct { a: 123, b: true });
  }

  // to_and_from!(@custom_eq
  //     @eq |a: altv::Vector3, b: altv::Vector3| a.x() == b.x(), altv::Vector3: altv::Vector3::new(f32::MAX, f32::MIN, 123);
  //     @eq |a: altv::Vector2, b: altv::Vector2| a.x() == b.x(), altv::Vector2: altv::Vector2::new(f32::MAX, f32::MIN);
  //     @eq |a: altv::Vector2, b: altv::Vector2| a.x() == b.x(), altv::Vector2: altv::Vector2::new(f32::MAX, 123);
  // );
}
