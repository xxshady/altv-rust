#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CStopSyncedSceneEvent {

alt::IPlayer* GetSource(const alt::CStopSyncedSceneEvent* ptr) {
    return ptr->GetSource();
}
i32 GetSceneID(const alt::CStopSyncedSceneEvent* ptr) {
    return ptr->GetSceneID();
}

} // namespace