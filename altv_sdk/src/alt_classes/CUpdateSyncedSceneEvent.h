#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CUpdateSyncedSceneEvent {

alt::IPlayer* GetSource(const alt::CUpdateSyncedSceneEvent* ptr) {
    return ptr->GetSource();
}
f32 GetStartRate(const alt::CUpdateSyncedSceneEvent* ptr) {
    return ptr->GetStartRate();
}
i32 GetSceneID(const alt::CUpdateSyncedSceneEvent* ptr) {
    return ptr->GetSceneID();
}

} // namespace