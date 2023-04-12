#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace VehicleModelInfo {

bool DoesExtraExist(const alt::VehicleModelInfo* ptr, u8 extraId) {
    return ptr->DoesExtraExist(extraId);
}
bool DoesExtraDefault(const alt::VehicleModelInfo* ptr, u8 extraId) {
    return ptr->DoesExtraDefault(extraId);
}

} // namespace