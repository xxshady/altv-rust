#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerDimensionChangeEvent {

alt::IPlayer* GetTarget(const alt::CPlayerDimensionChangeEvent* ptr) {
    return ptr->GetTarget();
}
i32 GetOldDimension(const alt::CPlayerDimensionChangeEvent* ptr) {
    return ptr->GetOldDimension();
}
i32 GetNewDimension(const alt::CPlayerDimensionChangeEvent* ptr) {
    return ptr->GetNewDimension();
}

} // namespace