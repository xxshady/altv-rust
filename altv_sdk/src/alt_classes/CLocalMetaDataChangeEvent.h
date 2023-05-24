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
ConstMValueWrapper GetVal(const alt::CLocalMetaDataChangeEvent* ptr) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetVal();
    return wrapper;
}
ConstMValueWrapper GetOldVal(const alt::CLocalMetaDataChangeEvent* ptr) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetOldVal();
    return wrapper;
}

} // namespace