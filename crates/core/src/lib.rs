use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

pub const PROTOCOL_ID: u64 = 0xA1CA_74A5_0000_0001;

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct NetTransform {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Player {
    /// Renet client ID used to identify the local player on clients.
    /// This should match the network ID assigned to the connection on the server.
    pub network_id: u64,
}

#[derive(Message, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PlayerInputCommand {
    pub x: i8,
    pub y: i8,
    pub sprint: bool,
    pub dash: bool,
}

pub fn register_replication(app: &mut App) {
    // Keep replication explicit and small: server sends transforms + player identity only.
    app.replicate::<NetTransform>();
    app.replicate::<Player>();
}
