#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CVehicleAttachEvent {

alt::IVehicle* GetTarget(const alt::CVehicleAttachEvent* ptr) {
    return ptr->GetTarget();
}
alt::IVehicle* GetAttached(const alt::CVehicleAttachEvent* ptr) {
    return ptr->GetAttached();
}

} // namespace