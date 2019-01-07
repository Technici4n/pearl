use amethyst::{
    assets::{Loader, ProgressCounter},
    core::Transform,
    ecs::prelude::{Join, WriteStorage},
    input::is_close_requested,
    prelude::*,
    renderer::{
        AmbientColor, Camera, DirectionalLight, Light, Material, MaterialDefaults, MeshHandle,
        PngFormat, Projection, TextureMetadata,
    },
    winit::{Event, WindowEvent},
};
use exploration_camera::ExplorationControlTag;

/// State representing the client game
#[derive(Default)]
pub struct Pearl {
    cube_mesh: Option<MeshHandle>,
    cube_material: Option<Material>,
}

impl SimpleState for Pearl {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
        self.initialize_cube(world);
        self.initialize_light(world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        state_event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &state_event {
            // Close the window if necessary
            if is_close_requested(&event) {
                return Trans::Quit;
            }
            // Adjust camera aspect ratio if the window's ratio was changed
            if let Event::WindowEvent {
                event: window_event,
                ..
            } = event
            {
                if let WindowEvent::Resized(logical_size) = window_event {
                    self.update_camera_ratio(
                        data.world,
                        (logical_size.width / logical_size.height) as f32,
                    );
                }
            }
        }
        Trans::None
    }
}

impl Pearl {
    fn initialize_light(&mut self, world: &mut World) {
        world
            .create_entity()
            .with(Transform::default())
            .with(Light::Directional(DirectionalLight {
                color: [0.1, 0.1, 0.1, 0.05].into(),
                direction: [-1.0, -2.0, -3.0],
            }))
            .build();
        world.add_resource(AmbientColor([0.4, 0.4, 0.4, 1.0].into()));
    }

    fn initialize_cube(&mut self, world: &mut World) {
        {
            let mesh_storage = world.read_resource();
            let texture_storage = world.read_resource();
            let loader = world.read_resource::<Loader>();
            // let mesh_data = Shape::Cube.generate::<Vec<PosNormTex>>(None);
            let mesh_data = crate::mesh::cube::generate_cube();
            println!("{:?}", mesh_data);
            let mut progress = ProgressCounter::new();
            self.cube_mesh =
                Some(loader.load_from_data(mesh_data.into(), &mut progress, &mesh_storage));
            let texture_handle = loader.load(
                "assets/dirt.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &texture_storage,
            );
            // let color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            self.cube_material = Some(Material {
                albedo: texture_handle, //loader.load_from_data(color.into(), &mut progress, &texture_storage),
                ..world.read_resource::<MaterialDefaults>().0.clone()
            });
        }

        let transform = Transform::default();
        world
            .create_entity()
            .with(transform)
            .with(self.cube_mesh.clone().unwrap())
            .with(self.cube_material.clone().unwrap())
            .build();
    }

    fn update_camera_ratio(&mut self, world: &mut World, ratio: f32) {
        let new_camera = Camera::from(Projection::perspective(ratio, 0.5));
        world.exec(|(mut cameras,): (WriteStorage<Camera>,)| {
            for c in (&mut cameras).join() {
                *c = new_camera.clone();
            }
        });
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(9.0);
    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.0, 0.5)))
        .with(transform)
        .with(ExplorationControlTag::default())
        .build();
}
