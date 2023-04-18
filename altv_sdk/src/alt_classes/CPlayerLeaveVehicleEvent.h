#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerLeaveVehicleEvent {

alt::IVehicle* GetTarget(const alt::CPlayerLeaveVehicleEvent* ptr) {
    return ptr->GetTarget();
}
alt::IPlayer* GetPlayer(const alt::CPlayerLeaveVehicleEvent* ptr) {
    return ptr->GetPlayer();
}
u8 GetSeat(const alt::CPlayerLeaveVehicleEvent* ptr) {
    return ptr->GetSeat();
}

} // namespace