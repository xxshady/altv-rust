#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CConnectionQueueRemoveEvent {

alt::IConnectionInfo* GetConnectionInfo(const alt::CConnectionQueueRemoveEvent* ptr) {
    return ptr->GetConnectionInfo();
}

} // namespace