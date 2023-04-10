#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CMetaChangeEvent {

alt::IEntity* GetTarget(const alt::CMetaChangeEvent* ptr) {
    return ptr->GetTarget();
}
std::string GetKey(const alt::CMetaChangeEvent* ptr) {
    return ptr->GetKey();
}
MValueWrapper GetVal(const alt::CMetaChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(ptr->GetVal());
    return wrapper;
}
MValueWrapper GetOldVal(const alt::CMetaChangeEvent* ptr) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(ptr->GetOldVal());
    return wrapper;
}

} // namespace