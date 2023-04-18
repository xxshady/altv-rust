#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IWorldObject {

Vector3Wrapper GetPosition(const alt::IWorldObject* ptr) {
    auto vector3 = ptr->GetPosition();
    return { vector3[0], vector3[1], vector3[2] };
}
void SetPosition(alt::IWorldObject* ptr, f32 pos_x, f32 pos_y, f32 pos_z) {
    return ptr->SetPosition({ pos_x, pos_y, pos_z });
}
i32 GetDimension(const alt::IWorldObject* ptr) {
    return ptr->GetDimension();
}
void SetDimension(alt::IWorldObject* ptr, i32 dimension) {
    return ptr->SetDimension(dimension);
}

} // namespace