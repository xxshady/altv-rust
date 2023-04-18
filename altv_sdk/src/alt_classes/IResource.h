#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IResource {

bool IsStarted(const alt::IResource* ptr) {
    return ptr->IsStarted();
}
const StdStringClone GetType(const alt::IResource* ptr) {
    return std::string { ptr->GetType() };
}
const StdStringClone GetName(const alt::IResource* ptr) {
    return std::string { ptr->GetName() };
}
const StdStringClone GetPath(const alt::IResource* ptr) {
    return std::string { ptr->GetPath() };
}
const StdStringClone GetMain(const alt::IResource* ptr) {
    return std::string { ptr->GetMain() };
}
std::string GetClientType(const alt::IResource* ptr) {
    return ptr->GetClientType();
}
std::string GetClientMain(const alt::IResource* ptr) {
    return ptr->GetClientMain();
}
const std::vector<std::string> GetClientFiles(const alt::IResource* ptr) {
    return ptr->GetClientFiles();
}

} // namespace