#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerRequestControlEvent {

alt::IEntity* GetTarget(const alt::CPlayerRequestControlEvent* ptr) {
    return ptr->GetTarget();
}
alt::IPlayer* GetPlayer(const alt::CPlayerRequestControlEvent* ptr) {
    return ptr->GetPlayer();
}

} // namespace