extern crate amethyst;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawShaded, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};
use exploration_camera::ExplorationCameraBundle;

mod mesh;
mod pearl;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();

    let key_bindings_path = format!("{}/resources/keybindings.ron", app_root);

    let path = format!("{}/resources/display_config.ron", app_root);
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawShaded::<PosNormTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            ExplorationCameraBundle::<String, String>::new(
                Some(String::from("move_x")),
                Some(String::from("move_y")),
                Some(String::from("move_z")),
            )
            .with_sensitivity(0.2, 0.2),
        )?
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(TransformBundle::new().with_dep(&["exploration_camera_movement"]))?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::new("./", pearl::Pearl::default(), game_data)?;

    game.run();

    Ok(())
}
