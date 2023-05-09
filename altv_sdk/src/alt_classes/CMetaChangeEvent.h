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
MValueWrapper GetVal(const alt::CMetaChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = ptr->GetVal();
    return wrapper;
}
MValueWrapper GetOldVal(const alt::CMetaChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = ptr->GetOldVal();
    return wrapper;
}

} // namespace