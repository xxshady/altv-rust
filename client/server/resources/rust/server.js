import alt from "alt-server"

alt.on('playerConnect', (player) => {
  init(player)
})

const player = alt.Player.all[0]
if (player) init(player)

function init(player) {
  alt.log('init')

  player.spawn("mp_m_freemode_01", new alt.Vector3(0, 0, 70))
  alt.Vehicle.all.forEach(v => v.destroy())

  alt.setTimeout(() => {
    const veh = new alt.Vehicle('sultan2', player.pos.add(0, 2, 0), alt.Vector3.zero)
  }, 1000)
}
