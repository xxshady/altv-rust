#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IEntity {

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
bool IsFrozen(const alt::IEntity* ptr) {
    return ptr->IsFrozen();
}
void SetFrozen(alt::IEntity* ptr, bool state) {
    return ptr->SetFrozen(state);
}
u32 GetScriptID(const alt::IEntity* ptr) {
    return ptr->GetScriptID();
}

} // namespace