#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CVehicleHornEvent {

alt::IVehicle* GetTarget(const alt::CVehicleHornEvent* ptr) {
    return ptr->GetTarget();
}
alt::IPlayer* GetReporter(const alt::CVehicleHornEvent* ptr) {
    return ptr->GetReporter();
}
bool GetToggle(const alt::CVehicleHornEvent* ptr) {
    return ptr->GetToggle();
}

} // namespace