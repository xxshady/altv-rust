#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerHealEvent {

alt::IPlayer* GetTarget(const alt::CPlayerHealEvent* ptr) {
    return ptr->GetTarget();
}
u16 GetOldHealth(const alt::CPlayerHealEvent* ptr) {
    return ptr->GetOldHealth();
}
u16 GetNewHealth(const alt::CPlayerHealEvent* ptr) {
    return ptr->GetNewHealth();
}
u16 GetOldArmour(const alt::CPlayerHealEvent* ptr) {
    return ptr->GetOldArmour();
}
u16 GetNewArmour(const alt::CPlayerHealEvent* ptr) {
    return ptr->GetNewArmour();
}

} // namespace