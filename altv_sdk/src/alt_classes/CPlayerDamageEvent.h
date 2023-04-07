#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerDamageEvent {

alt::IPlayer* GetTarget(const alt::CPlayerDamageEvent* ptr) {
    return ptr->GetTarget();
}
alt::IEntity* GetAttacker(const alt::CPlayerDamageEvent* ptr) {
    return ptr->GetAttacker();
}
u16 GetHealthDamage(const alt::CPlayerDamageEvent* ptr) {
    return ptr->GetHealthDamage();
}
u16 GetArmourDamage(const alt::CPlayerDamageEvent* ptr) {
    return ptr->GetArmourDamage();
}
u32 GetWeapon(const alt::CPlayerDamageEvent* ptr) {
    return ptr->GetWeapon();
}

} // namespace