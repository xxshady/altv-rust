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
void SetHealth(alt::IPed* ptr, u16 health) {
    return ptr->SetHealth(health);
}
void SetMaxHealth(alt::IPed* ptr, u16 health) {
    return ptr->SetMaxHealth(health);
}
void SetArmour(alt::IPed* ptr, u16 armor) {
    return ptr->SetArmour(armor);
}
void SetCurrentWeapon(alt::IPed* ptr, u32 weapon) {
    return ptr->SetCurrentWeapon(weapon);
}

} // namespace