#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IColShapeCylinder {

f32 GetRadius(const alt::IColShapeCylinder* ptr) {
    return ptr->GetRadius();
}
f32 GetHeight(const alt::IColShapeCylinder* ptr) {
    return ptr->GetHeight();
}

} // namespace