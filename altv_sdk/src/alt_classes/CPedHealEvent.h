#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPedHealEvent {

alt::IPed* GetTarget(const alt::CPedHealEvent* ptr) {
    return ptr->GetTarget();
}
u16 GetOldHealth(const alt::CPedHealEvent* ptr) {
    return ptr->GetOldHealth();
}
u16 GetNewHealth(const alt::CPedHealEvent* ptr) {
    return ptr->GetNewHealth();
}
u16 GetOldArmour(const alt::CPedHealEvent* ptr) {
    return ptr->GetOldArmour();
}
u16 GetNewArmour(const alt::CPedHealEvent* ptr) {
    return ptr->GetNewArmour();
}

} // namespace