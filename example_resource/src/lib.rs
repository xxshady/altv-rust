pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    alt::events::on_player_connect(|c| {
        let player = c.player.clone();
        alt::log!(
            "player connect: ~gl~ {} ip: {:?}",
            player.name()?,
            player.ip()?.to_string()
        );

        player.spawn("mp_m_freemode_01", alt::Vector3::new(0., 0., 72.))?;

        player.give_weapon("weapon_pistol", 10, false)?;
        player.give_weapon("weapon_snowball", 2, false)?;
        player.give_weapon("weapon_acidpackage", 2, false)?;

        dbg!(player.weapons()?);

        dbg!(player.set_clothes(11, 5, 0, 0)?);
        dbg!(player.get_clothes(11)?);

        dbg!(player.set_prop(0, 1, 0)?);
        dbg!(player.get_prop(0)?);

        let vehicle = alt::Vehicle::new("bmx", alt::Vector3::new(5., 0., 71.), 0)?;
        player.set_into_vehicle(vehicle.clone(), 1)?;

        vehicle.set_quaternion(0)?;

        Ok(())
    });
}
