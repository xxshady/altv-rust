#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace ICheckpoint {

u8 GetCheckpointType(const alt::ICheckpoint* ptr) {
    return ptr->GetCheckpointType();
}
f32 GetHeight(const alt::ICheckpoint* ptr) {
    return ptr->GetHeight();
}
f32 GetRadius(const alt::ICheckpoint* ptr) {
    return ptr->GetRadius();
}
RGBAWrapper GetColor(const alt::ICheckpoint* ptr) {
    auto rgba = ptr->GetColor();
    return { rgba.r, rgba.g, rgba.b, rgba.a };
}
Vector3Wrapper GetNextPosition(const alt::ICheckpoint* ptr) {
    auto vector3 = ptr->GetNextPosition();
    return { vector3[0], vector3[1], vector3[2] };
}
void SetCheckpointType(alt::ICheckpoint* ptr, u8 type) {
    return ptr->SetCheckpointType(type);
}
void SetHeight(alt::ICheckpoint* ptr, f32 height) {
    return ptr->SetHeight(height);
}
void SetRadius(alt::ICheckpoint* ptr, f32 radius) {
    return ptr->SetRadius(radius);
}
void SetColor(alt::ICheckpoint* ptr, u8 color_r, u8 color_g, u8 color_b, u8 color_a) {
    return ptr->SetColor({ color_r, color_g, color_b, color_a });
}
void SetNextPosition(alt::ICheckpoint* ptr, f32 pos_x, f32 pos_y, f32 pos_z) {
    return ptr->SetNextPosition({ pos_x, pos_y, pos_z });
}
u32 GetStreamingDistance(const alt::ICheckpoint* ptr) {
    return ptr->GetStreamingDistance();
}
void SetVisible(alt::ICheckpoint* ptr, bool toggle) {
    return ptr->SetVisible(toggle);
}
bool IsVisible(const alt::ICheckpoint* ptr) {
    return ptr->IsVisible();
}
bool HasStreamSyncedMetaData(const alt::ICheckpoint* ptr, const StdStringClone key) {
    return ptr->HasStreamSyncedMetaData(key);
}
MValueWrapper GetStreamSyncedMetaData(const alt::ICheckpoint* ptr, const StdStringClone key) {
    MValueWrapper wrapper;
    wrapper.ptr = ptr->GetStreamSyncedMetaData(key);
    return wrapper;
}
std::vector<std::string> GetStreamSyncedMetaDataKeys(const alt::ICheckpoint* ptr) {
    return ptr->GetStreamSyncedMetaDataKeys();
}
void SetStreamSyncedMetaData(alt::ICheckpoint* ptr, const StdStringClone key, MValueMutWrapper val) {
    return ptr->SetStreamSyncedMetaData(key, val.ptr);
}
void DeleteStreamSyncedMetaData(alt::ICheckpoint* ptr, const StdStringClone key) {
    return ptr->DeleteStreamSyncedMetaData(key);
}

} // namespace