#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CGameEntityCreateEvent {

alt::IEntity* GetTarget(const alt::CGameEntityCreateEvent* ptr) {
    return ptr->GetTarget();
}

} // namespace