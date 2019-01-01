extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Event, Pipeline,
                         RenderBundle, Stage, VirtualKeyCode};
use amethyst::utils::application_root_dir;

pub struct Pong;

/**
 * SimpleState handles a lot of the basics, such as handing updates and events,
 * which we would have to implement ourselves otherwise.
 */
impl SimpleState for Pong {

}

// Returns a amethyst::Result so that we can use `.?` for exit on setup failure.
fn main() -> amethyst::Result<()> {

    // Start the amethyst logger, with a default config so that we get messages
    // and warnings
    amethyst::start_logger(Default::default());

    // Load the display configuration from the Rust Object Notation file
    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

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
        )?;

    // Bind the OS event loop and the amethyst components
    let mut game = Application::new("./", Pong, game_data)?;

    // Start the game loop
    game.run();
    Ok(())
}
