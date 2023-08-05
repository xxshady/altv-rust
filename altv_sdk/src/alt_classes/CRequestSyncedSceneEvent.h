#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CRequestSyncedSceneEvent {

alt::IPlayer* GetSource(const alt::CRequestSyncedSceneEvent* ptr) {
    return ptr->GetSource();
}
i32 GetSceneID(const alt::CRequestSyncedSceneEvent* ptr) {
    return ptr->GetSceneID();
}

} // namespace