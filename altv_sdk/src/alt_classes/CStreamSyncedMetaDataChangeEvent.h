#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CStreamSyncedMetaDataChangeEvent {

alt::IBaseObject* GetTarget(const alt::CStreamSyncedMetaDataChangeEvent* ptr) {
    return ptr->GetTarget();
}
std::string GetKey(const alt::CStreamSyncedMetaDataChangeEvent* ptr) {
    return ptr->GetKey();
}
ConstMValueWrapper GetVal(const alt::CStreamSyncedMetaDataChangeEvent* ptr) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetVal();
    return wrapper;
}
ConstMValueWrapper GetOldVal(const alt::CStreamSyncedMetaDataChangeEvent* ptr) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetOldVal();
    return wrapper;
}

} // namespace