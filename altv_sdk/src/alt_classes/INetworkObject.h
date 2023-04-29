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
void ActivatePhysics(alt::INetworkObject* ptr) {
    return ptr->ActivatePhysics();
}
void PlaceOnGroundProperly(alt::INetworkObject* ptr) {
    return ptr->PlaceOnGroundProperly();
}
void SetAlpha(alt::INetworkObject* ptr, u8 alpha) {
    return ptr->SetAlpha(alpha);
}
void SetTextureVariation(alt::INetworkObject* ptr, u8 textureVariation) {
    return ptr->SetTextureVariation(textureVariation);
}
void SetLodDistance(alt::INetworkObject* ptr, u16 lodDistance) {
    return ptr->SetLodDistance(lodDistance);
}

} // namespace