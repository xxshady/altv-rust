#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CGameEntityDestroyEvent {

alt::IEntity* GetTarget(const alt::CGameEntityDestroyEvent* ptr) {
    return ptr->GetTarget();
}

} // namespace