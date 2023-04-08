#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CFireEvent {

alt::IPlayer* GetSource(const alt::CFireEvent* ptr) {
    return ptr->GetSource();
}

} // namespace