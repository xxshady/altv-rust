use std::collections::HashMap;

pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let group = altv::VirtualEntityGroup::new(10);
    let entity = altv::VirtualEntity::new_with_stream_meta(
        group,
        altv::Vector3::new(0, 0, 72),
        10,
        HashMap::from([("example", 123)]),
    )
    .unwrap();
    dbg!(&entity);

    altv::events::on_player_connect(|c| {
        c.player.spawn("mp_m_freemode_01", (0, 0, 72))?;
        Ok(())
    });

    let m = altv::Marker::new(altv::MarkerType::Markerarrow, 0, (255, 0, 0));

    altv::set_interval(
        move || {
            let now = std::time::Instant::now();
            for _ in 0..100_000 {
                m.marker_type().unwrap();
            }
            dbg!(now.elapsed());

            Ok(())
        },
        1500,
    );
}
