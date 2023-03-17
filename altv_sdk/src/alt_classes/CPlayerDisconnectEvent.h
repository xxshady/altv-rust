#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerDisconnectEvent {

alt::IPlayer* GetTarget(const alt::CPlayerDisconnectEvent* ptr) {
    return ptr->GetTarget();
}
StdStringClone GetReason(const alt::CPlayerDisconnectEvent* ptr) {
    return std::string { ptr->GetReason() };
}

} // namespace