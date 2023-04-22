use std::ptr::NonNull;

use crate::{
    base_objects::{player, voice_channel},
    helpers,
    resource::Resource,
    sdk, SomeResult, VoidResult,
};

impl voice_channel::VoiceChannel {
    pub fn new_spatial(max_distance: f32) -> SomeResult<voice_channel::VoiceChannelContainer> {
        Self::new(true, max_distance)
    }

    pub fn new_non_spatial() -> SomeResult<voice_channel::VoiceChannelContainer> {
        Self::new(false, 0.0)
    }

    pub(self) fn new(
        spatial: bool,
        max_distance: f32,
    ) -> SomeResult<voice_channel::VoiceChannelContainer> {
        let ptr = unsafe { sdk::ICore::CreateVoiceChannel(spatial, max_distance) };

        if ptr.is_null() {
            anyhow::bail!(
                "Failed to create voice channel\n\
                Maybe you forgot to enable voice in server.toml?\n\
                You can do this by adding the following line:\n\
                ```\n\
                [voice]\n\
                ```\n"
            );
        }

        Ok(voice_channel::add_to_pool!(NonNull::new(ptr).unwrap()))
    }

    pub fn id(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVoiceChannel::GetID(self.raw_ptr()?) })
    }

    pub fn spatial(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVoiceChannel::IsSpatial(self.raw_ptr()?) })
    }

    pub fn max_distance(&self) -> SomeResult<f32> {
        Ok(unsafe { sdk::IVoiceChannel::GetMaxDistance(self.raw_ptr()?) })
    }

    pub fn has_player(&self, player: player::PlayerContainer) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVoiceChannel::HasPlayer(self.raw_ptr()?, player.raw_ptr()?) })
    }

    pub fn add_player(&self, player: player::PlayerContainer) -> VoidResult {
        unsafe { sdk::IVoiceChannel::AddPlayer(self.raw_ptr()?, player.raw_ptr()?) }
        Ok(())
    }

    pub fn remove_player(&self, player: player::PlayerContainer) -> VoidResult {
        unsafe { sdk::IVoiceChannel::RemovePlayer(self.raw_ptr()?, player.raw_ptr()?) }
        Ok(())
    }

    pub fn is_player_muted(&self, player: player::PlayerContainer) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVoiceChannel::IsPlayerMuted(self.raw_ptr()?, player.raw_ptr()?) })
    }

    pub fn mute_player(&self, player: player::PlayerContainer) -> VoidResult {
        unsafe { sdk::IVoiceChannel::MutePlayer(self.raw_ptr()?, player.raw_ptr()?) }
        Ok(())
    }

    pub fn unmute_player(&self, player: player::PlayerContainer) -> VoidResult {
        unsafe { sdk::IVoiceChannel::UnmutePlayer(self.raw_ptr()?, player.raw_ptr()?) }
        Ok(())
    }

    pub fn players(&self) -> SomeResult<Vec<player::PlayerContainer>> {
        let players_raw = unsafe { sdk::IVoiceChannel::GetPlayers(self.raw_ptr()?) };
        Ok(Resource::with(|resource| {
            players_raw
                .into_iter()
                .map(|v| {
                    helpers::get_non_null_player(
                        unsafe { sdk::read_player_ptr_wrapper(v) },
                        resource,
                    )
                })
                .collect()
        }))
    }

    pub fn filter(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVoiceChannel::GetFilter(self.raw_ptr()?) })
    }

    pub fn set_filter(&self, filter: u32) -> VoidResult {
        unsafe { sdk::IVoiceChannel::SetFilter(self.raw_ptr()?, filter) }
        Ok(())
    }

    pub fn priority(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IVoiceChannel::GetPriority(self.raw_ptr()?) })
    }

    pub fn set_priority(&self, priority: i32) -> VoidResult {
        unsafe { sdk::IVoiceChannel::SetPriority(self.raw_ptr()?, priority) }
        Ok(())
    }

    pub fn destroy(&self) -> VoidResult {
        voice_channel::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}
