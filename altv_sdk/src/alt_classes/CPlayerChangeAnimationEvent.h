#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerChangeAnimationEvent {

alt::IPlayer* GetTarget(const alt::CPlayerChangeAnimationEvent* ptr) {
    return ptr->GetTarget();
}
u32 GetOldAnimationDict(const alt::CPlayerChangeAnimationEvent* ptr) {
    return ptr->GetOldAnimationDict();
}
u32 GetOldAnimationName(const alt::CPlayerChangeAnimationEvent* ptr) {
    return ptr->GetOldAnimationName();
}
u32 GetNewAnimationDict(const alt::CPlayerChangeAnimationEvent* ptr) {
    return ptr->GetNewAnimationDict();
}
u32 GetNewAnimationName(const alt::CPlayerChangeAnimationEvent* ptr) {
    return ptr->GetNewAnimationName();
}

} // namespace