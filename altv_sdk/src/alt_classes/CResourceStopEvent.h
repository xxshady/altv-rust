#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CResourceStopEvent {

alt::IResource* GetResource(const alt::CResourceStopEvent* ptr) {
    return ptr->GetResource();
}

} // namespace