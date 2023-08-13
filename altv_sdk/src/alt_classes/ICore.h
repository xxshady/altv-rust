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
        ResourceVector vec{};
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
    const StdStringClone GetRootDirectory() {
        return std::string{ alt::ICore::Instance().GetRootDirectory() };
    }
    alt::IResource* StartResource(const StdStringClone name) {
        return alt::ICore::Instance().StartResource(name);
    }
    void StopResource(const StdStringClone name) {
        return alt::ICore::Instance().StopResource(name);
    }
    void RestartResource(const StdStringClone name) {
        return alt::ICore::Instance().RestartResource(name);
    }
    void SetSyncedMetaData(const StdStringClone key, MValueMutWrapper val) {
        return alt::ICore::Instance().SetSyncedMetaData(key, val.ptr);
    }
    void DeleteSyncedMetaData(const StdStringClone key) {
        return alt::ICore::Instance().DeleteSyncedMetaData(key);
    }
    alt::IVehicle* CreateVehicle(u32 model, f32 pos_x, f32 pos_y, f32 pos_z, f32 rot_x, f32 rot_y, f32 rot_z) {
        return alt::ICore::Instance().CreateVehicle(model, { pos_x, pos_y, pos_z }, { rot_x, rot_y, rot_z });
    }
    alt::ICheckpoint* CreateCheckpoint(u8 type, f32 pos_x, f32 pos_y, f32 pos_z, f32 radius, f32 height, u8 color_r, u8 color_g, u8 color_b, u8 color_a, u32 streamingDistance) {
        return alt::ICore::Instance().CreateCheckpoint(type, { pos_x, pos_y, pos_z }, radius, height, { color_r, color_g, color_b, color_a }, streamingDistance);
    }
    alt::IBlip* CreateBlip(bool global, BlipType type, f32 pos_x, f32 pos_y, f32 pos_z, PlayerVector targets) {
        return alt::ICore::Instance().CreateBlip(global, static_cast<alt::IBlip::BlipType>(type), { pos_x, pos_y, pos_z }, player_wrapper_vec_to_alt(targets));
    }
    alt::IBlip* CreateBlip(bool global, BlipType type, alt::IEntity* attachTo, PlayerVector targets) {
        return alt::ICore::Instance().CreateBlip(global, static_cast<alt::IBlip::BlipType>(type), attachTo, player_wrapper_vec_to_alt(targets));
    }
    alt::IMarker* CreateMarker(alt::IPlayer* target, MarkerType type, f32 position_x, f32 position_y, f32 position_z, u8 color_r, u8 color_g, u8 color_b, u8 color_a, alt::IResource* res) {
        return alt::ICore::Instance().CreateMarker(target, static_cast<alt::IMarker::MarkerType>(type), { position_x, position_y, position_z }, { color_r, color_g, color_b, color_a }, res);
    }
    alt::IVoiceChannel* CreateVoiceChannel(bool spatial, f32 maxDistance) {
        return alt::ICore::Instance().CreateVoiceChannel(spatial, maxDistance);
    }
    PlayerVector GetPlayersByName(const StdStringClone name) {
        auto alt_vec = alt::ICore::Instance().GetPlayersByName(name);
        PlayerVector vec{};
        vec.reserve(alt_vec.size());
        for (const auto& e : alt_vec) {
            PlayerPtrWrapper wrapper;
            wrapper.ptr = std::make_shared<alt::IPlayer*>(e);
            vec.push_back(wrapper.clone());
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
        void OverrideFocusEntity(alt::IEntity * entity) {
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
        void SetVoiceExternalPublic(const StdStringClone host, u16 port) {
            return alt::ICore::Instance().SetVoiceExternalPublic(host, port);
        }
        void SetVoiceExternal(const StdStringClone host, u16 port) {
            return alt::ICore::Instance().SetVoiceExternal(host, port);
        }

    } // namespace
