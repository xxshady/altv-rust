use std::rc::Rc;

use crate::{
    base_objects::{player::PlayerContainer, ValidBaseObject},
    helpers::IntoString,
    SomeResult,
};

pub struct LocalPlayerMetaEntry {
    pub(super) base_object: PlayerContainer,
    pub(super) key: String,
}

pub trait LocalPlayerMeta: ValidBaseObject
where
    PlayerContainer: From<Rc<Self>>,
{
    /// Provides access to read or modify local meta of Player.
    ///
    /// # Examples
    /// ```rust
    /// let player = altv::Player::all()[0].clone();
    ///
    /// // Set "example" key of local meta to `123`
    /// player.local_meta_entry("example")?.set(123)?;
    ///
    /// // Read "example" key of local meta
    /// player.local_meta_entry("example")?.get()?; // Some(MValue::I64(123))
    /// ```
    fn local_meta_entry(self: &Rc<Self>, key: impl IntoString) -> SomeResult<LocalPlayerMetaEntry> {
        self.assert_valid()?;
        Ok(LocalPlayerMetaEntry {
            base_object: self.clone().into(),
            key: key.into_string(),
        })
    }
}
