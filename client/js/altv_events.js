import alt from "alt-client"
import { base_object_handle } from "./helpers.js"

/**
 * @param {import('./resource.js').Resource} resource
 * @param {*} event_name
 * @returns
 */
export function enable_altv_event(resource, event_name) {
  /**
   * @type {import("alt-client").IClientEvent}
   */
  const handlers = {
    serverStarted: () => {
      resource.call_export("on_altv_event", { serverStarted: {} })
    },
    consoleCommand: (name, ...args) => {
      resource.call_export("on_altv_event", { consoleCommand: { name, args } })
    },
    baseObjectCreate: (base_object) => {
      if (base_object.getStreamSyncedMeta) {
        alt.log(
          "[baseObjectCreate] ignoring base object with stream synced meta:",
          base_object.constructor.name,
        )
        return
      }

      resource.call_export("on_altv_event", {
        baseObjectCreate: {
          base_object: base_object_handle(base_object),
        },
      })
    },
    baseObjectRemove: (base_object) => {
      resource.call_export("on_altv_event", {
        baseObjectRemove: {
          base_object: base_object_handle(base_object),
        },
      })
    },
    gameEntityCreate: (entity) => {
      resource.call_export("on_altv_event", {
        gameEntityCreate: {
          entity: base_object_handle(entity),
        },
      })
    },
    gameEntityDestroy: (entity) => {
      resource.call_export("on_altv_event", {
        gameEntityDestroy: {
          entity: base_object_handle(entity),
        },
      })
    },
    worldObjectStreamIn: (world_object) => {
      resource.call_export("on_altv_event", {
        worldObjectStreamIn: {
          world_object: base_object_handle(world_object),
        },
      })
    },
    worldObjectStreamOut: (world_object) => {
      resource.call_export("on_altv_event", {
        worldObjectStreamOut: {
          world_object: base_object_handle(world_object),
        },
      })
    },
  }
  const handler = handlers[event_name]
  if (!handler) {
    alt.logError("unhandled event:", event_name)
    return
  }
  resource.add_event_handler(event_name, handler)
}
