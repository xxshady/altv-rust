use std::{marker::PhantomData, rc::Rc};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    base_objects::{player::PlayerRc, ValidBaseObject},
    helpers::{self, IntoString},
    sdk, SomeResult,
};

pub struct LocalPlayerMetaEntry<V> {
    pub(super) base_object: PlayerRc,
    pub(super) key: String,
    __type: PhantomData<V>,
}

pub trait LocalPlayerMeta: ValidBaseObject
where
    PlayerRc: From<Rc<Self>>,
{
    /// Provides access to read or modify **local** meta of Player.
    ///
    /// # Examples
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::{
    /// #   meta::{BaseObjectMetaEntry, LocalPlayerMeta},
    /// #   BaseObjectPoolFuncs,
    /// # };
    /// # fn test() -> altv::VoidResult {
    /// let player = altv::Player::all()[0].clone();
    ///
    /// let example_entry = player.local_meta_entry("example")?;
    ///
    /// // Set "example" key of player local meta to `123`
    /// example_entry.set(&123)?;
    ///
    /// // Read "example" key of player local meta
    /// let value: Option<i32> = example_entry.get()?; // Some(123)
    /// # Ok(()) }
    /// ```
    fn local_meta_entry<V: Serialize + DeserializeOwned>(
        self: &Rc<Self>,
        key: impl IntoString,
    ) -> SomeResult<LocalPlayerMetaEntry<V>> {
        self.assert_valid()?;
        let player: PlayerRc = self.clone().into();
        Ok(LocalPlayerMetaEntry {
            base_object: player,
            key: key.into_string(),
            __type: PhantomData,
        })
    }

    fn local_meta_keys(self: &Rc<Self>) -> SomeResult<Vec<String>> {
        let player: PlayerRc = self.clone().into();
        Ok(helpers::read_cpp_str_vec(unsafe {
            sdk::IPlayer::GetLocalMetaDataKeys(player.raw_ptr()?)
        }))
    }
}
