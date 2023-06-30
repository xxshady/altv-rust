#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace INetworkObject {

u8 GetAlpha(const alt::INetworkObject* ptr) {
    return ptr->GetAlpha();
}
u8 GetTextureVariation(const alt::INetworkObject* ptr) {
    return ptr->GetTextureVariation();
}
u16 GetLodDistance(const alt::INetworkObject* ptr) {
    return ptr->GetLodDistance();
}

} // namespace