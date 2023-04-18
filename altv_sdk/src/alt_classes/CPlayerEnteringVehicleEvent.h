#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerEnteringVehicleEvent {

alt::IVehicle* GetTarget(const alt::CPlayerEnteringVehicleEvent* ptr) {
    return ptr->GetTarget();
}
alt::IPlayer* GetPlayer(const alt::CPlayerEnteringVehicleEvent* ptr) {
    return ptr->GetPlayer();
}
u8 GetSeat(const alt::CPlayerEnteringVehicleEvent* ptr) {
    return ptr->GetSeat();
}

} // namespace