#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerChangeInteriorEvent {

alt::IPlayer* GetTarget(const alt::CPlayerChangeInteriorEvent* ptr) {
    return ptr->GetTarget();
}
u32 GetOldInteriorLocation(const alt::CPlayerChangeInteriorEvent* ptr) {
    return ptr->GetOldInteriorLocation();
}
u32 GetNewInteriorLocation(const alt::CPlayerChangeInteriorEvent* ptr) {
    return ptr->GetNewInteriorLocation();
}

} // namespace