#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerEnterVehicleEvent {

alt::IVehicle* GetTarget(const alt::CPlayerEnterVehicleEvent* ptr) {
    return ptr->GetTarget();
}
alt::IPlayer* GetPlayer(const alt::CPlayerEnterVehicleEvent* ptr) {
    return ptr->GetPlayer();
}
u8 GetSeat(const alt::CPlayerEnterVehicleEvent* ptr) {
    return ptr->GetSeat();
}

} // namespace