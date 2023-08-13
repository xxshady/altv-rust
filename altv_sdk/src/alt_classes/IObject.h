#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IObject {

u8 GetAlpha(const alt::IObject* ptr) {
    return ptr->GetAlpha();
}
u8 GetTextureVariation(const alt::IObject* ptr) {
    return ptr->GetTextureVariation();
}
u16 GetLodDistance(const alt::IObject* ptr) {
    return ptr->GetLodDistance();
}

} // namespace