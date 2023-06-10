#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CWeaponDamageEvent {

alt::IPlayer* GetSource(const alt::CWeaponDamageEvent* ptr) {
    return ptr->GetSource();
}
alt::IEntity* GetTarget(const alt::CWeaponDamageEvent* ptr) {
    return ptr->GetTarget();
}
u32 GetWeaponHash(const alt::CWeaponDamageEvent* ptr) {
    return ptr->GetWeaponHash();
}
u32 GetDamageValue(const alt::CWeaponDamageEvent* ptr) {
    return ptr->GetDamageValue();
}
Vector3Wrapper GetShotOffset(const alt::CWeaponDamageEvent* ptr) {
    auto vector3 = ptr->GetShotOffset();
    return { vector3[0], vector3[1], vector3[2] };
}
WeaponDamageEventBodyPart GetBodyPart(const alt::CWeaponDamageEvent* ptr) {
    return static_cast<int8_t>(ptr->GetBodyPart());
}
alt::IEntity* GetSourceEntity(const alt::CWeaponDamageEvent* ptr) {
    return ptr->GetSourceEntity();
}
void SetDamageValue(alt::CWeaponDamageEvent* ptr, u32 _damageValue) {
    return ptr->SetDamageValue(_damageValue);
}

} // namespace