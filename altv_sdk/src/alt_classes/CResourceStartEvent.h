#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CResourceStartEvent {

alt::IResource* GetResource(const alt::CResourceStartEvent* ptr) {
    return ptr->GetResource();
}

} // namespace