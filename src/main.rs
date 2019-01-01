extern crate amethyst;

mod systems;
mod pong;
use crate::pong::Pong;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline,
                         RenderBundle, Stage};
use amethyst::utils::application_root_dir;
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;

// Returns a amethyst::Result so that we can use `.?` for exit on setup failure.
fn main() -> amethyst::Result<()> {

    // Start the amethyst logger, with a default config so that we get messages
    // and warnings
    amethyst::start_logger(Default::default());

    // Load the display configuration from the Rust Object Notation file
    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    // Load the RON file that contains the bindings, where we bind the keys to
    // the respective axes
    let binding_path = format!(
        "{}/resources/bindings_config.ron",
        application_root_dir()
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    // Render a black background
    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
            );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
            .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        // input_system is a dependency that is defined in the standard
        // InputBundle
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]);

    // Bind the OS event loop and the amethyst components
    let mut game = Application::new("./", Pong, game_data)?;

    // Start the game loop
    game.run();
    Ok(())
}
