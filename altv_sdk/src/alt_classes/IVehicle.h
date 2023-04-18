#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IVehicle {

alt::IPlayer* GetDriver(const alt::IVehicle* ptr) {
    return ptr->GetDriver();
}
bool IsDestroyed(const alt::IVehicle* ptr) {
    return ptr->IsDestroyed();
}
u8 GetMod(const alt::IVehicle* ptr, u8 category) {
    return ptr->GetMod(category);
}
u8 GetModsCount(const alt::IVehicle* ptr, u8 category) {
    return ptr->GetModsCount(category);
}
u8 GetModKitsCount(const alt::IVehicle* ptr) {
    return ptr->GetModKitsCount();
}
u8 GetModKit(const alt::IVehicle* ptr) {
    return ptr->GetModKit();
}
bool IsPrimaryColorRGB(const alt::IVehicle* ptr) {
    return ptr->IsPrimaryColorRGB();
}
u8 GetPrimaryColor(const alt::IVehicle* ptr) {
    return ptr->GetPrimaryColor();
}
RGBAWrapper GetPrimaryColorRGB(const alt::IVehicle* ptr) {
    auto rgba = ptr->GetPrimaryColorRGB();
    return { rgba.r, rgba.g, rgba.b, rgba.a };
}
bool IsSecondaryColorRGB(const alt::IVehicle* ptr) {
    return ptr->IsSecondaryColorRGB();
}
u8 GetSecondaryColor(const alt::IVehicle* ptr) {
    return ptr->GetSecondaryColor();
}
RGBAWrapper GetSecondaryColorRGB(const alt::IVehicle* ptr) {
    auto rgba = ptr->GetSecondaryColorRGB();
    return { rgba.r, rgba.g, rgba.b, rgba.a };
}
u8 GetPearlColor(const alt::IVehicle* ptr) {
    return ptr->GetPearlColor();
}
u8 GetWheelColor(const alt::IVehicle* ptr) {
    return ptr->GetWheelColor();
}
u8 GetInteriorColor(const alt::IVehicle* ptr) {
    return ptr->GetInteriorColor();
}
u8 GetDashboardColor(const alt::IVehicle* ptr) {
    return ptr->GetDashboardColor();
}
bool IsTireSmokeColorCustom(const alt::IVehicle* ptr) {
    return ptr->IsTireSmokeColorCustom();
}
RGBAWrapper GetTireSmokeColor(const alt::IVehicle* ptr) {
    auto rgba = ptr->GetTireSmokeColor();
    return { rgba.r, rgba.g, rgba.b, rgba.a };
}
u8 GetWheelType(const alt::IVehicle* ptr) {
    return ptr->GetWheelType();
}
u8 GetWheelVariation(const alt::IVehicle* ptr) {
    return ptr->GetWheelVariation();
}
u8 GetRearWheelVariation(const alt::IVehicle* ptr) {
    return ptr->GetRearWheelVariation();
}
bool GetCustomTires(const alt::IVehicle* ptr) {
    return ptr->GetCustomTires();
}
u8 GetSpecialDarkness(const alt::IVehicle* ptr) {
    return ptr->GetSpecialDarkness();
}
u32 GetNumberplateIndex(const alt::IVehicle* ptr) {
    return ptr->GetNumberplateIndex();
}
std::string GetNumberplateText(const alt::IVehicle* ptr) {
    return ptr->GetNumberplateText();
}
u8 GetWindowTint(const alt::IVehicle* ptr) {
    return ptr->GetWindowTint();
}
u8 GetDirtLevel(const alt::IVehicle* ptr) {
    return ptr->GetDirtLevel();
}
bool IsExtraOn(const alt::IVehicle* ptr, u8 extraID) {
    return ptr->IsExtraOn(extraID);
}
bool IsNeonActive(const alt::IVehicle* ptr) {
    return ptr->IsNeonActive();
}
void GetNeonActive(const alt::IVehicle* ptr, bool* left, bool* right, bool* front, bool* back) {
    return ptr->GetNeonActive(left, right, front, back);
}
RGBAWrapper GetNeonColor(const alt::IVehicle* ptr) {
    auto rgba = ptr->GetNeonColor();
    return { rgba.r, rgba.g, rgba.b, rgba.a };
}
u8 GetLivery(const alt::IVehicle* ptr) {
    return ptr->GetLivery();
}
u8 GetRoofLivery(const alt::IVehicle* ptr) {
    return ptr->GetRoofLivery();
}
std::string GetAppearanceDataBase64(const alt::IVehicle* ptr) {
    return ptr->GetAppearanceDataBase64();
}
bool IsEngineOn(const alt::IVehicle* ptr) {
    return ptr->IsEngineOn();
}
bool IsHandbrakeActive(const alt::IVehicle* ptr) {
    return ptr->IsHandbrakeActive();
}
u8 GetHeadlightColor(const alt::IVehicle* ptr) {
    return ptr->GetHeadlightColor();
}
u32 GetRadioStationIndex(const alt::IVehicle* ptr) {
    return ptr->GetRadioStationIndex();
}
bool IsSirenActive(const alt::IVehicle* ptr) {
    return ptr->IsSirenActive();
}
u8 GetLockState(const alt::IVehicle* ptr) {
    return ptr->GetLockState();
}
u8 GetDoorState(const alt::IVehicle* ptr, u8 doorId) {
    return ptr->GetDoorState(doorId);
}
bool IsWindowOpened(const alt::IVehicle* ptr, u8 windowId) {
    return ptr->IsWindowOpened(windowId);
}
bool IsDaylightOn(const alt::IVehicle* ptr) {
    return ptr->IsDaylightOn();
}
bool IsNightlightOn(const alt::IVehicle* ptr) {
    return ptr->IsNightlightOn();
}
u8 GetRoofState(const alt::IVehicle* ptr) {
    return ptr->GetRoofState();
}
bool IsFlamethrowerActive(const alt::IVehicle* ptr) {
    return ptr->IsFlamethrowerActive();
}
f32 GetLightsMultiplier(const alt::IVehicle* ptr) {
    return ptr->GetLightsMultiplier();
}
std::string GetGameStateBase64(const alt::IVehicle* ptr) {
    return ptr->GetGameStateBase64();
}
i32 GetEngineHealth(const alt::IVehicle* ptr) {
    return ptr->GetEngineHealth();
}
i32 GetPetrolTankHealth(const alt::IVehicle* ptr) {
    return ptr->GetPetrolTankHealth();
}
u8 GetWheelsCount(const alt::IVehicle* ptr) {
    return ptr->GetWheelsCount();
}
bool IsWheelBurst(alt::IVehicle* ptr, u8 wheelId) {
    return ptr->IsWheelBurst(wheelId);
}
bool DoesWheelHasTire(alt::IVehicle* ptr, u8 wheelId) {
    return ptr->DoesWheelHasTire(wheelId);
}
bool IsWheelDetached(alt::IVehicle* ptr, u8 wheelId) {
    return ptr->IsWheelDetached(wheelId);
}
bool IsWheelOnFire(alt::IVehicle* ptr, u8 wheelId) {
    return ptr->IsWheelOnFire(wheelId);
}
f32 GetWheelHealth(alt::IVehicle* ptr, u8 wheelId) {
    return ptr->GetWheelHealth(wheelId);
}
u8 GetRepairsCount(const alt::IVehicle* ptr) {
    return ptr->GetRepairsCount();
}
u32 GetBodyHealth(const alt::IVehicle* ptr) {
    return ptr->GetBodyHealth();
}
u32 GetBodyAdditionalHealth(const alt::IVehicle* ptr) {
    return ptr->GetBodyAdditionalHealth();
}
std::string GetHealthDataBase64(const alt::IVehicle* ptr) {
    return ptr->GetHealthDataBase64();
}
u8 GetPartDamageLevel(alt::IVehicle* ptr, u8 partId) {
    return ptr->GetPartDamageLevel(partId);
}
u8 GetPartBulletHoles(alt::IVehicle* ptr, u8 partId) {
    return ptr->GetPartBulletHoles(partId);
}
bool IsLightDamaged(alt::IVehicle* ptr, u8 lightId) {
    return ptr->IsLightDamaged(lightId);
}
bool IsWindowDamaged(alt::IVehicle* ptr, u8 windowId) {
    return ptr->IsWindowDamaged(windowId);
}
bool IsSpecialLightDamaged(alt::IVehicle* ptr, u8 specialLightId) {
    return ptr->IsSpecialLightDamaged(specialLightId);
}
bool HasArmoredWindows(const alt::IVehicle* ptr) {
    return ptr->HasArmoredWindows();
}
f32 GetArmoredWindowHealth(alt::IVehicle* ptr, u8 windowId) {
    return ptr->GetArmoredWindowHealth(windowId);
}
u8 GetArmoredWindowShootCount(alt::IVehicle* ptr, u8 windowId) {
    return ptr->GetArmoredWindowShootCount(windowId);
}
u8 GetBumperDamageLevel(alt::IVehicle* ptr, u8 bumperId) {
    return ptr->GetBumperDamageLevel(bumperId);
}
std::string GetDamageDataBase64(const alt::IVehicle* ptr) {
    return ptr->GetDamageDataBase64();
}
bool IsManualEngineControl(const alt::IVehicle* ptr) {
    return ptr->IsManualEngineControl();
}
std::string GetScriptDataBase64(const alt::IVehicle* ptr) {
    return ptr->GetScriptDataBase64();
}
void ToggleExtra(alt::IVehicle* ptr, u8 extraID, bool state) {
    return ptr->ToggleExtra(extraID, state);
}
Vector3Wrapper GetVelocity(const alt::IVehicle* ptr) {
    auto vector3 = ptr->GetVelocity();
    return { vector3[0], vector3[1], vector3[2] };
}
void SetFixed(alt::IVehicle* ptr) {
    return ptr->SetFixed();
}
bool SetMod(alt::IVehicle* ptr, u8 category, u8 id) {
    return ptr->SetMod(category, id);
}
bool SetModKit(alt::IVehicle* ptr, u8 id) {
    return ptr->SetModKit(id);
}
void SetPrimaryColor(alt::IVehicle* ptr, u8 color) {
    return ptr->SetPrimaryColor(color);
}
void SetPrimaryColorRGB(alt::IVehicle* ptr, u8 color_r, u8 color_g, u8 color_b, u8 color_a) {
    return ptr->SetPrimaryColorRGB({ color_r, color_g, color_b, color_a });
}
void SetSecondaryColor(alt::IVehicle* ptr, u8 color) {
    return ptr->SetSecondaryColor(color);
}
void SetSecondaryColorRGB(alt::IVehicle* ptr, u8 color_r, u8 color_g, u8 color_b, u8 color_a) {
    return ptr->SetSecondaryColorRGB({ color_r, color_g, color_b, color_a });
}
void SetPearlColor(alt::IVehicle* ptr, u8 color) {
    return ptr->SetPearlColor(color);
}
void SetWheelColor(alt::IVehicle* ptr, u8 color) {
    return ptr->SetWheelColor(color);
}
void SetInteriorColor(alt::IVehicle* ptr, u8 color) {
    return ptr->SetInteriorColor(color);
}
void SetDashboardColor(alt::IVehicle* ptr, u8 color) {
    return ptr->SetDashboardColor(color);
}
void SetTireSmokeColor(alt::IVehicle* ptr, u8 color_r, u8 color_g, u8 color_b, u8 color_a) {
    return ptr->SetTireSmokeColor({ color_r, color_g, color_b, color_a });
}
void SetWheels(alt::IVehicle* ptr, u8 type, u8 variation) {
    return ptr->SetWheels(type, variation);
}
void SetRearWheels(alt::IVehicle* ptr, u8 variation) {
    return ptr->SetRearWheels(variation);
}
void SetCustomTires(alt::IVehicle* ptr, bool state) {
    return ptr->SetCustomTires(state);
}
void SetSpecialDarkness(alt::IVehicle* ptr, u8 value) {
    return ptr->SetSpecialDarkness(value);
}
void SetNumberplateIndex(alt::IVehicle* ptr, u32 index) {
    return ptr->SetNumberplateIndex(index);
}
void SetNumberplateText(alt::IVehicle* ptr, const StdStringClone text) {
    return ptr->SetNumberplateText(text);
}
void SetWindowTint(alt::IVehicle* ptr, u8 tint) {
    return ptr->SetWindowTint(tint);
}
void SetDirtLevel(alt::IVehicle* ptr, u8 level) {
    return ptr->SetDirtLevel(level);
}
void SetNeonActive(alt::IVehicle* ptr, bool left, bool right, bool front, bool back) {
    return ptr->SetNeonActive(left, right, front, back);
}
void SetNeonColor(alt::IVehicle* ptr, u8 color_r, u8 color_g, u8 color_b, u8 color_a) {
    return ptr->SetNeonColor({ color_r, color_g, color_b, color_a });
}
void SetLivery(alt::IVehicle* ptr, u8 livery) {
    return ptr->SetLivery(livery);
}
void SetRoofLivery(alt::IVehicle* ptr, u8 roofLivery) {
    return ptr->SetRoofLivery(roofLivery);
}
void LoadAppearanceDataFromBase64(alt::IVehicle* ptr, const StdStringClone base64) {
    return ptr->LoadAppearanceDataFromBase64(base64);
}
void SetEngineOn(alt::IVehicle* ptr, bool state) {
    return ptr->SetEngineOn(state);
}
void SetHeadlightColor(alt::IVehicle* ptr, u8 color) {
    return ptr->SetHeadlightColor(color);
}
void SetRadioStationIndex(alt::IVehicle* ptr, u32 stationIndex) {
    return ptr->SetRadioStationIndex(stationIndex);
}
void SetSirenActive(alt::IVehicle* ptr, bool state) {
    return ptr->SetSirenActive(state);
}
void SetLockState(alt::IVehicle* ptr, u8 state) {
    return ptr->SetLockState(state);
}
void SetDoorState(alt::IVehicle* ptr, u8 doorId, u8 state) {
    return ptr->SetDoorState(doorId, state);
}
void SetWindowOpened(alt::IVehicle* ptr, u8 windowId, bool state) {
    return ptr->SetWindowOpened(windowId, state);
}
void SetRoofState(alt::IVehicle* ptr, u8 state) {
    return ptr->SetRoofState(state);
}
void SetLightsMultiplier(alt::IVehicle* ptr, f32 multiplier) {
    return ptr->SetLightsMultiplier(multiplier);
}
void SetEngineHealth(alt::IVehicle* ptr, i32 health) {
    return ptr->SetEngineHealth(health);
}
void SetPetrolTankHealth(alt::IVehicle* ptr, i32 health) {
    return ptr->SetPetrolTankHealth(health);
}
void SetWheelBurst(alt::IVehicle* ptr, u8 wheelId, bool state) {
    return ptr->SetWheelBurst(wheelId, state);
}
void SetWheelHasTire(alt::IVehicle* ptr, u8 wheelId, bool state) {
    return ptr->SetWheelHasTire(wheelId, state);
}
void SetWheelDetached(alt::IVehicle* ptr, u8 wheelId, bool state) {
    return ptr->SetWheelDetached(wheelId, state);
}
void SetWheelOnFire(alt::IVehicle* ptr, u8 wheelId, bool state) {
    return ptr->SetWheelOnFire(wheelId, state);
}
void SetWheelHealth(alt::IVehicle* ptr, u8 wheelId, f32 health) {
    return ptr->SetWheelHealth(wheelId, health);
}
void SetWheelFixed(alt::IVehicle* ptr, u8 wheelId) {
    return ptr->SetWheelFixed(wheelId);
}
void SetBodyHealth(alt::IVehicle* ptr, u32 health) {
    return ptr->SetBodyHealth(health);
}
void SetBodyAdditionalHealth(alt::IVehicle* ptr, u32 health) {
    return ptr->SetBodyAdditionalHealth(health);
}
void SetPartDamageLevel(alt::IVehicle* ptr, u8 partId, u8 damage) {
    return ptr->SetPartDamageLevel(partId, damage);
}
void SetPartBulletHoles(alt::IVehicle* ptr, u8 partId, u8 shootsCount) {
    return ptr->SetPartBulletHoles(partId, shootsCount);
}
void SetLightDamaged(alt::IVehicle* ptr, u8 lightId, bool isDamaged) {
    return ptr->SetLightDamaged(lightId, isDamaged);
}
void SetWindowDamaged(alt::IVehicle* ptr, u8 windowId, bool isDamaged) {
    return ptr->SetWindowDamaged(windowId, isDamaged);
}
void SetSpecialLightDamaged(alt::IVehicle* ptr, u8 specialLightId, bool isDamaged) {
    return ptr->SetSpecialLightDamaged(specialLightId, isDamaged);
}
void SetArmoredWindowHealth(alt::IVehicle* ptr, u8 windowId, f32 health) {
    return ptr->SetArmoredWindowHealth(windowId, health);
}
void SetArmoredWindowShootCount(alt::IVehicle* ptr, u8 windowId, u8 count) {
    return ptr->SetArmoredWindowShootCount(windowId, count);
}
void SetBumperDamageLevel(alt::IVehicle* ptr, u8 bumperId, u8 damageLevel) {
    return ptr->SetBumperDamageLevel(bumperId, damageLevel);
}
void SetManualEngineControl(alt::IVehicle* ptr, bool state) {
    return ptr->SetManualEngineControl(state);
}
void LoadDamageDataFromBase64(alt::IVehicle* ptr, const StdStringClone base64) {
    return ptr->LoadDamageDataFromBase64(base64);
}
void LoadScriptDataFromBase64(alt::IVehicle* ptr, const StdStringClone base64) {
    return ptr->LoadScriptDataFromBase64(base64);
}
void LoadGameStateFromBase64(alt::IVehicle* ptr, const StdStringClone base64) {
    return ptr->LoadGameStateFromBase64(base64);
}
void LoadHealthDataFromBase64(alt::IVehicle* ptr, const StdStringClone base64) {
    return ptr->LoadHealthDataFromBase64(base64);
}
alt::IVehicle* GetAttached(const alt::IVehicle* ptr) {
    return ptr->GetAttached();
}
alt::IVehicle* GetAttachedTo(const alt::IVehicle* ptr) {
    return ptr->GetAttachedTo();
}
bool IsDriftMode(const alt::IVehicle* ptr) {
    return ptr->IsDriftMode();
}
void SetDriftMode(alt::IVehicle* ptr, bool state) {
    return ptr->SetDriftMode(state);
}
bool IsTrainMissionTrain(const alt::IVehicle* ptr) {
    return ptr->IsTrainMissionTrain();
}
void SetTrainMissionTrain(alt::IVehicle* ptr, bool value) {
    return ptr->SetTrainMissionTrain(value);
}
i8 GetTrainTrackId(const alt::IVehicle* ptr) {
    return ptr->GetTrainTrackId();
}
void SetTrainTrackId(alt::IVehicle* ptr, i8 trackId) {
    return ptr->SetTrainTrackId(trackId);
}
alt::IVehicle* GetTrainEngineId(const alt::IVehicle* ptr) {
    return ptr->GetTrainEngineId();
}
void SetTrainEngineId(alt::IVehicle* ptr, alt::IVehicle* vehicle) {
    return ptr->SetTrainEngineId(vehicle);
}
i8 GetTrainConfigIndex(const alt::IVehicle* ptr) {
    return ptr->GetTrainConfigIndex();
}
void SetTrainConfigIndex(alt::IVehicle* ptr, i8 trainConfigIndex) {
    return ptr->SetTrainConfigIndex(trainConfigIndex);
}
f32 GetTrainDistanceFromEngine(const alt::IVehicle* ptr) {
    return ptr->GetTrainDistanceFromEngine();
}
void SetTrainDistanceFromEngine(alt::IVehicle* ptr, f32 distanceFromEngine) {
    return ptr->SetTrainDistanceFromEngine(distanceFromEngine);
}
bool IsTrainEngine(const alt::IVehicle* ptr) {
    return ptr->IsTrainEngine();
}
void SetTrainIsEngine(alt::IVehicle* ptr, bool isEngine) {
    return ptr->SetTrainIsEngine(isEngine);
}
bool IsTrainCaboose(const alt::IVehicle* ptr) {
    return ptr->IsTrainCaboose();
}
void SetTrainIsCaboose(alt::IVehicle* ptr, bool isCaboose) {
    return ptr->SetTrainIsCaboose(isCaboose);
}
bool GetTrainDirection(const alt::IVehicle* ptr) {
    return ptr->GetTrainDirection();
}
void SetTrainDirection(alt::IVehicle* ptr, bool direction) {
    return ptr->SetTrainDirection(direction);
}
bool HasTrainPassengerCarriages(const alt::IVehicle* ptr) {
    return ptr->HasTrainPassengerCarriages();
}
void SetTrainHasPassengerCarriages(alt::IVehicle* ptr, bool hasPassengerCarriages) {
    return ptr->SetTrainHasPassengerCarriages(hasPassengerCarriages);
}
bool GetTrainRenderDerailed(const alt::IVehicle* ptr) {
    return ptr->GetTrainRenderDerailed();
}
void SetTrainRenderDerailed(alt::IVehicle* ptr, bool renderDerailed) {
    return ptr->SetTrainRenderDerailed(renderDerailed);
}
bool GetTrainForceDoorsOpen(const alt::IVehicle* ptr) {
    return ptr->GetTrainForceDoorsOpen();
}
void SetTrainForceDoorsOpen(alt::IVehicle* ptr, bool forceDoorsOpen) {
    return ptr->SetTrainForceDoorsOpen(forceDoorsOpen);
}
f32 GetTrainCruiseSpeed(const alt::IVehicle* ptr) {
    return ptr->GetTrainCruiseSpeed();
}
void SetTrainCruiseSpeed(alt::IVehicle* ptr, f32 cruiseSpeed) {
    return ptr->SetTrainCruiseSpeed(cruiseSpeed);
}
i8 GetTrainCarriageConfigIndex(const alt::IVehicle* ptr) {
    return ptr->GetTrainCarriageConfigIndex();
}
void SetTrainCarriageConfigIndex(alt::IVehicle* ptr, i8 carriageConfigIndex) {
    return ptr->SetTrainCarriageConfigIndex(carriageConfigIndex);
}
alt::IVehicle* GetTrainLinkedToBackwardId(const alt::IVehicle* ptr) {
    return ptr->GetTrainLinkedToBackwardId();
}
void SetTrainLinkedToBackwardId(alt::IVehicle* ptr, alt::IVehicle* vehicle) {
    return ptr->SetTrainLinkedToBackwardId(vehicle);
}
alt::IVehicle* GetTrainLinkedToForwardId(const alt::IVehicle* ptr) {
    return ptr->GetTrainLinkedToForwardId();
}
void SetTrainLinkedToForwardId(alt::IVehicle* ptr, alt::IVehicle* vehicle) {
    return ptr->SetTrainLinkedToForwardId(vehicle);
}
void SetTrainUnk1(alt::IVehicle* ptr, bool unk1) {
    return ptr->SetTrainUnk1(unk1);
}
bool GetTrainUnk1(const alt::IVehicle* ptr) {
    return ptr->GetTrainUnk1();
}
void SetTrainUnk2(alt::IVehicle* ptr, bool unk2) {
    return ptr->SetTrainUnk2(unk2);
}
bool GetTrainUnk2(const alt::IVehicle* ptr) {
    return ptr->GetTrainUnk2();
}
void SetTrainUnk3(alt::IVehicle* ptr, bool unk3) {
    return ptr->SetTrainUnk3(unk3);
}
bool GetTrainUnk3(const alt::IVehicle* ptr) {
    return ptr->GetTrainUnk3();
}
bool IsBoatAnchorActive(const alt::IVehicle* ptr) {
    return ptr->IsBoatAnchorActive();
}
void SetBoatAnchorActive(alt::IVehicle* ptr, bool state) {
    return ptr->SetBoatAnchorActive(state);
}
bool SetSearchLight(alt::IVehicle* ptr, bool state, alt::IEntity* spottedEntity) {
    return ptr->SetSearchLight(state, spottedEntity);
}
u8 GetLightState(const alt::IVehicle* ptr) {
    return ptr->GetLightState();
}
void SetLightState(alt::IVehicle* ptr, u8 state) {
    return ptr->SetLightState(state);
}
bool HasTimedExplosion(const alt::IVehicle* ptr) {
    return ptr->HasTimedExplosion();
}
alt::IPlayer* GetTimedExplosionCulprit(const alt::IVehicle* ptr) {
    return ptr->GetTimedExplosionCulprit();
}
u32 GetTimedExplosionTime(const alt::IVehicle* ptr) {
    return ptr->GetTimedExplosionTime();
}
void SetTimedExplosion(alt::IVehicle* ptr, bool state, alt::IPlayer* culprit, u32 time) {
    return ptr->SetTimedExplosion(state, culprit, time);
}
bool IsTowingDisabled(const alt::IVehicle* ptr) {
    return ptr->IsTowingDisabled();
}
void SetDisableTowing(alt::IVehicle* ptr, bool state) {
    return ptr->SetDisableTowing(state);
}
f32 GetRocketRefuelSpeed(const alt::IVehicle* ptr) {
    return ptr->GetRocketRefuelSpeed();
}
void SetRocketRefuelSpeed(alt::IVehicle* ptr, f32 rocketRefuelSpeed) {
    return ptr->SetRocketRefuelSpeed(rocketRefuelSpeed);
}
u32 GetCounterMeasureCount(const alt::IVehicle* ptr) {
    return ptr->GetCounterMeasureCount();
}
void SetCounterMeasureCount(alt::IVehicle* ptr, u32 counterMeasureCount) {
    return ptr->SetCounterMeasureCount(counterMeasureCount);
}
f32 GetScriptMaxSpeed(const alt::IVehicle* ptr) {
    return ptr->GetScriptMaxSpeed();
}
void SetScriptMaxSpeed(alt::IVehicle* ptr, f32 scriptMaxSpeed) {
    return ptr->SetScriptMaxSpeed(scriptMaxSpeed);
}
i32 GetWeaponCapacity(const alt::IVehicle* ptr, u8 index) {
    return ptr->GetWeaponCapacity(index);
}
void SetWeaponCapacity(alt::IVehicle* ptr, u8 index, i32 state) {
    return ptr->SetWeaponCapacity(index, state);
}
bool GetHybridExtraActive(const alt::IVehicle* ptr) {
    return ptr->GetHybridExtraActive();
}
void SetHybridExtraActive(alt::IVehicle* ptr, bool state) {
    return ptr->SetHybridExtraActive(state);
}
u8 GetHybridExtraState(const alt::IVehicle* ptr) {
    return ptr->GetHybridExtraState();
}
void SetHybridExtraState(alt::IVehicle* ptr, u8 state) {
    return ptr->SetHybridExtraState(state);
}
alt::Quaternion GetQuaternion(const alt::IVehicle* ptr) {
    return ptr->GetQuaternion();
}
void SetQuaternion(alt::IVehicle* ptr, f32 quaternion_x, f32 quaternion_y, f32 quaternion_z, f32 quaternion_w) {
    return ptr->SetQuaternion({ quaternion_x, quaternion_y, quaternion_z, quaternion_w });
}

} // namespace