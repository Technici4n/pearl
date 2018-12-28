use amethyst::{
    assets::{Loader, ProgressCounter},
    controls::FlyControlTag,
    core::Transform,
    prelude::*,
    renderer::{
        AmbientColor, Camera, DirectionalLight, Light, Material, MaterialDefaults, MeshHandle,
        PosNormTex, Projection, Shape,
    },
};

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
            let mesh_data = Shape::Cube.generate::<Vec<PosNormTex>>(None);
            let mut progress = ProgressCounter::new();
            self.cube_mesh =
                Some(loader.load_from_data(mesh_data.into(), &mut progress, &mesh_storage));
            let color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            self.cube_material = Some(Material {
                albedo: loader.load_from_data(color.into(), &mut progress, &texture_storage),
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
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(9.0);
    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.0, 0.5)))
        .with(transform)
        .with(FlyControlTag)
        .build();
}
