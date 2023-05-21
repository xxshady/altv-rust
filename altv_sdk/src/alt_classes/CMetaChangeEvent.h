#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CMetaChangeEvent {

alt::IBaseObject* GetTarget(const alt::CMetaChangeEvent* ptr) {
    return ptr->GetTarget();
}
std::string GetKey(const alt::CMetaChangeEvent* ptr) {
    return ptr->GetKey();
}
ConstMValueWrapper GetVal(const alt::CMetaChangeEvent* ptr) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetVal();
    return wrapper;
}
ConstMValueWrapper GetOldVal(const alt::CMetaChangeEvent* ptr) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetOldVal();
    return wrapper;
}

} // namespace