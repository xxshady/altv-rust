#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CGlobalSyncedMetaDataChangeEvent {

std::string GetKey(const alt::CGlobalSyncedMetaDataChangeEvent* ptr) {
    return ptr->GetKey();
}
MValueWrapper GetVal(const alt::CGlobalSyncedMetaDataChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = ptr->GetVal();
    return wrapper;
}
MValueWrapper GetOldVal(const alt::CGlobalSyncedMetaDataChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = ptr->GetOldVal();
    return wrapper;
}

} // namespace