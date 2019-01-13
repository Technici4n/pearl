use amethyst::{
    assets::{Loader, ProgressCounter},
    core::{nalgebra::Vector3, Transform},
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

use crate::{
    registry::Registry,
    world::{Block, CHUNK_SIZE},
    worldgen::ChunkGenerator,
};

/// State representing the client game
#[derive(Default)]
pub struct Pearl {
    chunk_material: Option<Material>,
    chunk_generator: Option<ChunkGenerator>,
}

impl SimpleState for Pearl {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
        self.initialize_light(world);
        self.initialize_block_registry(world);
        self.initialize_chunk_generator(world);
        self.initialize_chunk_texture(world);
        self.initialize_chunk(world);
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

    fn initialize_block_registry(&mut self, world: &mut World) {
        let mut block_registry = Registry::<Block>::new();
        block_registry.register("default:air", Block { air: true });
        block_registry.register("default:dirt", Block { air: false });
        world.add_resource(block_registry);
    }

    fn initialize_chunk_generator(&mut self, world: &mut World) {
        self.chunk_generator = Some(ChunkGenerator::new(&world.read_resource()));
    }

    fn initialize_chunk_texture(&mut self, world: &mut World) {
        let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
        let texture_storage = world.read_resource();
        let loader = world.read_resource::<Loader>();
        let texture_handle = loader.load(
            "assets/dirt.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        );
        self.chunk_material = Some(Material {
            albedo: texture_handle,
            ..material_defaults
        });
    }

    fn initialize_chunk(&mut self, world: &mut World) {
        let chunk_mesh: MeshHandle = {
            let mesh_storage = world.read_resource();
            let loader = world.read_resource::<Loader>();
            let block_registry = world.read_resource();
            let chunk = self
                .chunk_generator
                .as_ref()
                .expect("initialize_chunk was called before initialize_chunk_generator")
                .generate_chunk(Vector3::from([0, -1, 0]));
            let mesh_data = crate::mesh::chunk::generate_chunk(chunk, &block_registry);
            let mut progress = ProgressCounter::new();
            loader.load_from_data(mesh_data.into(), &mut progress, &mesh_storage)
        };

        let mut transform = Transform::default();
        transform.set_position(Vector3::from([0.0, -1.0 * CHUNK_SIZE as f32, 0.0]));
        world
            .create_entity()
            .with(transform)
            .with(chunk_mesh)
            .with(self.chunk_material.clone().unwrap())
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
    let transform = Transform::default();
    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.0, 0.5)))
        .with(transform)
        .with(ExplorationControlTag::default())
        .build();
}
