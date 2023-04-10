#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CLocalMetaDataChangeEvent {

alt::IPlayer* GetTarget(const alt::CLocalMetaDataChangeEvent* ptr) {
    return ptr->GetTarget();
}
std::string GetKey(const alt::CLocalMetaDataChangeEvent* ptr) {
    return ptr->GetKey();
}
MValueWrapper GetVal(const alt::CLocalMetaDataChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(ptr->GetVal());
    return wrapper;
}
MValueWrapper GetOldVal(const alt::CLocalMetaDataChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(ptr->GetOldVal());
    return wrapper;
}

} // namespace