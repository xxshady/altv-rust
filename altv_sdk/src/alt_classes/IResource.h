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
Config::Value::ValuePtr GetConfig(const alt::IResource* ptr) {
    return ptr->GetConfig();
}
void EnableNatives(alt::IResource* ptr) {
    return ptr->EnableNatives();
}
void AddGxtText(alt::IResource* ptr, u32 hash, const StdStringClone text) {
    return ptr->AddGxtText(hash, text);
}
void RemoveGxtText(alt::IResource* ptr, u32 hash) {
    return ptr->RemoveGxtText(hash);
}
const StdStringClone GetGxtText(alt::IResource* ptr, u32 hash) {
    return std::string { ptr->GetGxtText(hash) };
}
bool ToggleCursor(alt::IResource* ptr, bool state) {
    return ptr->ToggleCursor(state);
}
void ToggleGameControls(alt::IResource* ptr, bool state) {
    return ptr->ToggleGameControls(state);
}
bool CursorVisible(alt::IResource* ptr) {
    return ptr->CursorVisible();
}
bool GameControlsActive(alt::IResource* ptr) {
    return ptr->GameControlsActive();
}

} // namespace