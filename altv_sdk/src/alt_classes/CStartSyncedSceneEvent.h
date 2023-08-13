#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CStartSyncedSceneEvent {

alt::IPlayer* GetSource(const alt::CStartSyncedSceneEvent* ptr) {
    return ptr->GetSource();
}
i32 GetSceneID(const alt::CStartSyncedSceneEvent* ptr) {
    return ptr->GetSceneID();
}
Vector3Wrapper GetStartPosition(const alt::CStartSyncedSceneEvent* ptr) {
    auto vector3 = ptr->GetStartPosition();
    return { vector3[0], vector3[1], vector3[2] };
}
Vector3Wrapper GetStartRotation(const alt::CStartSyncedSceneEvent* ptr) {
    auto vector3 = ptr->GetStartRotation();
    return { vector3[0], vector3[1], vector3[2] };
}
u32 GetAnimDictHash(const alt::CStartSyncedSceneEvent* ptr) {
    return ptr->GetAnimDictHash();
}
EntityAnimHashPairsWrapper GetEntityAndAnimHashPairs(const alt::CStartSyncedSceneEvent* ptr) {
    EntityAnimHashPairsWrapper wrapper;
    wrapper.value = ptr->GetEntityAndAnimHashPairs();
    return wrapper;
}

} // namespace