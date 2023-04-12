#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerConnectEvent {

alt::IPlayer* GetTarget(const alt::CPlayerConnectEvent* ptr) {
    return ptr->GetTarget();
}
const StdStringClone GetReason(alt::CPlayerConnectEvent* ptr) {
    return std::string { ptr->GetReason() };
}
void Cancel(alt::CPlayerConnectEvent* ptr, const StdStringClone _reason) {
    return ptr->Cancel(_reason);
}

} // namespace