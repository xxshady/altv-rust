#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IVirtualEntityGroup {

u32 GetMaxEntitiesInStream(const alt::IVirtualEntityGroup* ptr) {
    return ptr->GetMaxEntitiesInStream();
}

} // namespace