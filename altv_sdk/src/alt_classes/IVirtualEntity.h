#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IVirtualEntity {

alt::IVirtualEntityGroup* GetGroup(const alt::IVirtualEntity* ptr) {
    return ptr->GetGroup();
}
bool HasStreamSyncedMetaData(const alt::IVirtualEntity* ptr, const StdStringClone key) {
    return ptr->HasStreamSyncedMetaData(key);
}
ConstMValueWrapper GetStreamSyncedMetaData(const alt::IVirtualEntity* ptr, const StdStringClone key) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetStreamSyncedMetaData(key);
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
bool IsStreamedIn(const alt::IVirtualEntity* ptr) {
    return ptr->IsStreamedIn();
}

} // namespace