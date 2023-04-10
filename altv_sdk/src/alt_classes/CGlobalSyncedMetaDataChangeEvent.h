#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CGlobalSyncedMetaDataChangeEvent {

std::string GetKey(const alt::CGlobalSyncedMetaDataChangeEvent* ptr) {
    return ptr->GetKey();
}
MValueWrapper GetVal(const alt::CGlobalSyncedMetaDataChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(ptr->GetVal());
    return wrapper;
}
MValueWrapper GetOldVal(const alt::CGlobalSyncedMetaDataChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(ptr->GetOldVal());
    return wrapper;
}

} // namespace