import alt from "alt-client"
import { Resource } from "./resource.js"
import { base_object_handle } from "./helpers.js"

/**
 * @param {Resource} resource
 */
export function init_created_base_objects(resource) {
  // TODO: other base object types

  alt.Vehicle.streamedIn.forEach(v => {
    resource.call_export("on_altv_event", {
      gameEntityCreate: {
        entity: base_object_handle(v),
      },
    })
  })
  alt.Player.streamedIn.forEach(v => {
    resource.call_export("on_altv_event", {
      gameEntityCreate: {
        entity: base_object_handle(v),
      },
    })
  })
}
