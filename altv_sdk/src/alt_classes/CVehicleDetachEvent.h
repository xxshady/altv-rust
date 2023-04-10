#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CVehicleDetachEvent {

alt::IVehicle* GetTarget(const alt::CVehicleDetachEvent* ptr) {
    return ptr->GetTarget();
}
alt::IVehicle* GetDetached(const alt::CVehicleDetachEvent* ptr) {
    return ptr->GetDetached();
}

} // namespace