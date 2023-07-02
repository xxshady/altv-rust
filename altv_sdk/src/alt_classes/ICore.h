#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace ICore {

std::string GetVersion() {
    return alt::ICore::Instance().GetVersion();
}
std::string GetBranch() {
    return alt::ICore::Instance().GetBranch();
}
void LogInfo(const StdStringClone str, alt::IResource* resource) {
    return alt::ICore::Instance().LogInfo(str, resource);
}
void LogDebug(const StdStringClone str, alt::IResource* resource) {
    return alt::ICore::Instance().LogDebug(str, resource);
}
void LogWarning(const StdStringClone str, alt::IResource* resource) {
    return alt::ICore::Instance().LogWarning(str, resource);
}
void LogError(const StdStringClone str, alt::IResource* resource) {
    return alt::ICore::Instance().LogError(str, resource);
}
void LogColored(const StdStringClone str, alt::IResource* resource) {
    return alt::ICore::Instance().LogColored(str, resource);
}
alt::IVirtualEntity* CreateVirtualEntity(alt::IVirtualEntityGroup* group, f32 pos_x, f32 pos_y, f32 pos_z, u32 streamingDistance, MValueUnorderedMapWrapper data) {
    return alt::ICore::Instance().CreateVirtualEntity(group, { pos_x, pos_y, pos_z }, streamingDistance, data.value);
}
alt::IVirtualEntityGroup* CreateVirtualEntityGroup(u32 maxEntitiesInStream) {
    return alt::ICore::Instance().CreateVirtualEntityGroup(maxEntitiesInStream);
}
alt::IColShape* CreateColShapeCylinder(f32 pos_x, f32 pos_y, f32 pos_z, f32 radius, f32 height) {
    return alt::ICore::Instance().CreateColShapeCylinder({ pos_x, pos_y, pos_z }, radius, height);
}
alt::IColShape* CreateColShapeSphere(f32 pos_x, f32 pos_y, f32 pos_z, f32 radius) {
    return alt::ICore::Instance().CreateColShapeSphere({ pos_x, pos_y, pos_z }, radius);
}
alt::IColShape* CreateColShapeCircle(f32 pos_x, f32 pos_y, f32 pos_z, f32 radius) {
    return alt::ICore::Instance().CreateColShapeCircle({ pos_x, pos_y, pos_z }, radius);
}
alt::IColShape* CreateColShapeCube(f32 pos_x, f32 pos_y, f32 pos_z, f32 pos2_x, f32 pos2_y, f32 pos2_z) {
    return alt::ICore::Instance().CreateColShapeCube({ pos_x, pos_y, pos_z }, { pos2_x, pos2_y, pos2_z });
}
alt::IColShape* CreateColShapeRectangle(f32 x1, f32 y1, f32 x2, f32 y2, f32 z) {
    return alt::ICore::Instance().CreateColShapeRectangle(x1, y1, x2, y2, z);
}
alt::IColShape* CreateColShapePolygon(f32 minZ, f32 maxZ, Vector2Vec points) {
    return alt::ICore::Instance().CreateColShapePolygon(minZ, maxZ, points.into_alt_vec());
}
bool IsDebug() {
    return alt::ICore::Instance().IsDebug();
}
u32 Hash(const StdStringClone str) {
    return alt::ICore::Instance().Hash(str);
}
bool FileExists(const StdStringClone path) {
    return alt::ICore::Instance().FileExists(path);
}
std::string FileRead(const StdStringClone path) {
    return alt::ICore::Instance().FileRead(path);
}
alt::IResource* GetResource(const StdStringClone name) {
    return alt::ICore::Instance().GetResource(name);
}
alt::IEntity* GetEntityBySyncID(u16 id) {
    return alt::ICore::Instance().GetEntityBySyncID(id);
}
PlayerVector GetPlayers() {
    auto alt_vec = alt::ICore::Instance().GetPlayers();
    PlayerVector vec {};
    vec.reserve(alt_vec.size());
    for (const auto& e : alt_vec) {
        PlayerPtrWrapper wrapper;
        wrapper.ptr = std::make_shared<alt::IPlayer*>(e);
        vec.push_back(wrapper.clone());
    }
    return vec;
}
bool HasMetaData(const StdStringClone key) {
    return alt::ICore::Instance().HasMetaData(key);
}
ConstMValueWrapper GetMetaData(const StdStringClone key) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = alt::ICore::Instance().GetMetaData(key);
    return wrapper;
}
void SetMetaData(const StdStringClone key, MValueMutWrapper val) {
    return alt::ICore::Instance().SetMetaData(key, val.ptr);
}
void DeleteMetaData(const StdStringClone key) {
    return alt::ICore::Instance().DeleteMetaData(key);
}
std::vector<std::string> GetMetaDataKeys() {
    return alt::ICore::Instance().GetMetaDataKeys();
}
bool HasSyncedMetaData(const StdStringClone key) {
    return alt::ICore::Instance().HasSyncedMetaData(key);
}
ConstMValueWrapper GetSyncedMetaData(const StdStringClone key) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = alt::ICore::Instance().GetSyncedMetaData(key);
    return wrapper;
}
std::vector<std::string> GetSyncedMetaDataKeys() {
    return alt::ICore::Instance().GetSyncedMetaDataKeys();
}
void DestroyBaseObject(alt::IBaseObject* handle) {
    return alt::ICore::Instance().DestroyBaseObject(handle);
}
const ResourceVector GetAllResources() {
    auto alt_vec = alt::ICore::Instance().GetAllResources();
    ResourceVector vec {};
    vec.reserve(alt_vec.size());
    for (const auto& e : alt_vec) {
        ResourcePtrWrapper wrapper;
        wrapper.ptr = std::make_shared<alt::IResource*>(e);
        vec.push_back(wrapper.clone());
    }
    return vec;
}
std::string StringToSHA256(const StdStringClone str) {
    return alt::ICore::Instance().StringToSHA256(str);
}
bool IsEventEnabled(u16 type) {
    return alt::ICore::Instance().IsEventEnabled(static_cast<alt::CEvent::Type>(type));
}
void ToggleEvent(u16 type, bool state) {
    return alt::ICore::Instance().ToggleEvent(static_cast<alt::CEvent::Type>(type), state);
}
bool AreControlsEnabled() {
    return alt::ICore::Instance().AreControlsEnabled();
}
Vector2Wrapper GetCursorPosition(bool normalized) {
    auto vector2 = alt::ICore::Instance().GetCursorPosition(normalized);
    return { vector2[0], vector2[1] };
}
void SetCursorPosition(f32 pos_x, f32 pos_y, bool normalized) {
    return alt::ICore::Instance().SetCursorPosition({ pos_x, pos_y }, normalized);
}
bool SetConfigFlag(const StdStringClone flag, bool state) {
    return alt::ICore::Instance().SetConfigFlag(flag, state);
}
bool GetConfigFlag(const StdStringClone flag) {
    return alt::ICore::Instance().GetConfigFlag(flag);
}
bool DoesConfigFlagExist(const StdStringClone flag) {
    return alt::ICore::Instance().DoesConfigFlagExist(flag);
}
void SetVoiceInputMuted(bool state) {
    return alt::ICore::Instance().SetVoiceInputMuted(state);
}
bool IsVoiceInputMuted() {
    return alt::ICore::Instance().IsVoiceInputMuted();
}
bool IsVoiceActivationEnabled() {
    return alt::ICore::Instance().IsVoiceActivationEnabled();
}
void ToggleVoiceControls(bool state) {
    return alt::ICore::Instance().ToggleVoiceControls(state);
}
bool AreVoiceControlsEnabled() {
    return alt::ICore::Instance().AreVoiceControlsEnabled();
}
u32 GetVoiceActivationKey() {
    return alt::ICore::Instance().GetVoiceActivationKey();
}
bool ToggleVoiceInput(bool state) {
    return alt::ICore::Instance().ToggleVoiceInput(state);
}
bool ToggleVoiceActivation(bool state) {
    return alt::ICore::Instance().ToggleVoiceActivation(state);
}
bool SetVoiceActivationLevel(f32 level) {
    return alt::ICore::Instance().SetVoiceActivationLevel(level);
}
f32 GetVoiceActivationLevel() {
    return alt::ICore::Instance().GetVoiceActivationLevel();
}
bool ToggleNoiseSuppression(bool state) {
    return alt::ICore::Instance().ToggleNoiseSuppression(state);
}
bool IsNoiseSuppressionEnabled() {
    return alt::ICore::Instance().IsNoiseSuppressionEnabled();
}
std::string GetLicenseHash() {
    return alt::ICore::Instance().GetLicenseHash();
}
std::string GetLocale() {
    return alt::ICore::Instance().GetLocale();
}
bool IsInStreamerMode() {
    return alt::ICore::Instance().IsInStreamerMode();
}
bool IsMenuOpen() {
    return alt::ICore::Instance().IsMenuOpen();
}
bool IsConsoleOpen() {
    return alt::ICore::Instance().IsConsoleOpen();
}
alt::IWorldObject* GetWorldObjectByScriptID(u32 scriptID) {
    return alt::ICore::Instance().GetWorldObjectByScriptID(scriptID);
}
void RequestIPL(const StdStringClone ipl) {
    return alt::ICore::Instance().RequestIPL(ipl);
}
void RemoveIPL(const StdStringClone ipl) {
    return alt::ICore::Instance().RemoveIPL(ipl);
}
bool BeginScaleformMovieMethodMinimap(const StdStringClone methodName) {
    return alt::ICore::Instance().BeginScaleformMovieMethodMinimap(methodName);
}
i32 GetMsPerGameMinute() {
    return alt::ICore::Instance().GetMsPerGameMinute();
}
void SetMsPerGameMinute(i32 val) {
    return alt::ICore::Instance().SetMsPerGameMinute(val);
}
void SetWeatherSyncActive(bool active) {
    return alt::ICore::Instance().SetWeatherSyncActive(active);
}
void SetCamFrozen(bool frozen) {
    return alt::ICore::Instance().SetCamFrozen(frozen);
}
bool IsCamFrozen() {
    return alt::ICore::Instance().IsCamFrozen();
}
u8 GetMapDataIDFromAlias(const StdStringClone alias) {
    return alt::ICore::Instance().GetMapDataIDFromAlias(alias);
}
void ResetMapData(u8 zoomDataId) {
    return alt::ICore::Instance().ResetMapData(zoomDataId);
}
void ResetMapData(const StdStringClone alias) {
    return alt::ICore::Instance().ResetMapData(alias);
}
void ResetAllMapData() {
    return alt::ICore::Instance().ResetAllMapData();
}
alt::IBlip* CreateBlip(BlipType type, f32 position_x, f32 position_y, f32 position_z, alt::IResource* res) {
    return alt::ICore::Instance().CreateBlip(static_cast<alt::IBlip::BlipType>(type), { position_x, position_y, position_z }, res);
}
alt::IBlip* CreateBlip(BlipType type, u32 entityID, alt::IResource* res) {
    return alt::ICore::Instance().CreateBlip(static_cast<alt::IBlip::BlipType>(type), entityID, res);
}
alt::IBlip* CreateBlip(f32 position_x, f32 position_y, f32 position_z, f32 radius, alt::IResource* res) {
    return alt::ICore::Instance().CreateBlip({ position_x, position_y, position_z }, radius, res);
}
alt::IBlip* CreateBlip(f32 position_x, f32 position_y, f32 position_z, f32 width, f32 height, alt::IResource* res) {
    return alt::ICore::Instance().CreateBlip({ position_x, position_y, position_z }, width, height, res);
}
alt::ICheckpoint* CreateCheckpoint(u8 type, f32 pos_x, f32 pos_y, f32 pos_z, f32 next_x, f32 next_y, f32 next_z, f32 radius, f32 height, u8 color_r, u8 color_g, u8 color_b, u8 color_a, u32 streamingDistance, alt::IResource* res) {
    return alt::ICore::Instance().CreateCheckpoint(type, { pos_x, pos_y, pos_z }, { next_x, next_y, next_z }, radius, height, { color_r, color_g, color_b, color_a }, streamingDistance, res);
}
alt::IMarker* CreateMarker(MarkerType type, f32 position_x, f32 position_y, f32 position_z, u8 color_r, u8 color_g, u8 color_b, u8 color_a, bool useStreaming, u32 streamingDistance, alt::IResource* res) {
    return alt::ICore::Instance().CreateMarker(static_cast<alt::IMarker::MarkerType>(type), { position_x, position_y, position_z }, { color_r, color_g, color_b, color_a }, useStreaming, streamingDistance, res);
}
bool IsGameFocused() {
    return alt::ICore::Instance().IsGameFocused();
}
void LoadModel(u32 hash) {
    return alt::ICore::Instance().LoadModel(hash);
}
void LoadModelAsync(u32 hash) {
    return alt::ICore::Instance().LoadModelAsync(hash);
}
bool LoadYtyp(const StdStringClone path) {
    return alt::ICore::Instance().LoadYtyp(path);
}
bool UnloadYtyp(const StdStringClone path) {
    return alt::ICore::Instance().UnloadYtyp(path);
}
std::string HeadshotToBase64(u8 id) {
    return alt::ICore::Instance().HeadshotToBase64(id);
}
void SetDlcClothes(i32 scriptID, u8 component, u16 drawable, u8 texture, u8 palette, u32 dlc) {
    return alt::ICore::Instance().SetDlcClothes(scriptID, component, drawable, texture, palette, dlc);
}
void SetDlcProps(i32 scriptID, u8 component, u8 drawable, u8 texture, u32 dlc) {
    return alt::ICore::Instance().SetDlcProps(scriptID, component, drawable, texture, dlc);
}
void ClearProps(i32 scriptID, u8 component) {
    return alt::ICore::Instance().ClearProps(scriptID, component);
}
void SetWatermarkPosition(u8 pos) {
    return alt::ICore::Instance().SetWatermarkPosition(pos);
}
u16 GetFps() {
    return alt::ICore::Instance().GetFps();
}
u16 GetPing() {
    return alt::ICore::Instance().GetPing();
}
u64 GetTotalPacketsSent() {
    return alt::ICore::Instance().GetTotalPacketsSent();
}
u32 GetTotalPacketsLost() {
    return alt::ICore::Instance().GetTotalPacketsLost();
}
std::string GetServerIp() {
    return alt::ICore::Instance().GetServerIp();
}
u16 GetServerPort() {
    return alt::ICore::Instance().GetServerPort();
}
std::string GetClientPath() {
    return alt::ICore::Instance().GetClientPath();
}
bool HasLocalMetaData(const StdStringClone key) {
    return alt::ICore::Instance().HasLocalMetaData(key);
}
MValueMutWrapper GetLocalMetaData(const StdStringClone key) {
    MValueMutWrapper wrapper;
    wrapper.ptr = alt::ICore::Instance().GetLocalMetaData(key);
    return wrapper;
}
bool CopyToClipboard(const StdStringClone value) {
    return alt::ICore::Instance().CopyToClipboard(value);
}
void ToggleRmlDebugger(bool state) {
    return alt::ICore::Instance().ToggleRmlDebugger(state);
}
bool LoadRmlFontFace(alt::IResource* resource, const StdStringClone path, const StdStringClone currentPath, const StdStringClone name, bool italic, bool bold) {
    return alt::ICore::Instance().LoadRmlFontFace(resource, path, currentPath, name, italic, bold);
}
void ToggleRmlControls(bool state) {
    return alt::ICore::Instance().ToggleRmlControls(state);
}
bool AreRmlControlsEnabled() {
    return alt::ICore::Instance().AreRmlControlsEnabled();
}
Vector3Wrapper WorldToScreen(f32 pos_x, f32 pos_y, f32 pos_z) {
    auto vector3 = alt::ICore::Instance().WorldToScreen({ pos_x, pos_y, pos_z });
    return { vector3[0], vector3[1], vector3[2] };
}
Vector3Wrapper ScreenToWorld(f32 pos_x, f32 pos_y) {
    auto vector3 = alt::ICore::Instance().ScreenToWorld({ pos_x, pos_y });
    return { vector3[0], vector3[1], vector3[2] };
}
Vector3Wrapper GetCamPos() {
    auto vector3 = alt::ICore::Instance().GetCamPos();
    return { vector3[0], vector3[1], vector3[2] };
}
void SetMinimapIsRectangle(bool state) {
    return alt::ICore::Instance().SetMinimapIsRectangle(state);
}
Config::Value::ValuePtr GetClientConfig() {
    return alt::ICore::Instance().GetClientConfig();
}
bool IsFocusOverriden() {
    return alt::ICore::Instance().IsFocusOverriden();
}
Vector3Wrapper GetFocusOverridePos() {
    auto vector3 = alt::ICore::Instance().GetFocusOverridePos();
    return { vector3[0], vector3[1], vector3[2] };
}
Vector3Wrapper GetFocusOverrideOffset() {
    auto vector3 = alt::ICore::Instance().GetFocusOverrideOffset();
    return { vector3[0], vector3[1], vector3[2] };
}
alt::IEntity* GetFocusOverrideEntity() {
    return alt::ICore::Instance().GetFocusOverrideEntity();
}
void OverrideFocusEntity(alt::IEntity* entity) {
    return alt::ICore::Instance().OverrideFocusEntity(entity);
}
void ClearFocusOverride() {
    return alt::ICore::Instance().ClearFocusOverride();
}
void LoadDefaultIpls() {
    return alt::ICore::Instance().LoadDefaultIpls();
}
bool IsPointOnScreen(f32 point_x, f32 point_y, f32 point_z) {
    return alt::ICore::Instance().IsPointOnScreen({ point_x, point_y, point_z });
}
Vector3Wrapper GetPedBonePos(i32 scriptId, u16 boneId) {
    auto vector3 = alt::ICore::Instance().GetPedBonePos(scriptId, boneId);
    return { vector3[0], vector3[1], vector3[2] };
}
void InternalAddCefBootstrap(const StdStringClone bootstrap) {
    return alt::ICore::Instance().InternalAddCefBootstrap(bootstrap);
}
bool IsFullScreen() {
    return alt::ICore::Instance().IsFullScreen();
}
alt::IBlip* GetBlipByGameID(u32 gameID) {
    return alt::ICore::Instance().GetBlipByGameID(gameID);
}
alt::ICheckpoint* GetCheckpointByGameID(u32 gameID) {
    return alt::ICore::Instance().GetCheckpointByGameID(gameID);
}
bool IsWebViewGpuAccelerationActive() {
    return alt::ICore::Instance().IsWebViewGpuAccelerationActive();
}

} // namespace