#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CSyncedMetaDataChangeEvent {

alt::IEntity* GetTarget(const alt::CSyncedMetaDataChangeEvent* ptr) {
    return ptr->GetTarget();
}
std::string GetKey(const alt::CSyncedMetaDataChangeEvent* ptr) {
    return ptr->GetKey();
}
MValueWrapper GetVal(const alt::CSyncedMetaDataChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(ptr->GetVal());
    return wrapper;
}
MValueWrapper GetOldVal(const alt::CSyncedMetaDataChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(ptr->GetOldVal());
    return wrapper;
}

} // namespace