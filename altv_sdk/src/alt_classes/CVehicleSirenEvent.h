#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CVehicleSirenEvent {

alt::IVehicle* GetTarget(const alt::CVehicleSirenEvent* ptr) {
    return ptr->GetTarget();
}
bool GetToggle(const alt::CVehicleSirenEvent* ptr) {
    return ptr->GetToggle();
}

} // namespace