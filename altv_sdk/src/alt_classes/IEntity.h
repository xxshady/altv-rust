#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IEntity {

u32 GetID(const alt::IEntity* ptr) {
    return ptr->GetID();
}
u16 GetSyncID(const alt::IEntity* ptr) {
    return ptr->GetSyncID();
}
alt::IPlayer* GetNetworkOwner(const alt::IEntity* ptr) {
    return ptr->GetNetworkOwner();
}
u32 GetModel(const alt::IEntity* ptr) {
    return ptr->GetModel();
}
Vector3Wrapper GetRotation(const alt::IEntity* ptr) {
    auto vector3 = ptr->GetRotation();
    return { vector3[0], vector3[1], vector3[2] };
}
void SetRotation(alt::IEntity* ptr, f32 rot_x, f32 rot_y, f32 rot_z) {
    return ptr->SetRotation({ rot_x, rot_y, rot_z });
}
bool HasStreamSyncedMetaData(const alt::IEntity* ptr, const StdStringClone key) {
    return ptr->HasStreamSyncedMetaData(key);
}
ConstMValueWrapper GetStreamSyncedMetaData(const alt::IEntity* ptr, const StdStringClone key) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetStreamSyncedMetaData(key);
    return wrapper;
}
std::vector<std::string> GetStreamSyncedMetaDataKeys(const alt::IEntity* ptr) {
    return ptr->GetStreamSyncedMetaDataKeys();
}
bool GetVisible(const alt::IEntity* ptr) {
    return ptr->GetVisible();
}
void SetNetworkOwner(alt::IEntity* ptr, alt::IPlayer* player, bool disableMigration) {
    return ptr->SetNetworkOwner(player, disableMigration);
}
void SetStreamSyncedMetaData(alt::IEntity* ptr, const StdStringClone key, MValueMutWrapper val) {
    return ptr->SetStreamSyncedMetaData(key, val.ptr);
}
void DeleteStreamSyncedMetaData(alt::IEntity* ptr, const StdStringClone key) {
    return ptr->DeleteStreamSyncedMetaData(key);
}
void SetVisible(alt::IEntity* ptr, bool toggle) {
    return ptr->SetVisible(toggle);
}
void AttachToEntity(alt::IEntity* ptr, alt::IEntity* entity, i16 otherBoneIndex, i16 myBoneIndex, f32 position_x, f32 position_y, f32 position_z, f32 rotation_x, f32 rotation_y, f32 rotation_z, bool collision, bool noFixedRotation) {
    return ptr->AttachToEntity(entity, otherBoneIndex, myBoneIndex, { position_x, position_y, position_z }, { rotation_x, rotation_y, rotation_z }, collision, noFixedRotation);
}
void AttachToEntity(alt::IEntity* ptr, alt::IEntity* entity, const StdStringClone otherBoneName, const StdStringClone myBoneName, f32 position_x, f32 position_y, f32 position_z, f32 rotation_x, f32 rotation_y, f32 rotation_z, bool collision, bool noFixedRotation) {
    return ptr->AttachToEntity(entity, otherBoneName, myBoneName, { position_x, position_y, position_z }, { rotation_x, rotation_y, rotation_z }, collision, noFixedRotation);
}
void Detach(alt::IEntity* ptr) {
    return ptr->Detach();
}
void SetStreamed(alt::IEntity* ptr, bool toggle) {
    return ptr->SetStreamed(toggle);
}
bool GetStreamed(const alt::IEntity* ptr) {
    return ptr->GetStreamed();
}
bool IsFrozen(const alt::IEntity* ptr) {
    return ptr->IsFrozen();
}
void SetFrozen(alt::IEntity* ptr, bool state) {
    return ptr->SetFrozen(state);
}
bool HasCollision(const alt::IEntity* ptr) {
    return ptr->HasCollision();
}
void SetCollision(alt::IEntity* ptr, bool state) {
    return ptr->SetCollision(state);
}

} // namespace