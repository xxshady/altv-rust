#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IVirtualEntity {

u32 GetID(const alt::IVirtualEntity* ptr) {
    return ptr->GetID();
}
alt::IVirtualEntityGroup* GetGroup(const alt::IVirtualEntity* ptr) {
    return ptr->GetGroup();
}
bool HasStreamSyncedMetaData(const alt::IVirtualEntity* ptr, const StdStringClone key) {
    return ptr->HasStreamSyncedMetaData(key);
}
MValueWrapper GetStreamSyncedMetaData(const alt::IVirtualEntity* ptr, const StdStringClone key) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(ptr->GetStreamSyncedMetaData(key));
    return wrapper;
}
std::vector<std::string> GetStreamSyncedMetaDataKeys(const alt::IVirtualEntity* ptr) {
    return ptr->GetStreamSyncedMetaDataKeys();
}
u32 GetStreamingDistance(const alt::IVirtualEntity* ptr) {
    return ptr->GetStreamingDistance();
}
void SetVisible(alt::IVirtualEntity* ptr, bool toggle) {
    return ptr->SetVisible(toggle);
}
bool IsVisible(const alt::IVirtualEntity* ptr) {
    return ptr->IsVisible();
}
void SetStreamSyncedMetaData(alt::IVirtualEntity* ptr, const StdStringClone key, MValueMutWrapper val) {
    return ptr->SetStreamSyncedMetaData(key, *(val.ptr));
}
void DeleteStreamSyncedMetaData(alt::IVirtualEntity* ptr, const StdStringClone key) {
    return ptr->DeleteStreamSyncedMetaData(key);
}

} // namespace