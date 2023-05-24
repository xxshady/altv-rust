#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CSyncedMetaDataChangeEvent {

alt::IBaseObject* GetTarget(const alt::CSyncedMetaDataChangeEvent* ptr) {
    return ptr->GetTarget();
}
std::string GetKey(const alt::CSyncedMetaDataChangeEvent* ptr) {
    return ptr->GetKey();
}
ConstMValueWrapper GetVal(const alt::CSyncedMetaDataChangeEvent* ptr) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetVal();
    return wrapper;
}
ConstMValueWrapper GetOldVal(const alt::CSyncedMetaDataChangeEvent* ptr) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetOldVal();
    return wrapper;
}

} // namespace