#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CGlobalMetaDataChangeEvent {

std::string GetKey(const alt::CGlobalMetaDataChangeEvent* ptr) {
    return ptr->GetKey();
}
ConstMValueWrapper GetVal(const alt::CGlobalMetaDataChangeEvent* ptr) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetVal();
    return wrapper;
}
ConstMValueWrapper GetOldVal(const alt::CGlobalMetaDataChangeEvent* ptr) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetOldVal();
    return wrapper;
}

} // namespace