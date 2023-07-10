#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IPlayer {

std::string GetName(const alt::IPlayer* ptr) {
    return ptr->GetName();
}
u16 GetHealth(const alt::IPlayer* ptr) {
    return ptr->GetHealth();
}
u16 GetMaxHealth(const alt::IPlayer* ptr) {
    return ptr->GetMaxHealth();
}
bool HasWeaponComponent(const alt::IPlayer* ptr, u32 weapon, u32 component) {
    return ptr->HasWeaponComponent(weapon, component);
}
std::vector<u32> GetCurrentWeaponComponents(const alt::IPlayer* ptr) {
    return ptr->GetCurrentWeaponComponents();
}
u8 GetWeaponTintIndex(const alt::IPlayer* ptr, u32 weapon) {
    return ptr->GetWeaponTintIndex(weapon);
}
u8 GetCurrentWeaponTintIndex(const alt::IPlayer* ptr) {
    return ptr->GetCurrentWeaponTintIndex();
}
u32 GetCurrentWeapon(const alt::IPlayer* ptr) {
    return ptr->GetCurrentWeapon();
}
bool IsDead(const alt::IPlayer* ptr) {
    return ptr->IsDead();
}
bool IsJumping(const alt::IPlayer* ptr) {
    return ptr->IsJumping();
}
bool IsInRagdoll(const alt::IPlayer* ptr) {
    return ptr->IsInRagdoll();
}
bool IsAiming(const alt::IPlayer* ptr) {
    return ptr->IsAiming();
}
bool IsShooting(const alt::IPlayer* ptr) {
    return ptr->IsShooting();
}
bool IsReloading(const alt::IPlayer* ptr) {
    return ptr->IsReloading();
}
bool IsEnteringVehicle(const alt::IPlayer* ptr) {
    return ptr->IsEnteringVehicle();
}
bool IsLeavingVehicle(const alt::IPlayer* ptr) {
    return ptr->IsLeavingVehicle();
}
bool IsOnLadder(const alt::IPlayer* ptr) {
    return ptr->IsOnLadder();
}
bool IsInMelee(const alt::IPlayer* ptr) {
    return ptr->IsInMelee();
}
bool IsInCover(const alt::IPlayer* ptr) {
    return ptr->IsInCover();
}
u16 GetArmour(const alt::IPlayer* ptr) {
    return ptr->GetArmour();
}
u16 GetMaxArmour(const alt::IPlayer* ptr) {
    return ptr->GetMaxArmour();
}
f32 GetMoveSpeed(const alt::IPlayer* ptr) {
    return ptr->GetMoveSpeed();
}
Vector3Wrapper GetAimPos(const alt::IPlayer* ptr) {
    auto vector3 = ptr->GetAimPos();
    return { vector3[0], vector3[1], vector3[2] };
}
Vector3Wrapper GetHeadRotation(const alt::IPlayer* ptr) {
    auto vector3 = ptr->GetHeadRotation();
    return { vector3[0], vector3[1], vector3[2] };
}
bool IsInVehicle(const alt::IPlayer* ptr) {
    return ptr->IsInVehicle();
}
alt::IVehicle* GetVehicle(const alt::IPlayer* ptr) {
    return ptr->GetVehicle();
}
u8 GetSeat(const alt::IPlayer* ptr) {
    return ptr->GetSeat();
}
alt::IEntity* GetEntityAimingAt(const alt::IPlayer* ptr) {
    return ptr->GetEntityAimingAt();
}
Vector3Wrapper GetEntityAimOffset(const alt::IPlayer* ptr) {
    auto vector3 = ptr->GetEntityAimOffset();
    return { vector3[0], vector3[1], vector3[2] };
}
bool IsFlashlightActive(const alt::IPlayer* ptr) {
    return ptr->IsFlashlightActive();
}
bool IsSuperJumpEnabled(const alt::IPlayer* ptr) {
    return ptr->IsSuperJumpEnabled();
}
bool IsCrouching(const alt::IPlayer* ptr) {
    return ptr->IsCrouching();
}
bool IsStealthy(const alt::IPlayer* ptr) {
    return ptr->IsStealthy();
}
u32 GetCurrentAnimationDict(const alt::IPlayer* ptr) {
    return ptr->GetCurrentAnimationDict();
}
u32 GetCurrentAnimationName(const alt::IPlayer* ptr) {
    return ptr->GetCurrentAnimationName();
}
bool IsSpawned(const alt::IPlayer* ptr) {
    return ptr->IsSpawned();
}
f32 GetForwardSpeed(const alt::IPlayer* ptr) {
    return ptr->GetForwardSpeed();
}
f32 GetStrafeSpeed(const alt::IPlayer* ptr) {
    return ptr->GetStrafeSpeed();
}
bool IsTalking(const alt::IPlayer* ptr) {
    return ptr->IsTalking();
}
f32 GetMicLevel(const alt::IPlayer* ptr) {
    return ptr->GetMicLevel();
}
f32 GetSpatialVolume(const alt::IPlayer* ptr) {
    return ptr->GetSpatialVolume();
}
void SetSpatialVolume(alt::IPlayer* ptr, f32 volume) {
    return ptr->SetSpatialVolume(volume);
}
f32 GetNonSpatialVolume(const alt::IPlayer* ptr) {
    return ptr->GetNonSpatialVolume();
}
void SetNonSpatialVolume(alt::IPlayer* ptr, f32 volume) {
    return ptr->SetNonSpatialVolume(volume);
}
void SetAmmoSpecialType(alt::IPlayer* ptr, u32 ammoHash, AmmoSpecialType_t ammoSpecialType) {
    return ptr->SetAmmoSpecialType(ammoHash, static_cast<alt::AmmoSpecialType>(ammoSpecialType));
}
AmmoSpecialType_t GetAmmoSpecialType(const alt::IPlayer* ptr, u32 ammoHash) {
    return static_cast<uint32_t>(ptr->GetAmmoSpecialType(ammoHash));
}
void SetAmmoFlags(alt::IPlayer* ptr, u32 ammoHash, bool ammoFlags_infiniteAmmo, bool ammoFlags_addSmokeOnExplosion, bool ammoFlags_fuse, bool ammoFlags_fixedAfterExplosion) {
    return ptr->SetAmmoFlags(ammoHash, create_ammo_flags_from_params(
        ammoFlags_infiniteAmmo,
        ammoFlags_addSmokeOnExplosion,
        ammoFlags_fuse,
        ammoFlags_fixedAfterExplosion
        ));
}
alt::AmmoFlags GetAmmoFlags(const alt::IPlayer* ptr, u32 ammoHash) {
    return ptr->GetAmmoFlags(ammoHash);
}
void SetAmmoMax(alt::IPlayer* ptr, u32 ammoHash, i32 ammoMax) {
    return ptr->SetAmmoMax(ammoHash, ammoMax);
}
i32 GetAmmoMax(const alt::IPlayer* ptr, u32 ammoHash) {
    return ptr->GetAmmoMax(ammoHash);
}
void SetAmmoMax50(alt::IPlayer* ptr, u32 ammoHash, i32 ammoMax50) {
    return ptr->SetAmmoMax50(ammoHash, ammoMax50);
}
i32 GetAmmoMax50(const alt::IPlayer* ptr, u32 ammoHash) {
    return ptr->GetAmmoMax50(ammoHash);
}
void SetAmmoMax100(alt::IPlayer* ptr, u32 ammoHash, i32 ammoMax100) {
    return ptr->SetAmmoMax100(ammoHash, ammoMax100);
}
i32 GetAmmoMax100(const alt::IPlayer* ptr, u32 ammoHash) {
    return ptr->GetAmmoMax100(ammoHash);
}
void AddDecoration(alt::IPlayer* ptr, u32 collection, u32 overlay) {
    return ptr->AddDecoration(collection, overlay);
}
void RemoveDecoration(alt::IPlayer* ptr, u32 collection, u32 overlay) {
    return ptr->RemoveDecoration(collection, overlay);
}
void ClearDecorations(alt::IPlayer* ptr) {
    return ptr->ClearDecorations();
}
std::vector<alt::CDecoration> GetDecorations(const alt::IPlayer* ptr) {
    return ptr->GetDecorations();
}

} // namespace