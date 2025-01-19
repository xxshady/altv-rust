// mod helpers;

// mod mvalue;
// use std::time::Duration;

// use mvalue::test_mvalue;

// mod base_object_pool_funcs;
// use base_object_pool_funcs::test_base_object_pool_funcs;

// mod base_object_funcs;
// use base_object_funcs::test_base_object_funcs;

// mod weapon_model_info;
// use weapon_model_info::test_weapon_model_info;

// mod timers;
// use timers::test_timers;

// mod script_events;
// use script_events::test_script_events;

// mod blip;
// use blip::test_blip;

// mod core_funcs;
// use core_funcs::test_core_funcs;

// mod vehicle_model_info;
// use vehicle_model_info::test_vehicle_model_info;

// mod resource;
// use resource::test_resource;

// mod events;
// use events::test_events;

// mod ped;
// use ped::test_ped;

// mod error_backtrace;
// use error_backtrace::test_error_backtrace;

// mod closest_entities;
// use closest_entities::{test_closest_entities, CLOSEST_ENTITIES_TEST_TIMER};

// mod metadata_events;
// use metadata_events::test_metadata_events;

use std::{future::Future, time::Duration};

use futures::task::LocalSpawnExt;

mod asyncc;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
  let fut = async {
    println!("hello world");
    let time = std::time::Instant::now();
    asyncc::wait(Duration::from_millis(500)).await;
    dbg!(time.elapsed());
    asyncc::wait(Duration::from_millis(500)).await;
    dbg!(time.elapsed());
  };

  altv::set_interval(asyncc::on_every_tick, 0);
  let mut local_pool = futures::executor::LocalPool::new();
  local_pool.spawner().spawn_local(fut).unwrap();

  println!("waiting");
  loop {
    // asyncc::on_every_tick();
    altv::process_timers();

    if local_pool.try_run_one() {
      break;
    }
  }
  println!("end");

  // let rt = tokio::runtime::Builder::new_current_thread()
  //   .enable_all()
  //   .build()
  //   .unwrap();

  // let local_rt = tokio::runtime::LocalRuntime::new();

  // rt.block_on(async {
  //   println!("Hello world");
  //   tokio::time::sleep(Duration::from_millis(500)).await;
  //   println!("after");
  // });

  // altv::events::on_console_command(|_| {
  //   tokio::task::spawn_local(async {
  //     tokio::time::sleep(Duration::from_millis(500)).await;
  //     dbg!(3);
  //   });
  // });

  // let mut rt = Some(rt);
  // altv::events::on_resource_stop(move |_| {
  //   println!("waiting for shutdown");
  //   rt.take()
  //     .unwrap()
  //     .shutdown_timeout(Duration::from_millis(1000));
  // });

  // altv::log!("#################### ped");
  // // should be before core funcs because of changes in stream distance
  // test_ped();

  // altv::log!("#################### core_funcs");
  // test_core_funcs();
  // altv::log!("#################### test_base_object_funcs");
  // test_base_object_funcs();
  // altv::log!("#################### test_base_object_pool_funcs");
  // test_base_object_pool_funcs();
  // altv::log!("#################### mvalue");
  // test_mvalue();
  // altv::log!("#################### weapon_model_info");
  // test_weapon_model_info();
  // altv::log!("#################### timers");
  // test_timers();
  // altv::log!("#################### script_events");
  // test_script_events();
  // altv::log!("#################### blip");
  // test_blip();
  // altv::log!("#################### vehicle_model_info");
  // test_vehicle_model_info();
  // altv::log!("#################### resource");
  // test_resource();
  // altv::log!("#################### events");
  // test_events();
  // altv::log!("#################### error_backtrace");
  // test_error_backtrace();

  // altv::log!("#################### closest_entities");
  // test_closest_entities();

  // altv::set_timeout(
  //   || {
  //     altv::log!("#################### metadata_events");
  //     test_metadata_events();

  //     altv::set_timeout(
  //       || {
  //         altv::log!("stopping resource...");
  //         altv::Resource::current().stop().unwrap();
  //       },
  //       1000,
  //     );
  //   },
  //   CLOSEST_ENTITIES_TEST_TIMER,
  // );
}
