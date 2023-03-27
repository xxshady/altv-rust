#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IVirtualEntityGroup {

u32 GetID(const alt::IVirtualEntityGroup* ptr) {
    return ptr->GetID();
}
u32 GetStreamingRangeLimit(const alt::IVirtualEntityGroup* ptr) {
    return ptr->GetStreamingRangeLimit();
}

} // namespace