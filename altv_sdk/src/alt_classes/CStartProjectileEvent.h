#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CStartProjectileEvent {

alt::IPlayer* GetSource(const alt::CStartProjectileEvent* ptr) {
    return ptr->GetSource();
}
Vector3Wrapper GetStartPosition(const alt::CStartProjectileEvent* ptr) {
    auto vector3 = ptr->GetStartPosition();
    return { vector3[0], vector3[1], vector3[2] };
}
Vector3Wrapper GetDirection(const alt::CStartProjectileEvent* ptr) {
    auto vector3 = ptr->GetDirection();
    return { vector3[0], vector3[1], vector3[2] };
}
u32 GetAmmoHash(const alt::CStartProjectileEvent* ptr) {
    return ptr->GetAmmoHash();
}
u32 GetWeaponHash(const alt::CStartProjectileEvent* ptr) {
    return ptr->GetWeaponHash();
}

} // namespace