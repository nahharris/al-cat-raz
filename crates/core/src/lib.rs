use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

pub const PROTOCOL_ID: u64 = 0xA1CA_74A5_0000_0001;

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct NetTransform {
    pub x: f32,
    pub y: f32,
}

pub fn register_replication(app: &mut App) {
    // Replicate a tiny, explicit component to keep the hello world simple.
    // Rendering will read this and drive a visual Transform client-side.
    app.replicate::<NetTransform>();
}
