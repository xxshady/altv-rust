#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CFireEvent {

alt::IPlayer* GetSource(const alt::CFireEvent* ptr) {
    return ptr->GetSource();
}
std::vector<FireInfoWrapper> GetFires(const alt::CFireEvent* ptr) {
    auto alt_array = ptr->GetFires();
    std::vector<FireInfoWrapper> vec {};
    vec.reserve(alt_array.GetSize());
    for (const auto& e : alt_array) {
        vec.push_back({ { e.position[0], e.position[1], e.position[2] }, e.weaponHash });
    }
    return vec;
}

} // namespace