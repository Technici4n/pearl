use amethyst::{
    controls::{HideCursor, WindowFocus},
    core::{
        nalgebra::{Matrix3, Unit, UnitQuaternion, Vector3},
        shrev::{EventChannel, ReaderId},
        transform::Transform,
        Time,
    },
    ecs::{Join, Read, Resources, System, WriteStorage},
    input::{get_input_axis_simple, InputHandler},
    winit::{DeviceEvent, Event},
};
use std::{f32::consts::PI, hash::Hash, marker::PhantomData};

use super::*;

/// The system that manages the exploration camera's rotation and movement.
///
/// # Type parameters
///
/// * `A`: This is the key the `InputHandler` is using for axes. Often, this is a `String`.
/// * `B`: This is the key the `InputHandler` is using for actions. Often, this is a `String`.
pub struct ExplorationMovementSystem<A, B> {
    /// The movement speed of the system in units per second.
    speed: f32,
    /// The name of the input axis to locally move horizontally in the x coordinates.
    right_input_axis: Option<A>,
    /// The name of the input axis to globally move vertically in the y coordinates.
    up_input_axis: Option<A>,
    /// The name of the input axis to locally move horizontally in the z coordinates.
    forward_input_axis: Option<A>,
    /// Horizontal mouse sensitivity
    sensitivity_x: f32,
    /// Vertical mouse sensitivity
    sensitivity_y: f32,
    event_reader: Option<ReaderId<Event>>,
    _marker: PhantomData<B>,
}

impl<A, B> ExplorationMovementSystem<A, B>
where
    A: Send + Sync + Hash + Eq + Clone + 'static,
    B: Send + Sync + Hash + Eq + Clone + 'static,
{
    /// Builds a new `ExplorationMovementSystem` using the provided speeds and axis controls.
    pub fn new(
        speed: f32,
        right_input_axis: Option<A>,
        up_input_axis: Option<A>,
        forward_input_axis: Option<A>,
        sensitivity_x: f32,
        sensitivity_y: f32,
    ) -> Self {
        Self {
            speed,
            right_input_axis,
            up_input_axis,
            forward_input_axis,
            sensitivity_x,
            sensitivity_y,
            event_reader: None,
            _marker: PhantomData,
        }
    }
}

impl<'a, A, B> System<'a> for ExplorationMovementSystem<A, B>
where
    A: Send + Sync + Hash + Eq + Clone + 'static,
    B: Send + Sync + Hash + Eq + Clone + 'static,
{
    type SystemData = (
        WriteStorage<'a, ExplorationControlTag>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
        Read<'a, InputHandler<A, B>>,
        Read<'a, EventChannel<Event>>,
        Read<'a, WindowFocus>,
        Read<'a, HideCursor>,
    );

    fn run(
        &mut self,
        (mut tag, mut transform, time, input, events, focus, hide): Self::SystemData,
    ) {
        // Update rotation
        let focused = focus.is_focused;
        for event in events.read(
            &mut self
                .event_reader
                .as_mut()
                .expect("`ExplorationMovementSystem::setup` was not called before `ExplorationMovementSystem::run`"),
        ) {
            if focused && hide.hide {
                if let Event::DeviceEvent { ref event, .. } = *event {
                    if let DeviceEvent::MouseMotion { delta: (x, y) } = *event {
                        for (transform, tag) in (&mut transform, &mut tag).join() {
                            tag.yaw -= (x as f32 * self.sensitivity_x).to_radians();
                            tag.pitch -= (y as f32 * self.sensitivity_y).to_radians();

                            // Block the rotation at +/- PI/2 radians.
                            tag.pitch = tag.pitch.max(-PI / 2.0);
                            tag.pitch = tag.pitch.min(PI / 2.0);
                            transform.set_rotation(UnitQuaternion::<f32>::from_euler_angles(
                                tag.pitch, tag.yaw, 0.0,//tag.pitch,//tag.yaw,
                            ));
                        }
                    }
                }
            }
        }

        // Translate
        let x = get_input_axis_simple(&self.right_input_axis, &input);
        let y = get_input_axis_simple(&self.up_input_axis, &input);
        let z = get_input_axis_simple(&self.forward_input_axis, &input);

        if let Some(global_horizontal) = Unit::try_new(Vector3::new(x, 0.0, z), 1.0e-6) {
            for (transform, tag) in (&mut transform, &tag).join() {
                let a = tag.yaw;
                let local_horizontal = Matrix3::new(
                    a.cos(),
                    0.0,
                    -a.sin(),
                    0.0,
                    1.0,
                    0.0,
                    -a.sin(),
                    0.0,
                    -a.cos(),
                ) * (*global_horizontal);
                transform.move_along_global(
                    Unit::try_new(local_horizontal, 1.0e-6)
                        .expect("The rotated unit vector was not a unit vector anymore"),
                    time.delta_seconds() * self.speed,
                );
            }
        }
        if let Some(vertical_direction) = Unit::try_new(Vector3::new(0.0, y, 0.0), 1.0e-6) {
            for (transform, _) in (&mut transform, &tag).join() {
                transform.move_along_global(vertical_direction, time.delta_seconds() * self.speed);
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        use amethyst::ecs::SystemData;

        Self::SystemData::setup(res);
        self.event_reader = Some(res.fetch_mut::<EventChannel<Event>>().register_reader());
    }
}
