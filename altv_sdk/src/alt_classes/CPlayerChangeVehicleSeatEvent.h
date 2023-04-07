#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerChangeVehicleSeatEvent {

alt::IVehicle* GetTarget(const alt::CPlayerChangeVehicleSeatEvent* ptr) {
    return ptr->GetTarget();
}
alt::IPlayer* GetPlayer(const alt::CPlayerChangeVehicleSeatEvent* ptr) {
    return ptr->GetPlayer();
}
u8 GetOldSeat(const alt::CPlayerChangeVehicleSeatEvent* ptr) {
    return ptr->GetOldSeat();
}
u8 GetNewSeat(const alt::CPlayerChangeVehicleSeatEvent* ptr) {
    return ptr->GetNewSeat();
}

} // namespace