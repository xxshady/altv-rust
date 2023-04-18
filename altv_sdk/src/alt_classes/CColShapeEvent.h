#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CColShapeEvent {

alt::IColShape* GetTarget(const alt::CColShapeEvent* ptr) {
    return ptr->GetTarget();
}
alt::IWorldObject* GetEntity(const alt::CColShapeEvent* ptr) {
    return ptr->GetEntity();
}
bool GetState(const alt::CColShapeEvent* ptr) {
    return ptr->GetState();
}

} // namespace