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
const std::vector<std::string> GetDependencies(const alt::IResource* ptr) {
    return ptr->GetDependencies();
}
const std::vector<std::string> GetDependants(const alt::IResource* ptr) {
    return ptr->GetDependants();
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
Config::Value::ValuePtr GetConfig(const alt::IResource* ptr) {
    return ptr->GetConfig();
}

} // namespace