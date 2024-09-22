use std::marker::PhantomData;

use altv::{
  meta::{BaseObjectMetaEntry, StreamSyncedEntityMeta},
  BaseObjectPoolFuncs, Entity, WorldObject,
};

use serde::{Deserialize, Serialize};

#[altv::main]
fn main() {
  // let veh = altv::Vehicle::new("sultan2", (0, 3, 71), 0).unwrap();
  // let veh = altv::Vehicle::new("sultan2", (0, 3, 71), 0).unwrap();

  altv::events::on_player_connect(move |ctx| {
    new_player(ctx.player.clone())?;
    Ok(())
  });

  altv::set_timeout(
    || {
      let all = altv::Player::all();
      dbg!(all.len());

      all.iter().for_each(|p| {
        new_player(p.clone()).unwrap();
      });
    },
    1000,
  );

  fn new_player(player: altv::PlayerContainer) -> altv::VoidResult {
    altv::log!("new player: {}", player.name()?);
    // p.emit("test", &[&bincode::serialize(&(123_i32,))?])?;

    player.spawn("mp_f_freemode_01", (0, 0, 71))?;

    // altv::Vehicle::all().iter().for_each(|v| {
    //   v.destroy().unwrap();
    // });

    altv::set_timeout(
      move || {
        let veh = altv::Vehicle::new("sultan2", player.pos()?, 0)?;
        // veh.stream_synced_meta_entry("test")?.set(&123)?;
        altv::log!("created vehicle: {}", veh.id()?);

        player.emit(
          "deserialize_base_object",
          &[&bincode::serialize(&(AnyBaseObjectHandle::Vehicle(
            VehicleHandle {
              id: veh.id()?,
              generation: veh
                .stream_synced_meta_entry("&^#altv-rust")?
                .get()
                .unwrap()
                .unwrap(),
              _t: PhantomData,
            },
          ),))?],
        )?;

        Ok(())
      },
      1000,
    );

    // altv::set_timeout(
    //     || {
    //         altv::log!("hiding all vehicles");
    //         altv::Vehicle::all().iter().for_each(|v| {
    //             v.set_streamed(false).unwrap();
    //         });
    //     },
    //     10_000,
    // );

    // altv::set_timeout(
    //     || {
    //         altv::log!("showing all vehicles");
    //         altv::Vehicle::all().iter().for_each(|v| {
    //             v.set_streamed(true).unwrap();
    //         });
    //     },
    //     12_000,
    // );
    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnyBaseObjectHandle {
  Vehicle(VehicleHandle),
  // Player(PlayerHandle),
  // LocalPlayer(LocalPlayerHandle),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BaseObjectHandle<T> {
  id: u32,
  generation: u64,
  #[serde(skip_serializing, skip_deserializing)]
  _t: PhantomData<T>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct VehicleType;
pub type VehicleHandle = BaseObjectHandle<VehicleType>;
