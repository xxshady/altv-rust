#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IPed {

u16 GetHealth(const alt::IPed* ptr) {
    return ptr->GetHealth();
}
u16 GetMaxHealth(const alt::IPed* ptr) {
    return ptr->GetMaxHealth();
}
u16 GetArmour(const alt::IPed* ptr) {
    return ptr->GetArmour();
}
u32 GetCurrentWeapon(const alt::IPed* ptr) {
    return ptr->GetCurrentWeapon();
}

} // namespace