#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPedDamageEvent {

alt::IPed* GetTarget(const alt::CPedDamageEvent* ptr) {
    return ptr->GetTarget();
}
alt::IEntity* GetAttacker(const alt::CPedDamageEvent* ptr) {
    return ptr->GetAttacker();
}
u16 GetHealthDamage(const alt::CPedDamageEvent* ptr) {
    return ptr->GetHealthDamage();
}
u16 GetArmourDamage(const alt::CPedDamageEvent* ptr) {
    return ptr->GetArmourDamage();
}
u32 GetWeapon(const alt::CPedDamageEvent* ptr) {
    return ptr->GetWeapon();
}

} // namespace