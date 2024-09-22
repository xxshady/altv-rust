import alt from "alt-client"

/**
 * @param {import("./resource.js").Resource} resource
 */
export function init(resource) {
  resource.generic_local_event_handler = (event_name, ...args) => {
    resource.call_export("on_script_event", {
      source: 0, // local
      name: event_name,
      args,
    })
  }
  alt.on(resource.generic_local_event_handler)

  resource.generic_remote_event_handler = (event_name, ...args) => {
    resource.call_export("on_script_event", {
      source: 1, // remote
      name: event_name,
      args,
    })
  }
  alt.onServer(resource.generic_remote_event_handler)
}
