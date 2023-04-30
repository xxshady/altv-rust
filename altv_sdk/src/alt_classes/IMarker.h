#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IMarker {

u32 GetID(const alt::IMarker* ptr) {
    return ptr->GetID();
}
bool IsGlobal(const alt::IMarker* ptr) {
    return ptr->IsGlobal();
}
alt::IPlayer* GetTarget(const alt::IMarker* ptr) {
    return ptr->GetTarget();
}
RGBAWrapper GetColor(const alt::IMarker* ptr) {
    auto rgba = ptr->GetColor();
    return { rgba.r, rgba.g, rgba.b, rgba.a };
}
void SetColor(alt::IMarker* ptr, u8 color_r, u8 color_g, u8 color_b, u8 color_a) {
    return ptr->SetColor({ color_r, color_g, color_b, color_a });
}
bool IsVisible(const alt::IMarker* ptr) {
    return ptr->IsVisible();
}
void SetVisible(alt::IMarker* ptr, bool visible) {
    return ptr->SetVisible(visible);
}
MarkerType GetMarkerType(const alt::IMarker* ptr) {
    return static_cast<uint32_t>(ptr->GetMarkerType());
}
void SetMarkerType(alt::IMarker* ptr, MarkerType type) {
    return ptr->SetMarkerType(static_cast<alt::IMarker::MarkerType>(type));
}
Vector3Wrapper GetScale(const alt::IMarker* ptr) {
    auto vector3 = ptr->GetScale();
    return { vector3[0], vector3[1], vector3[2] };
}
void SetScale(alt::IMarker* ptr, f32 scale_x, f32 scale_y, f32 scale_z) {
    return ptr->SetScale({ scale_x, scale_y, scale_z });
}
Vector3Wrapper GetRotation(const alt::IMarker* ptr) {
    auto vector3 = ptr->GetRotation();
    return { vector3[0], vector3[1], vector3[2] };
}
void SetRotation(alt::IMarker* ptr, f32 _rot_x, f32 _rot_y, f32 _rot_z) {
    return ptr->SetRotation({ _rot_x, _rot_y, _rot_z });
}
Vector3Wrapper GetDirection(const alt::IMarker* ptr) {
    auto vector3 = ptr->GetDirection();
    return { vector3[0], vector3[1], vector3[2] };
}
void SetDirection(alt::IMarker* ptr, f32 dir_x, f32 dir_y, f32 dir_z) {
    return ptr->SetDirection({ dir_x, dir_y, dir_z });
}
bool IsFaceCamera(const alt::IMarker* ptr) {
    return ptr->IsFaceCamera();
}
void SetFaceCamera(alt::IMarker* ptr, bool faceCamera) {
    return ptr->SetFaceCamera(faceCamera);
}
u32 GetStreamingDistance(const alt::IMarker* ptr) {
    return ptr->GetStreamingDistance();
}

} // namespace