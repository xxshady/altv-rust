#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IBlip {

void SetVisible(alt::IBlip* ptr, bool toggle) {
    return ptr->SetVisible(toggle);
}
bool IsVisible(const alt::IBlip* ptr) {
    return ptr->IsVisible();
}
bool IsGlobal(const alt::IBlip* ptr) {
    return ptr->IsGlobal();
}
bool IsAttached(const alt::IBlip* ptr) {
    return ptr->IsAttached();
}
alt::IEntity* AttachedTo(const alt::IBlip* ptr) {
    return ptr->AttachedTo();
}
void AttachTo(alt::IBlip* ptr, alt::IEntity* entity) {
    return ptr->AttachTo(entity);
}
BlipType GetBlipType(const alt::IBlip* ptr) {
    return static_cast<uint8_t>(ptr->GetBlipType());
}
void SetBlipType(alt::IBlip* ptr, BlipType blipType) {
    return ptr->SetBlipType(static_cast<alt::IBlip::BlipType>(blipType));
}
Vector2Wrapper GetScaleXY(const alt::IBlip* ptr) {
    auto vector2 = ptr->GetScaleXY();
    return { vector2[0], vector2[1] };
}
void SetScaleXY(alt::IBlip* ptr, f32 scale_x, f32 scale_y) {
    return ptr->SetScaleXY({ scale_x, scale_y });
}
u32 GetDisplay(const alt::IBlip* ptr) {
    return ptr->GetDisplay();
}
void SetDisplay(alt::IBlip* ptr, u32 display) {
    return ptr->SetDisplay(display);
}
u32 GetSprite(const alt::IBlip* ptr) {
    return ptr->GetSprite();
}
u32 GetColor(const alt::IBlip* ptr) {
    return ptr->GetColor();
}
RGBAWrapper GetSecondaryColor(const alt::IBlip* ptr) {
    auto rgba = ptr->GetSecondaryColor();
    return { rgba.r, rgba.g, rgba.b, rgba.a };
}
u32 GetAlpha(const alt::IBlip* ptr) {
    return ptr->GetAlpha();
}
cpp_int GetFlashTimer(const alt::IBlip* ptr) {
    return ptr->GetFlashTimer();
}
cpp_int GetFlashInterval(const alt::IBlip* ptr) {
    return ptr->GetFlashInterval();
}
bool GetAsFriendly(const alt::IBlip* ptr) {
    return ptr->GetAsFriendly();
}
bool GetRoute(const alt::IBlip* ptr) {
    return ptr->GetRoute();
}
bool GetBright(const alt::IBlip* ptr) {
    return ptr->GetBright();
}
cpp_int GetNumber(const alt::IBlip* ptr) {
    return ptr->GetNumber();
}
bool GetShowCone(const alt::IBlip* ptr) {
    return ptr->GetShowCone();
}
bool GetFlashes(const alt::IBlip* ptr) {
    return ptr->GetFlashes();
}
bool GetFlashesAlternate(const alt::IBlip* ptr) {
    return ptr->GetFlashesAlternate();
}
bool GetAsShortRange(const alt::IBlip* ptr) {
    return ptr->GetAsShortRange();
}
u32 GetPriority(const alt::IBlip* ptr) {
    return ptr->GetPriority();
}
f32 GetRotation(const alt::IBlip* ptr) {
    return ptr->GetRotation();
}
std::string GetGxtName(const alt::IBlip* ptr) {
    return ptr->GetGxtName();
}
std::string GetName(const alt::IBlip* ptr) {
    return ptr->GetName();
}
RGBAWrapper GetRouteColor(const alt::IBlip* ptr) {
    auto rgba = ptr->GetRouteColor();
    return { rgba.r, rgba.g, rgba.b, rgba.a };
}
bool GetPulse(const alt::IBlip* ptr) {
    return ptr->GetPulse();
}
bool GetAsMissionCreator(const alt::IBlip* ptr) {
    return ptr->GetAsMissionCreator();
}
bool GetTickVisible(const alt::IBlip* ptr) {
    return ptr->GetTickVisible();
}
bool GetHeadingIndicatorVisible(const alt::IBlip* ptr) {
    return ptr->GetHeadingIndicatorVisible();
}
bool GetOutlineIndicatorVisible(const alt::IBlip* ptr) {
    return ptr->GetOutlineIndicatorVisible();
}
bool GetFriendIndicatorVisible(const alt::IBlip* ptr) {
    return ptr->GetFriendIndicatorVisible();
}
bool GetCrewIndicatorVisible(const alt::IBlip* ptr) {
    return ptr->GetCrewIndicatorVisible();
}
u32 GetCategory(const alt::IBlip* ptr) {
    return ptr->GetCategory();
}
bool GetAsHighDetail(const alt::IBlip* ptr) {
    return ptr->GetAsHighDetail();
}
bool GetShrinked(const alt::IBlip* ptr) {
    return ptr->GetShrinked();
}
void SetGlobal(alt::IBlip* ptr, bool state) {
    return ptr->SetGlobal(state);
}
void AddTargetPlayer(alt::IBlip* ptr, alt::IPlayer* player) {
    return ptr->AddTargetPlayer(player);
}
void RemoveTargetPlayer(alt::IBlip* ptr, alt::IPlayer* player) {
    return ptr->RemoveTargetPlayer(player);
}
PlayerVector GetTargets(const alt::IBlip* ptr) {
    auto alt_vec = ptr->GetTargets();
    PlayerVector vec {};
    vec.reserve(alt_vec.size());
    for (const auto& e : alt_vec) {
        PlayerPtrWrapper wrapper;
        wrapper.ptr = std::make_shared<alt::IPlayer*>(e);
        vec.push_back(wrapper.clone());
    }
    return vec;
}
void SetSprite(alt::IBlip* ptr, u32 sprite) {
    return ptr->SetSprite(sprite);
}
void SetColor(alt::IBlip* ptr, u32 color) {
    return ptr->SetColor(color);
}
void SetRoute(alt::IBlip* ptr, bool state) {
    return ptr->SetRoute(state);
}
void SetRouteColor(alt::IBlip* ptr, u8 color_r, u8 color_g, u8 color_b, u8 color_a) {
    return ptr->SetRouteColor({ color_r, color_g, color_b, color_a });
}
void SetSecondaryColor(alt::IBlip* ptr, u8 color_r, u8 color_g, u8 color_b, u8 color_a) {
    return ptr->SetSecondaryColor({ color_r, color_g, color_b, color_a });
}
void SetAlpha(alt::IBlip* ptr, u32 alpha) {
    return ptr->SetAlpha(alpha);
}
void SetFlashTimer(alt::IBlip* ptr, cpp_int timer) {
    return ptr->SetFlashTimer(timer);
}
void SetFlashInterval(alt::IBlip* ptr, cpp_int interval) {
    return ptr->SetFlashInterval(interval);
}
void SetAsFriendly(alt::IBlip* ptr, bool friendly) {
    return ptr->SetAsFriendly(friendly);
}
void SetBright(alt::IBlip* ptr, bool bright) {
    return ptr->SetBright(bright);
}
void SetNumber(alt::IBlip* ptr, cpp_int number) {
    return ptr->SetNumber(number);
}
void SetShowCone(alt::IBlip* ptr, bool state) {
    return ptr->SetShowCone(state);
}
void SetFlashes(alt::IBlip* ptr, bool state) {
    return ptr->SetFlashes(state);
}
void SetFlashesAlternate(alt::IBlip* ptr, bool state) {
    return ptr->SetFlashesAlternate(state);
}
void SetAsShortRange(alt::IBlip* ptr, bool state) {
    return ptr->SetAsShortRange(state);
}
void SetPriority(alt::IBlip* ptr, u32 state) {
    return ptr->SetPriority(state);
}
void SetRotation(alt::IBlip* ptr, f32 rot) {
    return ptr->SetRotation(rot);
}
void SetGxtName(alt::IBlip* ptr, const StdStringClone name) {
    return ptr->SetGxtName(name);
}
void SetName(alt::IBlip* ptr, const StdStringClone name) {
    return ptr->SetName(name);
}
void SetPulse(alt::IBlip* ptr, bool val) {
    return ptr->SetPulse(val);
}
void SetAsMissionCreator(alt::IBlip* ptr, bool val) {
    return ptr->SetAsMissionCreator(val);
}
void SetTickVisible(alt::IBlip* ptr, bool val) {
    return ptr->SetTickVisible(val);
}
void SetHeadingIndicatorVisible(alt::IBlip* ptr, bool val) {
    return ptr->SetHeadingIndicatorVisible(val);
}
void SetOutlineIndicatorVisible(alt::IBlip* ptr, bool val) {
    return ptr->SetOutlineIndicatorVisible(val);
}
void SetFriendIndicatorVisible(alt::IBlip* ptr, bool val) {
    return ptr->SetFriendIndicatorVisible(val);
}
void SetCrewIndicatorVisible(alt::IBlip* ptr, bool val) {
    return ptr->SetCrewIndicatorVisible(val);
}
void SetCategory(alt::IBlip* ptr, u32 val) {
    return ptr->SetCategory(val);
}
void SetAsHighDetail(alt::IBlip* ptr, bool val) {
    return ptr->SetAsHighDetail(val);
}
void SetShrinked(alt::IBlip* ptr, bool val) {
    return ptr->SetShrinked(val);
}
void Fade(alt::IBlip* ptr, u32 opacity, u32 duration) {
    return ptr->Fade(opacity, duration);
}
bool IsHiddenOnLegend(const alt::IBlip* ptr) {
    return ptr->IsHiddenOnLegend();
}
void SetHiddenOnLegend(alt::IBlip* ptr, bool state) {
    return ptr->SetHiddenOnLegend(state);
}
bool IsMinimalOnEdge(const alt::IBlip* ptr) {
    return ptr->IsMinimalOnEdge();
}
void SetMinimalOnEdge(alt::IBlip* ptr, bool state) {
    return ptr->SetMinimalOnEdge(state);
}
bool IsUseHeightIndicatorOnEdge(const alt::IBlip* ptr) {
    return ptr->IsUseHeightIndicatorOnEdge();
}
void SetUseHeightIndicatorOnEdge(alt::IBlip* ptr, bool state) {
    return ptr->SetUseHeightIndicatorOnEdge(state);
}
bool IsShortHeightThreshold(const alt::IBlip* ptr) {
    return ptr->IsShortHeightThreshold();
}
void SetShortHeightThreshold(alt::IBlip* ptr, bool state) {
    return ptr->SetShortHeightThreshold(state);
}

} // namespace