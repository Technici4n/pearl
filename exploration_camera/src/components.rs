use amethyst::ecs::prelude::{Component, VecStorage};

use super::*;

/// Add this to a camera if you want it to be an exploration camera.
/// You need to add the ExplorationCameraBundle or the required systems for it to work.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct ExplorationControlTag {
    pub pitch: f32,
    pub yaw: f32,
}

impl Component for ExplorationControlTag {
    type Storage = VecStorage<Self>;
}
