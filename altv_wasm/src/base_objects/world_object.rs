use altv_wasm_shared::Vector3;

use crate::__imports;
use super::base::private::Ptr;

pub trait WorldObject: Ptr {
    fn pos(&self) -> Vector3 {
        __imports::world_object_get_pos(self.ptr())
    }

    // TODO: should be implemented for server entities and owned entities separately
    // fn set_pos(&self, value: Vector3) {
    //     __imports::world_object_set_pos(self.ptr(), value)
    // }

    fn dimension(&self) -> i32 {
        __imports::world_object_get_dimension(self.ptr())
    }

    // TODO: should be implemented for server entities and owned entities separately
    // fn set_dimension(&self, value: i32) {
    //     __imports::world_object_set_dimension(self.ptr(), value)
    // }
}
