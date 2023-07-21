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
void ActivatePhysics(alt::IObject* ptr) {
    return ptr->ActivatePhysics();
}
void PlaceOnGroundProperly(alt::IObject* ptr) {
    return ptr->PlaceOnGroundProperly();
}
void SetAlpha(alt::IObject* ptr, u8 alpha) {
    return ptr->SetAlpha(alpha);
}
void SetTextureVariation(alt::IObject* ptr, u8 textureVariation) {
    return ptr->SetTextureVariation(textureVariation);
}
void SetLodDistance(alt::IObject* ptr, u16 lodDistance) {
    return ptr->SetLodDistance(lodDistance);
}

} // namespace