#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CVehicleDamageEvent {

alt::IVehicle* GetTarget(const alt::CVehicleDamageEvent* ptr) {
    return ptr->GetTarget();
}
alt::IEntity* GetDamager(const alt::CVehicleDamageEvent* ptr) {
    return ptr->GetDamager();
}
u32 GetBodyHealthDamage(const alt::CVehicleDamageEvent* ptr) {
    return ptr->GetBodyHealthDamage();
}
u32 GetBodyAdditionalHealthDamage(const alt::CVehicleDamageEvent* ptr) {
    return ptr->GetBodyAdditionalHealthDamage();
}
u32 GetEngineHealthDamage(const alt::CVehicleDamageEvent* ptr) {
    return ptr->GetEngineHealthDamage();
}
u32 GetPetrolTankHealthDamage(const alt::CVehicleDamageEvent* ptr) {
    return ptr->GetPetrolTankHealthDamage();
}
u32 GetDamagedWith(const alt::CVehicleDamageEvent* ptr) {
    return ptr->GetDamagedWith();
}

} // namespace