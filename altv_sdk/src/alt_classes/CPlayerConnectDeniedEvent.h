#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace CPlayerConnectDeniedEvent {

PlayerConnectDeniedReason GetReason(const alt::CPlayerConnectDeniedEvent* ptr) {
    return static_cast<uint8_t>(ptr->GetReason());
}
const StdStringClone GetName(const alt::CPlayerConnectDeniedEvent* ptr) {
    return std::string { ptr->GetName() };
}
const StdStringClone GetIp(const alt::CPlayerConnectDeniedEvent* ptr) {
    return std::string { ptr->GetIp() };
}
u64 GetPasswordHash(const alt::CPlayerConnectDeniedEvent* ptr) {
    return ptr->GetPasswordHash();
}
bool IsDebug(const alt::CPlayerConnectDeniedEvent* ptr) {
    return ptr->IsDebug();
}
const StdStringClone GetBranch(const alt::CPlayerConnectDeniedEvent* ptr) {
    return std::string { ptr->GetBranch() };
}
u32 GetMajorVersion(const alt::CPlayerConnectDeniedEvent* ptr) {
    return ptr->GetMajorVersion();
}
const StdStringClone GetCdnUrl(const alt::CPlayerConnectDeniedEvent* ptr) {
    return std::string { ptr->GetCdnUrl() };
}
i64 GetDiscordId(const alt::CPlayerConnectDeniedEvent* ptr) {
    return ptr->GetDiscordId();
}

} // namespace