import {
  get_client_base_object_generation_id,
  get_server_base_object_generation_id,
} from "./generation_id.js"

export function base_object_handle(base_object) {
  return base_object.isRemote ?
    {
      sdk_type: base_object.type,
      is_remote: true,
      id: base_object.remoteID,
      generation: get_server_base_object_generation_id(base_object),
    } :
    {
      sdk_type: base_object.type,
      is_remote: false,
      id: base_object.id,
      generation: get_client_base_object_generation_id(base_object),
    }
}
