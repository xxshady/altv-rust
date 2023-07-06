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
bool IsConnected(const alt::IPlayer* ptr) {
    return ptr->IsConnected();
}
u32 GetPing(const alt::IPlayer* ptr) {
    return ptr->GetPing();
}
std::string GetIP(const alt::IPlayer* ptr) {
    return ptr->GetIP();
}
u64 GetSocialID(const alt::IPlayer* ptr) {
    return ptr->GetSocialID();
}
std::string GetSocialClubName(const alt::IPlayer* ptr) {
    return ptr->GetSocialClubName();
}
u64 GetHwidHash(const alt::IPlayer* ptr) {
    return ptr->GetHwidHash();
}
u64 GetHwidExHash(const alt::IPlayer* ptr) {
    return ptr->GetHwidExHash();
}
std::string GetAuthToken(const alt::IPlayer* ptr) {
    return ptr->GetAuthToken();
}
i64 GetDiscordId(const alt::IPlayer* ptr) {
    return ptr->GetDiscordId();
}
void Spawn(alt::IPlayer* ptr, f32 pos_x, f32 pos_y, f32 pos_z, u32 delayMs) {
    return ptr->Spawn({ pos_x, pos_y, pos_z }, delayMs);
}
void Despawn(alt::IPlayer* ptr) {
    return ptr->Despawn();
}
void SetModel(alt::IPlayer* ptr, u32 model) {
    return ptr->SetModel(model);
}
void SetArmour(alt::IPlayer* ptr, u16 armor) {
    return ptr->SetArmour(armor);
}
void SetMaxArmour(alt::IPlayer* ptr, u16 armor) {
    return ptr->SetMaxArmour(armor);
}
void SetCurrentWeapon(alt::IPlayer* ptr, u32 weapon) {
    return ptr->SetCurrentWeapon(weapon);
}
void SetWeaponTintIndex(alt::IPlayer* ptr, u32 weapon, u8 tintIndex) {
    return ptr->SetWeaponTintIndex(weapon, tintIndex);
}
void AddWeaponComponent(alt::IPlayer* ptr, u32 weapon, u32 component) {
    return ptr->AddWeaponComponent(weapon, component);
}
void RemoveWeaponComponent(alt::IPlayer* ptr, u32 weapon, u32 component) {
    return ptr->RemoveWeaponComponent(weapon, component);
}
void ClearBloodDamage(alt::IPlayer* ptr) {
    return ptr->ClearBloodDamage();
}
void SetHealth(alt::IPlayer* ptr, u16 health) {
    return ptr->SetHealth(health);
}
void SetMaxHealth(alt::IPlayer* ptr, u16 health) {
    return ptr->SetMaxHealth(health);
}
void GiveWeapon(alt::IPlayer* ptr, u32 weapon, i32 ammo, bool selectWeapon) {
    return ptr->GiveWeapon(weapon, ammo, selectWeapon);
}
bool RemoveWeapon(alt::IPlayer* ptr, u32 weapon) {
    return ptr->RemoveWeapon(weapon);
}
void RemoveAllWeapons(alt::IPlayer* ptr, bool removeAllAmmo) {
    return ptr->RemoveAllWeapons(removeAllAmmo);
}
void SetDateTime(alt::IPlayer* ptr, cpp_int day, cpp_int month, cpp_int year, cpp_int hour, cpp_int minute, cpp_int second) {
    return ptr->SetDateTime(day, month, year, hour, minute, second);
}
void SetWeather(alt::IPlayer* ptr, u32 weather) {
    return ptr->SetWeather(weather);
}
void Kick(alt::IPlayer* ptr, const StdStringClone reason) {
    return ptr->Kick(reason);
}
alt::Cloth GetClothes(const alt::IPlayer* ptr, u8 component) {
    return ptr->GetClothes(component);
}
bool SetClothes(alt::IPlayer* ptr, u8 component, u16 drawable, u8 texture, u8 palette) {
    return ptr->SetClothes(component, drawable, texture, palette);
}
alt::DlcCloth GetDlcClothes(const alt::IPlayer* ptr, u8 component) {
    return ptr->GetDlcClothes(component);
}
bool SetDlcClothes(alt::IPlayer* ptr, u8 component, u16 drawable, u8 texture, u8 palette, u32 dlc) {
    return ptr->SetDlcClothes(component, drawable, texture, palette, dlc);
}
alt::Prop GetProps(const alt::IPlayer* ptr, u8 component) {
    return ptr->GetProps(component);
}
bool SetProps(alt::IPlayer* ptr, u8 component, u16 drawable, u8 texture) {
    return ptr->SetProps(component, drawable, texture);
}
alt::DlcProp GetDlcProps(const alt::IPlayer* ptr, u8 component) {
    return ptr->GetDlcProps(component);
}
bool SetDlcProps(alt::IPlayer* ptr, u8 component, u8 drawable, u8 texture, u32 dlc) {
    return ptr->SetDlcProps(component, drawable, texture, dlc);
}
void ClearProps(alt::IPlayer* ptr, u8 component) {
    return ptr->ClearProps(component);
}
bool IsEntityInStreamingRange(alt::IPlayer* ptr, u16 entityId) {
    return ptr->IsEntityInStreamingRange(entityId);
}
void SetInvincible(alt::IPlayer* ptr, bool toggle) {
    return ptr->SetInvincible(toggle);
}
bool GetInvincible(const alt::IPlayer* ptr) {
    return ptr->GetInvincible();
}
void SetIntoVehicle(alt::IPlayer* ptr, alt::IVehicle* vehicle, u8 seat) {
    return ptr->SetIntoVehicle(vehicle, seat);
}
void PlayAmbientSpeech(alt::IPlayer* ptr, const StdStringClone speechName, const StdStringClone speechParam, u32 speechDictHash) {
    return ptr->PlayAmbientSpeech(speechName, speechParam, speechDictHash);
}
bool SetHeadOverlay(alt::IPlayer* ptr, u8 overlayID, u8 index, f32 opacity) {
    return ptr->SetHeadOverlay(overlayID, index, opacity);
}
bool RemoveHeadOverlay(alt::IPlayer* ptr, u8 overlayID) {
    return ptr->RemoveHeadOverlay(overlayID);
}
bool SetHeadOverlayColor(alt::IPlayer* ptr, u8 overlayID, u8 colorType, u8 colorIndex, u8 secondColorIndex) {
    return ptr->SetHeadOverlayColor(overlayID, colorType, colorIndex, secondColorIndex);
}
alt::HeadOverlay GetHeadOverlay(const alt::IPlayer* ptr, u8 overlayID) {
    return ptr->GetHeadOverlay(overlayID);
}
bool SetFaceFeature(alt::IPlayer* ptr, u8 index, f32 scale) {
    return ptr->SetFaceFeature(index, scale);
}
f32 GetFaceFeatureScale(const alt::IPlayer* ptr, u8 index) {
    return ptr->GetFaceFeatureScale(index);
}
bool RemoveFaceFeature(alt::IPlayer* ptr, u8 index) {
    return ptr->RemoveFaceFeature(index);
}
bool SetHeadBlendPaletteColor(alt::IPlayer* ptr, u8 id, u8 red, u8 green, u8 blue) {
    return ptr->SetHeadBlendPaletteColor(id, red, green, blue);
}
RGBAWrapper GetHeadBlendPaletteColor(const alt::IPlayer* ptr, u8 id) {
    auto rgba = ptr->GetHeadBlendPaletteColor(id);
    return { rgba.r, rgba.g, rgba.b, rgba.a };
}
void SetHeadBlendData(alt::IPlayer* ptr, u32 shapeFirstID, u32 shapeSecondID, u32 shapeThirdID, u32 skinFirstID, u32 skinSecondID, u32 skinThirdID, f32 shapeMix, f32 skinMix, f32 thirdMix) {
    return ptr->SetHeadBlendData(shapeFirstID, shapeSecondID, shapeThirdID, skinFirstID, skinSecondID, skinThirdID, shapeMix, skinMix, thirdMix);
}
alt::HeadBlendData GetHeadBlendData(const alt::IPlayer* ptr) {
    return ptr->GetHeadBlendData();
}
bool SetEyeColor(alt::IPlayer* ptr, i16 eyeColor) {
    return ptr->SetEyeColor(eyeColor);
}
i16 GetEyeColor(const alt::IPlayer* ptr) {
    return ptr->GetEyeColor();
}
void SetHairColor(alt::IPlayer* ptr, u8 hairColor) {
    return ptr->SetHairColor(hairColor);
}
u8 GetHairColor(const alt::IPlayer* ptr) {
    return ptr->GetHairColor();
}
void SetHairHighlightColor(alt::IPlayer* ptr, u8 hairHighlightColor) {
    return ptr->SetHairHighlightColor(hairHighlightColor);
}
u8 GetHairHighlightColor(const alt::IPlayer* ptr) {
    return ptr->GetHairHighlightColor();
}
std::vector<WeaponWrapper> GetWeapons(const alt::IPlayer* ptr) {
    auto alt_weapons = ptr->GetWeapons();
    std::vector<WeaponWrapper> weapons {};
    weapons.reserve(alt_weapons.size());
    for (const auto& w : alt_weapons) {
        weapons.push_back({ w.hash, w.tintIndex, w.components });
    }
    return weapons;
}
bool HasWeapon(const alt::IPlayer* ptr, u32 weapon) {
    return ptr->HasWeapon(weapon);
}
bool HasLocalMetaData(const alt::IPlayer* ptr, const StdStringClone key) {
    return ptr->HasLocalMetaData(key);
}
void SetLocalMetaData(alt::IPlayer* ptr, const StdStringClone key, MValueMutWrapper val) {
    return ptr->SetLocalMetaData(key, val.ptr);
}
ConstMValueWrapper GetLocalMetaData(const alt::IPlayer* ptr, const StdStringClone key) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetLocalMetaData(key);
    return wrapper;
}
void DeleteLocalMetaData(alt::IPlayer* ptr, const StdStringClone key) {
    return ptr->DeleteLocalMetaData(key);
}
std::vector<std::string> GetLocalMetaDataKeys(const alt::IPlayer* ptr) {
    return ptr->GetLocalMetaDataKeys();
}
u32 GetInteriorLocation(const alt::IPlayer* ptr) {
    return ptr->GetInteriorLocation();
}
u32 GetLastDamagedBodyPart(const alt::IPlayer* ptr) {
    return ptr->GetLastDamagedBodyPart();
}
void SetLastDamagedBodyPart(alt::IPlayer* ptr, u32 bodyPart) {
    return ptr->SetLastDamagedBodyPart(bodyPart);
}
void SetSendNames(alt::IPlayer* ptr, bool state) {
    return ptr->SetSendNames(state);
}
bool GetSendNames(const alt::IPlayer* ptr) {
    return ptr->GetSendNames();
}
void PlayAnimation(alt::IPlayer* ptr, const StdStringClone animDict, const StdStringClone animName, f32 blendInSpeed, f32 blendOutSpeed, cpp_int duration, cpp_int flags, f32 playbackRate, bool lockX, bool lockY, bool lockZ) {
    return ptr->PlayAnimation(animDict, animName, blendInSpeed, blendOutSpeed, duration, flags, playbackRate, lockX, lockY, lockZ);
}
void ClearTasks(alt::IPlayer* ptr) {
    return ptr->ClearTasks();
}
std::string GetCloudAuthHash(const alt::IPlayer* ptr) {
    return ptr->GetCloudAuthHash();
}
std::vector<StreamedEntityWrapper> GetStreamedEntities(const alt::IPlayer* ptr) {
    auto alt_vec = ptr->GetStreamedEntities();
    std::vector<StreamedEntityWrapper> vec {};
    vec.reserve(alt_vec.size());
    for (const auto& pair : alt_vec) {
        vec.push_back({ pair.first, pair.second });
    }
    return vec;
}
void SetAmmo(alt::IPlayer* ptr, u32 ammoHash, u16 ammo) {
    return ptr->SetAmmo(ammoHash, ammo);
}
u16 GetAmmo(const alt::IPlayer* ptr, u32 ammoHash) {
    return ptr->GetAmmo(ammoHash);
}
void SetWeaponAmmo(alt::IPlayer* ptr, u32 weaponHash, u16 ammo) {
    return ptr->SetWeaponAmmo(weaponHash, ammo);
}
u16 GetWeaponAmmo(const alt::IPlayer* ptr, u32 weaponHash) {
    return ptr->GetWeaponAmmo(weaponHash);
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

} // namespace