import alt from "alt-client"
import load_wasm from "../rust_wasm/pkg/rust_wasm_bg.wasm"
import { enable_altv_event } from "./altv_events.js"
import { Resource } from "./resource.js"
import * as script_events from "./script_events.js"
import "./generation_id.js"
import { get_server_base_object_generation_id } from "./generation_id.js"
import * as sourcemap from "./sourcemap.js"
import { base_object_handle } from "./helpers.js"
import { init_created_base_objects } from "./init_created_base_objects.js"

async function main() {
  Error.stackTraceLimit = 100

  await sourcemap.init()

  let resource_instance

  const exports = load_wasm({
    log_info(string) {
      alt.log("[rust]", string)
    },
    log_warn(string) {
      alt.logWarning("[rust]", string)
    },
    log_error(string) {
      alt.logError("[rust]", string)
    },
    enable_altv_event(event_name) {
      enable_altv_event(resource_instance, event_name)
    },
    disable_altv_event(event_name) {
      resource_instance.remove_event_handler(event_name)
    },
    get_base_object_ref(sdk_type, is_remote, id) {
      if (is_remote) {
        return alt.BaseObject.getByRemoteID(sdk_type, id)
      }
      else {
        return alt.BaseObject.getByID(sdk_type, id)
      }
    },
    emit_local_event_rust(event_name, buffer) {
      console.log("emit_local_event_rust", { event_name, buffer })
      alt.emit(event_name, buffer)
    },
    emit_local_event_js(event_name, args) {
      alt.emit(event_name, ...args)
    },

    // returns Vec<PlayerHandle>
    get_streamed_in_players() {
      return alt.Player.streamedIn.map(p => ({
        id: p.id,
        generation: get_server_base_object_generation_id(p),
      }))
    },

    // returns Vec<VehicleHandle>
    get_streamed_in_vehicles() {
      return alt.Vehicle.streamedIn.map(v => ({
        id: v.id,
        generation: get_server_base_object_generation_id(v),
      }))
    },

    get_net_time() {
      return alt.getNetTime()
    },

    get_base_object_raw_handle(js_ref) {
      return base_object_handle(js_ref)
    },

    get_local_player() {
      return alt.Player.local
    },

    is_local_player(base_object) {
      return alt.Player.local === base_object
    },

    BaseObject: alt.BaseObject,
  })
  resource_instance = new Resource(exports)

  script_events.init(resource_instance)

  resource_instance.call_export("main")
  init_created_base_objects(resource_instance)

  resource_instance.add_timer(alt.everyTick(() => {
    resource_instance.call_export("on_every_tick")
  }))

  // ----------------------------- testing

  // resource_instance.call_export("test_base_object")
  resource_instance.call_export("test_script_events")
  // resource_instance.call_export("test_timers")
  // resource_instance.call_export("test_timers2")
  // resource_instance.call_export("test_altv_events2")
  // resource_instance.call_export("test_local_player")
  // resource_instance.call_export("test_vehicle")

  // alt.emit("test")
  // alt.emit("test", 1, 2, 3)
  // alt.emit("test", 256)
}

main().catch(alt.logError)
