#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IVoiceChannel {

u32 GetID(const alt::IVoiceChannel* ptr) {
    return ptr->GetID();
}
bool IsSpatial(const alt::IVoiceChannel* ptr) {
    return ptr->IsSpatial();
}
f32 GetMaxDistance(const alt::IVoiceChannel* ptr) {
    return ptr->GetMaxDistance();
}
bool HasPlayer(const alt::IVoiceChannel* ptr, alt::IPlayer* player) {
    return ptr->HasPlayer(player);
}
void AddPlayer(alt::IVoiceChannel* ptr, alt::IPlayer* player) {
    return ptr->AddPlayer(player);
}
void RemovePlayer(alt::IVoiceChannel* ptr, alt::IPlayer* player) {
    return ptr->RemovePlayer(player);
}
bool IsPlayerMuted(const alt::IVoiceChannel* ptr, alt::IPlayer* player) {
    return ptr->IsPlayerMuted(player);
}
void MutePlayer(alt::IVoiceChannel* ptr, alt::IPlayer* player) {
    return ptr->MutePlayer(player);
}
void UnmutePlayer(alt::IVoiceChannel* ptr, alt::IPlayer* player) {
    return ptr->UnmutePlayer(player);
}
const PlayerVector GetPlayers(const alt::IVoiceChannel* ptr) {
    auto alt_vec = ptr->GetPlayers();
    PlayerVector vec {};
    vec.reserve(alt_vec.size());
    for (const auto& e : alt_vec) {
        PlayerPtrWrapper wrapper;
        wrapper.ptr = std::make_shared<alt::IPlayer*>(e);
        vec.push_back(wrapper.clone());
    }
    return vec;
}
u32 GetFilter(const alt::IVoiceChannel* ptr) {
    return ptr->GetFilter();
}
void SetFilter(alt::IVoiceChannel* ptr, u32 filter) {
    return ptr->SetFilter(filter);
}
i32 GetPriority(const alt::IVoiceChannel* ptr) {
    return ptr->GetPriority();
}
void SetPriority(alt::IVoiceChannel* ptr, i32 priority) {
    return ptr->SetPriority(priority);
}

} // namespace