#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CClientRequestObjectEvent {

alt::IPlayer* GetTarget(const alt::CClientRequestObjectEvent* ptr) {
    return ptr->GetTarget();
}
u32 GetModel(const alt::CClientRequestObjectEvent* ptr) {
    return ptr->GetModel();
}
Vector3Wrapper GetPosition(const alt::CClientRequestObjectEvent* ptr) {
    auto vector3 = ptr->GetPosition();
    return { vector3[0], vector3[1], vector3[2] };
}

} // namespace