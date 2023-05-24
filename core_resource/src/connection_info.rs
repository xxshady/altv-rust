use crate::{base_objects::connection_info, helpers::IntoString, sdk, SomeResult, VoidResult};

/// # **`ConnectionInfo implementation`**
impl connection_info::ConnectionInfo {
    pub fn accept(&self, send_player_names: bool) -> VoidResult {
        unsafe { sdk::IConnectionInfo::Accept(self.raw_ptr()?, send_player_names) }
        Ok(())
    }

    pub fn is_accepted(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IConnectionInfo::IsAccepted(self.raw_ptr()?) })
    }

    pub fn decline(&self, reason: impl IntoString) -> VoidResult {
        unsafe { sdk::IConnectionInfo::Decline(self.raw_ptr()?, reason.into_string()) }
        Ok(())
    }

    pub fn name(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IConnectionInfo::GetName(self.raw_ptr()?) }.to_string())
    }

    pub fn social_id(&self) -> SomeResult<u64> {
        Ok(unsafe { sdk::IConnectionInfo::GetSocialId(self.raw_ptr()?) })
    }

    pub fn social_name(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IConnectionInfo::GetName(self.raw_ptr()?) }.to_string())
    }

    pub fn hw_id_hash(&self) -> SomeResult<u64> {
        Ok(unsafe { sdk::IConnectionInfo::GetHwIdHash(self.raw_ptr()?) })
    }

    pub fn hw_id_ex_hash(&self) -> SomeResult<u64> {
        Ok(unsafe { sdk::IConnectionInfo::GetHwIdExHash(self.raw_ptr()?) })
    }

    pub fn auth_token(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IConnectionInfo::GetAuthToken(self.raw_ptr()?) }.to_string())
    }

    pub fn is_debug(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IConnectionInfo::GetIsDebug(self.raw_ptr()?) })
    }

    pub fn branch(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IConnectionInfo::GetBranch(self.raw_ptr()?) }.to_string())
    }

    pub fn build(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IConnectionInfo::GetBuild(self.raw_ptr()?) })
    }

    pub fn cdn_url(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IConnectionInfo::GetCdnUrl(self.raw_ptr()?) }.to_string())
    }

    pub fn password_hash(&self) -> SomeResult<u64> {
        Ok(unsafe { sdk::IConnectionInfo::GetPasswordHash(self.raw_ptr()?) })
    }

    pub fn ip(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IConnectionInfo::GetIp(self.raw_ptr()?) }.to_string())
    }

    pub fn discord_user_id(&self) -> SomeResult<i64> {
        Ok(unsafe { sdk::IConnectionInfo::GetDiscordUserID(self.raw_ptr()?) })
    }

    pub fn cloud_auth_hash(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IConnectionInfo::GetCloudAuthHash(self.raw_ptr()?) }.to_string())
    }
}