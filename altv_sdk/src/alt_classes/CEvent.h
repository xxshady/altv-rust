#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CEvent {

bool IsCancellable(const alt::CEvent* ptr) {
    return ptr->IsCancellable();
}
EventType GetType(const alt::CEvent* ptr) {
    return static_cast<uint16_t>(ptr->GetType());
}

} // namespace