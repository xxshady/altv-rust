#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CVehicleDestroyEvent {

alt::IVehicle* GetTarget(const alt::CVehicleDestroyEvent* ptr) {
    return ptr->GetTarget();
}

} // namespace