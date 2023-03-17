#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IColShape {

ColShapeType GetColshapeType(const alt::IColShape* ptr) {
    return static_cast<uint8_t>(ptr->GetColshapeType());
}
bool IsEntityIn(const alt::IColShape* ptr, alt::IEntity* ent) {
    return ptr->IsEntityIn(ent);
}
bool IsEntityIdIn(const alt::IColShape* ptr, u16 id) {
    return ptr->IsEntityIdIn(id);
}
bool IsPointIn(const alt::IColShape* ptr, f32 p_x, f32 p_y, f32 p_z) {
    return ptr->IsPointIn({ p_x, p_y, p_z });
}
void SetPlayersOnly(alt::IColShape* ptr, bool state) {
    return ptr->SetPlayersOnly(state);
}
bool IsPlayersOnly(const alt::IColShape* ptr) {
    return ptr->IsPlayersOnly();
}

} // namespace