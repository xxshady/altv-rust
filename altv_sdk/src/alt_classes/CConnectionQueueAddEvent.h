#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CConnectionQueueAddEvent {

alt::IConnectionInfo* GetConnectionInfo(const alt::CConnectionQueueAddEvent* ptr) {
    return ptr->GetConnectionInfo();
}

} // namespace