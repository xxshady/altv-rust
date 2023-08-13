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
f32 GetSteeringAngle(const alt::IVehicle* ptr) {
    return ptr->GetSteeringAngle();
}
f32 GetWheelSpeed(const alt::IVehicle* ptr) {
    return ptr->GetWheelSpeed();
}
u16 GetCurrentGear(const alt::IVehicle* ptr) {
    return ptr->GetCurrentGear();
}
f32 GetCurrentRPM(const alt::IVehicle* ptr) {
    return ptr->GetCurrentRPM();
}
Vector3Wrapper GetSpeedVector(const alt::IVehicle* ptr) {
    auto vector3 = ptr->GetSpeedVector();
    return { vector3[0], vector3[1], vector3[2] };
}
u16 GetMaxGear(const alt::IVehicle* ptr) {
    return ptr->GetMaxGear();
}
void SetCurrentGear(alt::IVehicle* ptr, u16 currentGear) {
    return ptr->SetCurrentGear(currentGear);
}
void SetMaxGear(alt::IVehicle* ptr, u16 gearMax) {
    return ptr->SetMaxGear(gearMax);
}
void SetCurrentRPM(alt::IVehicle* ptr, f32 rpm) {
    return ptr->SetCurrentRPM(rpm);
}
bool IsHandlingModified(const alt::IVehicle* ptr) {
    return ptr->IsHandlingModified();
}
void ResetHandling(alt::IVehicle* ptr) {
    return ptr->ResetHandling();
}
void ReplaceHandling(alt::IVehicle* ptr) {
    return ptr->ReplaceHandling();
}
u8 GetLightsIndicator(const alt::IVehicle* ptr) {
    return ptr->GetLightsIndicator();
}
void SetLightsIndicator(alt::IVehicle* ptr, u8 lightsIndicatorFlag) {
    return ptr->SetLightsIndicator(lightsIndicatorFlag);
}
u8 GetSeatCount(const alt::IVehicle* ptr) {
    return ptr->GetSeatCount();
}
u8 GetOccupiedSeatsCount(const alt::IVehicle* ptr) {
    return ptr->GetOccupiedSeatsCount();
}
void ToggleTaxiLight(alt::IVehicle* ptr, bool state) {
    return ptr->ToggleTaxiLight(state);
}
bool IsTaxiLightOn(const alt::IVehicle* ptr) {
    return ptr->IsTaxiLightOn();
}
f32 GetWheelCamber(const alt::IVehicle* ptr, u8 wheel) {
    return ptr->GetWheelCamber(wheel);
}
void SetWheelCamber(alt::IVehicle* ptr, u8 wheel, f32 value) {
    return ptr->SetWheelCamber(wheel, value);
}
f32 GetWheelTrackWidth(const alt::IVehicle* ptr, u8 wheel) {
    return ptr->GetWheelTrackWidth(wheel);
}
void SetWheelTrackWidth(alt::IVehicle* ptr, u8 wheel, f32 value) {
    return ptr->SetWheelTrackWidth(wheel, value);
}
f32 GetWheelHeight(const alt::IVehicle* ptr, u8 wheel) {
    return ptr->GetWheelHeight(wheel);
}
void SetWheelHeight(alt::IVehicle* ptr, u8 wheel, f32 value) {
    return ptr->SetWheelHeight(wheel, value);
}
f32 GetWheelTyreRadius(const alt::IVehicle* ptr, u8 wheel) {
    return ptr->GetWheelTyreRadius(wheel);
}
void SetWheelTyreRadius(alt::IVehicle* ptr, u8 wheel, f32 value) {
    return ptr->SetWheelTyreRadius(wheel, value);
}
f32 GetWheelRimRadius(const alt::IVehicle* ptr, u8 wheel) {
    return ptr->GetWheelRimRadius(wheel);
}
void SetWheelRimRadius(alt::IVehicle* ptr, u8 wheel, f32 value) {
    return ptr->SetWheelRimRadius(wheel, value);
}
f32 GetWheelTyreWidth(const alt::IVehicle* ptr, u8 wheel) {
    return ptr->GetWheelTyreWidth(wheel);
}
void SetWheelTyreWidth(alt::IVehicle* ptr, u8 wheel, f32 value) {
    return ptr->SetWheelTyreWidth(wheel, value);
}
u32 GetWheelSurfaceMaterial(const alt::IVehicle* ptr, u8 wheel) {
    return ptr->GetWheelSurfaceMaterial(wheel);
}
f32 GetEngineTemperature(const alt::IVehicle* ptr) {
    return ptr->GetEngineTemperature();
}
void SetEngineTemperature(alt::IVehicle* ptr, f32 value) {
    return ptr->SetEngineTemperature(value);
}
f32 GetFuelLevel(const alt::IVehicle* ptr) {
    return ptr->GetFuelLevel();
}
void SetFuelLevel(alt::IVehicle* ptr, f32 value) {
    return ptr->SetFuelLevel(value);
}
f32 GetOilLevel(const alt::IVehicle* ptr) {
    return ptr->GetOilLevel();
}
void SetOilLevel(alt::IVehicle* ptr, f32 value) {
    return ptr->SetOilLevel(value);
}
bool GetEngineLightState(const alt::IVehicle* ptr) {
    return ptr->GetEngineLightState();
}
void SetEngineLightState(alt::IVehicle* ptr, bool state) {
    return ptr->SetEngineLightState(state);
}
bool GetAbsLightState(const alt::IVehicle* ptr) {
    return ptr->GetAbsLightState();
}
void SetAbsLightState(alt::IVehicle* ptr, bool state) {
    return ptr->SetAbsLightState(state);
}
bool GetPetrolLightState(const alt::IVehicle* ptr) {
    return ptr->GetPetrolLightState();
}
void SetPetrolLightState(alt::IVehicle* ptr, bool state) {
    return ptr->SetPetrolLightState(state);
}
bool GetOilLightState(const alt::IVehicle* ptr) {
    return ptr->GetOilLightState();
}
void SetOilLightState(alt::IVehicle* ptr, bool state) {
    return ptr->SetOilLightState(state);
}
bool GetBatteryLightState(const alt::IVehicle* ptr) {
    return ptr->GetBatteryLightState();
}
void SetBatteryLightState(alt::IVehicle* ptr, bool state) {
    return ptr->SetBatteryLightState(state);
}
void ResetDashboardLights(alt::IVehicle* ptr) {
    return ptr->ResetDashboardLights();
}
f32 GetSuspensionHeight(const alt::IVehicle* ptr) {
    return ptr->GetSuspensionHeight();
}
void SetSuspensionHeight(alt::IVehicle* ptr, f32 value) {
    return ptr->SetSuspensionHeight(value);
}

} // namespace