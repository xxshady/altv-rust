pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    // TODO: fix backtrace panic
    std::env::set_var("RUST_BACKTRACE", "full");

    alt::events::on_stream_synced_meta_change(|c| {
        dbg!(c);
        Ok(())
    });

    alt::events::on_server_started(|_| {
        let veh = alt::Vehicle::new("sultan2", 0, 0).unwrap();

        dbg!(veh.set_stream_synced_meta("test", alt::mvalue::list![veh.clone()].unwrap()));
        dbg!(veh.get_stream_synced_meta_keys());
        dbg!(veh.get_stream_synced_meta("test"));
        dbg!(veh.has_stream_synced_meta("test"));
        dbg!(veh.delete_stream_synced_meta("test"));
        dbg!(veh.get_stream_synced_meta("test"));
        dbg!(veh.has_stream_synced_meta("test"));
        dbg!(veh.get_stream_synced_meta_keys());

        dbg!(veh.set_meta("test", 123i64));
        dbg!(veh.get_meta("test"));
        dbg!(veh.get_meta_keys());

        veh.attach_to_entity_bone_index(
            veh.clone(),
            alt::AttachToEntityBoneIndex {
                collision: true,
                ..Default::default()
            },
        )?;

        Ok(())
    });

    alt::events::on_client("test", |player, _| {
        player.set_date_time(alt::PlayerDateTime {
            hour: 4,
            ..Default::default()
        })?;
        Ok(())
    });
}
