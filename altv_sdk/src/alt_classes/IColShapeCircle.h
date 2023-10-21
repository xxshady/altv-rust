#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IColShapeCircle {

f32 GetRadius(const alt::IColShapeCircle* ptr) {
    return ptr->GetRadius();
}

} // namespace