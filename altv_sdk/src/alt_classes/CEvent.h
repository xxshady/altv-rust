#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CEvent {

EventType GetType(const alt::CEvent* ptr) {
    return static_cast<uint16_t>(ptr->GetType());
}
bool WasCancelled(const alt::CEvent* ptr) {
    return ptr->WasCancelled();
}
void Cancel(const alt::CEvent* ptr) {
    return ptr->Cancel();
}

} // namespace