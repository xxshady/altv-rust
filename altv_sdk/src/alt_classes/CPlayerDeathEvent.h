#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerDeathEvent {

alt::IPlayer* GetTarget(const alt::CPlayerDeathEvent* ptr) {
    return ptr->GetTarget();
}
alt::IEntity* GetKiller(const alt::CPlayerDeathEvent* ptr) {
    return ptr->GetKiller();
}
u32 GetWeapon(const alt::CPlayerDeathEvent* ptr) {
    return ptr->GetWeapon();
}

} // namespace