#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IConnectionInfo {

std::string GetName(const alt::IConnectionInfo* ptr) {
    return ptr->GetName();
}
u64 GetSocialId(const alt::IConnectionInfo* ptr) {
    return ptr->GetSocialId();
}
std::string GetSocialName(const alt::IConnectionInfo* ptr) {
    return ptr->GetSocialName();
}
u64 GetHwIdHash(const alt::IConnectionInfo* ptr) {
    return ptr->GetHwIdHash();
}
u64 GetHwIdExHash(const alt::IConnectionInfo* ptr) {
    return ptr->GetHwIdExHash();
}
std::string GetAuthToken(const alt::IConnectionInfo* ptr) {
    return ptr->GetAuthToken();
}
bool GetIsDebug(const alt::IConnectionInfo* ptr) {
    return ptr->GetIsDebug();
}
std::string GetBranch(const alt::IConnectionInfo* ptr) {
    return ptr->GetBranch();
}
u32 GetBuild(const alt::IConnectionInfo* ptr) {
    return ptr->GetBuild();
}
std::string GetCdnUrl(const alt::IConnectionInfo* ptr) {
    return ptr->GetCdnUrl();
}
u64 GetPasswordHash(const alt::IConnectionInfo* ptr) {
    return ptr->GetPasswordHash();
}
std::string GetIp(const alt::IConnectionInfo* ptr) {
    return ptr->GetIp();
}
i64 GetDiscordUserID(const alt::IConnectionInfo* ptr) {
    return ptr->GetDiscordUserID();
}
std::string GetCloudAuthHash(const alt::IConnectionInfo* ptr) {
    return ptr->GetCloudAuthHash();
}
void Accept(alt::IConnectionInfo* ptr, bool sendNames) {
    return ptr->Accept(sendNames);
}
void Decline(alt::IConnectionInfo* ptr, const StdStringClone reason) {
    return ptr->Decline(reason);
}
bool IsAccepted(const alt::IConnectionInfo* ptr) {
    return ptr->IsAccepted();
}

} // namespace