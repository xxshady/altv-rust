#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerWeaponChangeEvent {

u32 GetOldWeapon(const alt::CPlayerWeaponChangeEvent* ptr) {
    return ptr->GetOldWeapon();
}
u32 GetNewWeapon(const alt::CPlayerWeaponChangeEvent* ptr) {
    return ptr->GetNewWeapon();
}

} // namespace