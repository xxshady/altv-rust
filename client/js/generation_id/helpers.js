import alt from "alt-client"

// special key to avoid collisions with user's code
export const GENERATION_ID_KEY = "&^#altv-rust"

/**
 * @param {import("alt-client").BaseObject} base_object
 */
export function get_server_base_object_generation_id(base_object) {
  let generation_id = base_object.server_generation_id
  if (generation_id != null) return generation_id

  if (base_object.getStreamSyncedMeta) {
    generation_id = base_object.getStreamSyncedMeta(GENERATION_ID_KEY)
  }
  else if (base_object.getSyncedMeta) {
    generation_id = base_object.getSyncedMeta(GENERATION_ID_KEY)
  }

  alt.Utils.assert(
    generation_id != null,
    `Failed to obtain generation id from server base object ${base_object.constructor.name}`,
  )

  base_object.server_generation_id = generation_id

  return generation_id
}

/**
 * @param {import("alt-client").BaseObject} base_object
 */
export function get_client_base_object_generation_id(base_object) {
  const generation_id = base_object.generation_id
  alt.Utils.assert(
    generation_id != null,
    `Failed to obtain generation id from client base object ${base_object.constructor.name}`,
  )
  return generation_id
}

/**
 * @param {import("alt-client").BaseObject} base_object
 */
export function get_base_object_generation_id(base_object) {
  if (base_object.isRemote) {
    return get_server_base_object_generation_id(base_object)
  }
  else {
    return get_client_base_object_generation_id(base_object)
  }
}
