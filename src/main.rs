use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderPbr3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::{InputBundle, StringBindings},
    gltf::{GltfSceneLoaderSystemDesc},
};
use amethyst_physics::PhysicsBundle;
use amethyst_nphysics::NPhysicsBackend;

mod gameplay;
mod entities;
mod systems;
mod component_creators;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_dir = app_root.join("config");
    let assets_dir = app_root.join("assets");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = config_dir.join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_system_desc(GltfSceneLoaderSystemDesc::default(), "gltf_system", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .expect("Cannot load from config")
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderPbr3D::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(PhysicsBundle::<f32, NPhysicsBackend>::new()).unwrap()
        .with(systems::WASDMovementSystem, "wasd_movement_system", &["input_system"]);

    let mut game = Application::new(assets_dir, gameplay::GameplayState, game_data)?;
    game.run();

    Ok(())
}
