#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CGlobalMetaDataChangeEvent {

std::string GetKey(const alt::CGlobalMetaDataChangeEvent* ptr) {
    return ptr->GetKey();
}
MValueWrapper GetVal(const alt::CGlobalMetaDataChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(ptr->GetVal());
    return wrapper;
}
MValueWrapper GetOldVal(const alt::CGlobalMetaDataChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(ptr->GetOldVal());
    return wrapper;
}

} // namespace