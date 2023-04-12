#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CConsoleCommandEvent {

std::string GetName(const alt::CConsoleCommandEvent* ptr) {
    return ptr->GetName();
}
const std::vector<std::string> GetArgs(const alt::CConsoleCommandEvent* ptr) {
    return ptr->GetArgs();
}

} // namespace