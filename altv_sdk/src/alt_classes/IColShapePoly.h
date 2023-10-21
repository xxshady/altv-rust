#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IColShapePoly {

f32 GetMinZ(const alt::IColShapePoly* ptr) {
    return ptr->GetMinZ();
}
f32 GetMaxZ(const alt::IColShapePoly* ptr) {
    return ptr->GetMaxZ();
}
Vector2Vec GetPoints(const alt::IColShapePoly* ptr) {
    auto vec = create_vector2_vec();
    for (const auto& v : ptr->GetPoints()) {
        push_to_vector2_vec(vec, v[0], v[1]);
    }
    return vec;
}

} // namespace