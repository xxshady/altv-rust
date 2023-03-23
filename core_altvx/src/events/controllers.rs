use crate::player::PlayerContainer;

#[derive(Debug)]
pub struct ServerStarted {}

#[derive(Debug)]
pub struct ColshapeEvent {}

#[derive(Debug)]
pub struct VehicleEnterColShape {}

#[derive(Debug)]
pub struct PlayerConnect {
    pub player: PlayerContainer,
}
