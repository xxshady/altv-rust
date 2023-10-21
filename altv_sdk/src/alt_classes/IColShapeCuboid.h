#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IColShapeCuboid {

Vector3Wrapper GetMin(const alt::IColShapeCuboid* ptr) {
    auto vector3 = ptr->GetMin();
    return { vector3[0], vector3[1], vector3[2] };
}
Vector3Wrapper GetMax(const alt::IColShapeCuboid* ptr) {
    auto vector3 = ptr->GetMax();
    return { vector3[0], vector3[1], vector3[2] };
}

} // namespace