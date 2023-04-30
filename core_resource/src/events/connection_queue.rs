use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{helpers::IntoString, sdk, VoidResult};

pub(super) type InfoPtr = *mut sdk::alt::IConnectionInfo;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) enum ConnectionQueueState {
    None,
    Accepted,
    Declined,
    Invalid,
}

#[derive(Debug)]
pub struct ConnectionQueueInfo {
    pub name: String,
    pub id: u32,
    pub social_id: u64,
    pub social_name: String,
    pub hw_id_hash: u64,
    pub hw_id_ex_hash: u64,
    pub auth_token: String,
    pub is_debug: bool,
    pub branch: String,
    pub build: u32,
    pub cdn_url: String,
    pub password_hash: u64,
    pub ip: String,
    pub discord_user_id: i64,
    pub cloud_auth_hash: String,
}

impl ConnectionQueueInfo {
    pub(super) unsafe fn new(ptr: *const sdk::alt::IConnectionInfo) -> Self {
        use sdk::IConnectionInfo::*;
        Self {
            name: GetName(ptr).to_string(),
            id: GetID(ptr),
            social_id: GetSocialId(ptr),
            social_name: GetSocialName(ptr).to_string(),
            hw_id_hash: GetHwIdHash(ptr),
            hw_id_ex_hash: GetHwIdExHash(ptr),
            auth_token: GetAuthToken(ptr).to_string(),
            is_debug: GetIsDebug(ptr),
            branch: GetBranch(ptr).to_string(),
            build: GetBuild(ptr),
            cdn_url: GetCdnUrl(ptr).to_string(),
            password_hash: GetPasswordHash(ptr),
            ip: GetIp(ptr).to_string(),
            discord_user_id: GetDiscordUserID(ptr),
            cloud_auth_hash: GetCloudAuthHash(ptr).to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ConnectionQueueContext {
    info_ptr: InfoPtr,
    state: RefCell<ConnectionQueueState>,
}

impl ConnectionQueueContext {
    pub fn new(info_ptr: InfoPtr) -> Rc<Self> {
        Rc::new(Self {
            info_ptr,
            state: RefCell::new(ConnectionQueueState::None),
        })
    }

    pub fn accept(&self, send_player_names: bool) -> VoidResult {
        self.check_state()?;
        *self.state.try_borrow_mut()? = ConnectionQueueState::Accepted;

        unsafe { sdk::IConnectionInfo::Accept(self.info_ptr, send_player_names) }
        Ok(())
    }

    pub fn decline(&self, reason: impl IntoString) -> VoidResult {
        self.check_state()?;
        *self.state.try_borrow_mut()? = ConnectionQueueState::Declined;

        unsafe { sdk::IConnectionInfo::Decline(self.info_ptr, reason.into_string()) }
        Ok(())
    }

    fn check_state(&self) -> VoidResult {
        let current = *self.state.try_borrow()?;
        if current != ConnectionQueueState::None {
            anyhow::bail!(
                "Failed to accept or decline this player in connection queue, current state: {:?}",
                current
            );
        }
        Ok(())
    }

    pub(self) fn invalidate(&self) -> VoidResult {
        *self.state.try_borrow_mut()? = ConnectionQueueState::Invalid;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ConnectionQueueManager {
    contexts: HashMap<InfoPtr, Rc<ConnectionQueueContext>>,
}

impl ConnectionQueueManager {
    pub(super) fn add(&mut self, ptr: InfoPtr, context: Rc<ConnectionQueueContext>) {
        logger::debug!("adding context ptr: {ptr:?}");
        self.contexts.insert(ptr, context);
    }

    pub(super) fn remove(&mut self, ptr: InfoPtr) {
        logger::debug!("removing context ptr: {ptr:?}");

        let Some(c) = self.contexts.remove(&ptr) else {
            logger::error!("remove unknown context ptr: {ptr:?}");
            return;
        };

        if let Err(e) = c.invalidate() {
            logger::error!("failed to invalidate context ptr: {ptr:?}, error: {e:?}")
        }
    }
}
