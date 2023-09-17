#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CClientDeleteObjectEvent {

alt::IPlayer* GetTarget(const alt::CClientDeleteObjectEvent* ptr) {
    return ptr->GetTarget();
}

} // namespace