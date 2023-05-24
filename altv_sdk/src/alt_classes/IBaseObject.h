#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IBaseObject {

BaseObjectType GetType(const alt::IBaseObject* ptr) {
    return static_cast<uint8_t>(ptr->GetType());
}
u32 GetID(const alt::IBaseObject* ptr) {
    return ptr->GetID();
}
bool HasMetaData(const alt::IBaseObject* ptr, const StdStringClone key) {
    return ptr->HasMetaData(key);
}
ConstMValueWrapper GetMetaData(const alt::IBaseObject* ptr, const StdStringClone key) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetMetaData(key);
    return wrapper;
}
void SetMetaData(alt::IBaseObject* ptr, const StdStringClone key, MValueMutWrapper val) {
    return ptr->SetMetaData(key, val.ptr);
}
void DeleteMetaData(alt::IBaseObject* ptr, const StdStringClone key) {
    return ptr->DeleteMetaData(key);
}
std::vector<std::string> GetMetaDataKeys(const alt::IBaseObject* ptr) {
    return ptr->GetMetaDataKeys();
}
bool HasSyncedMetaData(const alt::IBaseObject* ptr, const StdStringClone key) {
    return ptr->HasSyncedMetaData(key);
}
ConstMValueWrapper GetSyncedMetaData(const alt::IBaseObject* ptr, const StdStringClone key) {
    ConstMValueWrapper wrapper;
    wrapper.ptr = ptr->GetSyncedMetaData(key);
    return wrapper;
}
std::vector<std::string> GetSyncedMetaDataKeys(const alt::IBaseObject* ptr) {
    return ptr->GetSyncedMetaDataKeys();
}
void SetSyncedMetaData(alt::IBaseObject* ptr, const StdStringClone key, MValueMutWrapper val) {
    return ptr->SetSyncedMetaData(key, val.ptr);
}
void DeleteSyncedMetaData(alt::IBaseObject* ptr, const StdStringClone key) {
    return ptr->DeleteSyncedMetaData(key);
}
bool IsRemoved(const alt::IBaseObject* ptr) {
    return ptr->IsRemoved();
}

} // namespace