#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerWeaponChangeEvent {

alt::IPlayer* GetTarget(const alt::CPlayerWeaponChangeEvent* ptr) {
    return ptr->GetTarget();
}
u32 GetOldWeapon(const alt::CPlayerWeaponChangeEvent* ptr) {
    return ptr->GetOldWeapon();
}
u32 GetNewWeapon(const alt::CPlayerWeaponChangeEvent* ptr) {
    return ptr->GetNewWeapon();
}

} // namespace