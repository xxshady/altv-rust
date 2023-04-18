#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerSpawnEvent {

alt::IPlayer* GetPlayer(const alt::CPlayerSpawnEvent* ptr) {
    return ptr->GetPlayer();
}

} // namespace