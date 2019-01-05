use amethyst::{
    controls::{CursorHideSystem, MouseFocusUpdateSystem},
    core::{Result, SystemBundle},
    ecs::DispatcherBuilder,
};
use std::{hash::Hash, marker::PhantomData};

use super::*;

/// The bundle that creates an exploration movement system.
///
/// Note: Will not actually create a moving entity. It will only register the needed resources and
/// systems. The generic parameters `A` and `B` are the ones used in `InputHandler<A,B>`.
///
/// You might want to add `"exploration_camera_movement"` as dependencies of the
/// `TransformSystem` in order to apply changes made by these systems in the same frame.
/// Adding this bundle will grab the mouse, hide it and keep it centered.
///
/// # Type parameters
///
/// * `A`: This is the key the `InputHandler` is using for axes. Often, this is a `String`.
/// * `B`: This is the key the `InputHandler` is using for actions. Often, this is a `String`.
///
/// # Systems
///
/// This bundle adds the following systems:
///
/// * `MovementSystem` (from this crate)
/// * `MouseFocusUpdateSystem` (from amethyst::controls)
/// * `CursorHideSystem` (from amethyst::controls)
pub struct ExplorationCameraBundle<A, B> {
    sensitivity_x: f32,
    sensitivity_y: f32,
    speed: f32,
    right_input_axis: Option<A>,
    up_input_axis: Option<A>,
    forward_input_axis: Option<A>,
    _marker: PhantomData<B>,
}

impl<A, B> ExplorationCameraBundle<A, B> {
    /// Builds a new exploration camera bundle using the provided axes as controls.
    pub fn new(
        right_input_axis: Option<A>,
        up_input_axis: Option<A>,
        forward_input_axis: Option<A>,
    ) -> Self {
        Self {
            sensitivity_x: 1.0,
            sensitivity_y: 1.0,
            speed: 1.0,
            right_input_axis,
            up_input_axis,
            forward_input_axis,
            _marker: PhantomData,
        }
    }

    /// Alters the mouse sensitivy on this `ExplorationCameraBundle`
    pub fn with_sensitivity(mut self, x: f32, y: f32) -> Self {
        self.sensitivity_x = x;
        self.sensitivity_y = y;
        self
    }

    /// Alters the speed on this `ExplorationCameraBundle`.
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
}

impl<'a, 'b, A, B> SystemBundle<'a, 'b> for ExplorationCameraBundle<A, B>
where
    A: Send + Sync + Hash + Eq + Clone + 'static,
    B: Send + Sync + Hash + Eq + Clone + 'static,
{
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            ExplorationMovementSystem::<A, B>::new(
                self.speed,
                self.right_input_axis,
                self.up_input_axis,
                self.forward_input_axis,
                self.sensitivity_x,
                self.sensitivity_y,
            ),
            "exploration_camera_movement",
            &[],
        );
        builder.add(MouseFocusUpdateSystem::new(), "mouse_focus", &[]);
        builder.add(CursorHideSystem::new(), "cursor_hide", &["mouse_focus"]);
        Ok(())
    }
}
