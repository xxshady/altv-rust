#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CGivePedScriptedTaskEvent {

alt::IPlayer* GetSource(const alt::CGivePedScriptedTaskEvent* ptr) {
    return ptr->GetSource();
}
alt::IPed* GetTarget(const alt::CGivePedScriptedTaskEvent* ptr) {
    return ptr->GetTarget();
}
u32 GetTaskType(const alt::CGivePedScriptedTaskEvent* ptr) {
    return ptr->GetTaskType();
}

} // namespace