#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CExplosionEvent {

alt::IPlayer* GetSource(const alt::CExplosionEvent* ptr) {
    return ptr->GetSource();
}
alt::IEntity* GetTarget(const alt::CExplosionEvent* ptr) {
    return ptr->GetTarget();
}
ExplosionType GetExplosionType(const alt::CExplosionEvent* ptr) {
    return static_cast<int8_t>(ptr->GetExplosionType());
}
Vector3Wrapper GetPosition(const alt::CExplosionEvent* ptr) {
    auto vector3 = ptr->GetPosition();
    return { vector3[0], vector3[1], vector3[2] };
}
u32 GetExplosionFX(const alt::CExplosionEvent* ptr) {
    return ptr->GetExplosionFX();
}

} // namespace