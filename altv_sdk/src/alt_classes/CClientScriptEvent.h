#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CClientScriptEvent {

alt::IPlayer* GetTarget(const alt::CClientScriptEvent* ptr) {
    return ptr->GetTarget();
}
const StdStringClone GetName(const alt::CClientScriptEvent* ptr) {
    return std::string { ptr->GetName() };
}
const MValueWrapperVec GetArgs(const alt::CClientScriptEvent* ptr) {
    auto args = ptr->GetArgs();
    auto mvalue_vec = create_mvalue_vec();
    auto size = args.GetSize();
    for (alt::Size i = 0; i < size; ++i) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(args[i]);
    mvalue_vec.push_back(wrapper.clone());
    }
    return mvalue_vec;
}

} // namespace