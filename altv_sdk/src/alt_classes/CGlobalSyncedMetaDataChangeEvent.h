#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CGlobalSyncedMetaDataChangeEvent {

std::string GetKey(const alt::CGlobalSyncedMetaDataChangeEvent* ptr) {
    return ptr->GetKey();
}
ConstMValueWrapper GetVal(const alt::CGlobalSyncedMetaDataChangeEvent* ptr) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetVal();
    return wrapper;
}
ConstMValueWrapper GetOldVal(const alt::CGlobalSyncedMetaDataChangeEvent* ptr) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetOldVal();
    return wrapper;
}

} // namespace