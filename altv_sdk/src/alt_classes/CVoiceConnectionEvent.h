#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CVoiceConnectionEvent {

VoiceConnectionState GetState(const alt::CVoiceConnectionEvent* ptr) {
    return static_cast<uint8_t>(ptr->GetState());
}

} // namespace