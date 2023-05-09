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
MValueWrapper GetVal(const alt::CStreamSyncedMetaDataChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = ptr->GetVal();
    return wrapper;
}
MValueWrapper GetOldVal(const alt::CStreamSyncedMetaDataChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = ptr->GetOldVal();
    return wrapper;
}

} // namespace