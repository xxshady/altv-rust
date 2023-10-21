#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IColShapeRect {

Vector2Wrapper GetMin(const alt::IColShapeRect* ptr) {
    auto vector2 = ptr->GetMin();
    return { vector2[0], vector2[1] };
}
Vector2Wrapper GetMax(const alt::IColShapeRect* ptr) {
    auto vector2 = ptr->GetMax();
    return { vector2[0], vector2[1] };
}

} // namespace