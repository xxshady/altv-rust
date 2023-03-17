#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerConnectEvent {

alt::IPlayer* GetTarget(const alt::CPlayerConnectEvent* ptr) {
    return ptr->GetTarget();
}
StdStringClone GetReason(alt::CPlayerConnectEvent* ptr) {
    return std::string { ptr->GetReason() };
}

} // namespace