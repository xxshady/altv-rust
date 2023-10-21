#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPedDeathEvent {

alt::IPed* GetTarget(const alt::CPedDeathEvent* ptr) {
    return ptr->GetTarget();
}
alt::IEntity* GetKiller(const alt::CPedDeathEvent* ptr) {
    return ptr->GetKiller();
}
u32 GetWeapon(const alt::CPedDeathEvent* ptr) {
    return ptr->GetWeapon();
}

} // namespace