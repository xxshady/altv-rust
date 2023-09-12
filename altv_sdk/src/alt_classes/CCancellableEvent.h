#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CCancellableEvent {

bool IsCancellable(const alt::CCancellableEvent* ptr) {
    return ptr->IsCancellable();
}
bool WasCancelled(const alt::CCancellableEvent* ptr) {
    return ptr->WasCancelled();
}
void Cancel(const alt::CCancellableEvent* ptr) {
    return ptr->Cancel();
}

} // namespace