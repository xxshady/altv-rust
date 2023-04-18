#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CNetOwnerChangeEvent {

alt::IEntity* GetTarget(const alt::CNetOwnerChangeEvent* ptr) {
    return ptr->GetTarget();
}
alt::IPlayer* GetNewOwner(const alt::CNetOwnerChangeEvent* ptr) {
    return ptr->GetNewOwner();
}
alt::IPlayer* GetOldOwner(const alt::CNetOwnerChangeEvent* ptr) {
    return ptr->GetOldOwner();
}

} // namespace