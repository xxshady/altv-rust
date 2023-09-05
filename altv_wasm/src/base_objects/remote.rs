use crate::__imports;

use super::base::private::Ptr;

pub trait RemoteBaseObject: Ptr {
    fn remote_id(&self) -> u32 {
        __imports::base_object_get_remote_id(self.ptr())
    }
}
