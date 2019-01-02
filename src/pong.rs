use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};


pub struct Pong;

// Determines the arena width and height
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
// Determines paddle width and height
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;
// Determines ball attibutes
pub const BALL_VELOCITY_X: f32 = 50.0;
pub const BALL_VELOCITY_Y: f32 = 25.0;
pub const BALL_RADIUS: f32 = 2.0;

// SimpleState handles a lot of the basics, such as handing updates and events,
// which we would have to implement ourselves otherwise.
impl SimpleState for Pong {
    // Will be called on game start
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        // Load the spritesheet necessary to render the graphics.
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Ball>();

        initialize_ball(world, sprite_sheet_handle.clone());
        initialise_paddles(world, sprite_sheet_handle);
        initialize_camera(world);
        initialize_scoreboard(world);
    }
}


// Creates a camera that covers the whole world of the game
fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();

    // move the camera back on the Z-axis, so that it can
    // see all the sprites..
    transform.set_z(1.0);

    // .. and then we create the camera entity
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
                    0.0,
                    ARENA_WIDTH,
                    0.0,
                    ARENA_HEIGHT,
                    )))
        .with(transform)
        .build();
}


// Details the side of the screen for the paddle
#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right
}

// Represents a paddle
pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: 1.0,
            height: 1.0,
        }
    }
}

// By implementing Component, and detailing storage type, we can now add the
// Paddle component to entities.
impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

// Initializes the two paddles
fn initialise_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // position the paddles
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // assign sprites
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    // create the left paddle entity
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    // create the right paddle entity
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Flipped::Horizontal)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}

// Loads the sprite sheet needed to render the graphics
fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
        )
}

pub struct Ball {
    pub velocity: [f32; 2], // 2-element array containing the X and Y velocities
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_ball(world: &mut World, spritesheet_handle: SpriteSheetHandle) {
    // Create the transform
    let mut transform = Transform::default();
    // Ball will be placed center-ish
    transform.set_xyz(ARENA_WIDTH/2.0, ARENA_HEIGHT/2.0, 0.0);

    // set the sprite for the ball
    let sprite_render = SpriteRender {
        sprite_sheet: spritesheet_handle,
        sprite_number: 1,                   // Ball sprite is 2nd in the sheet
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
            radius: BALL_RADIUS,
        })
        .with(transform)
        .build();
}

/// ScoreBoard contains the score data
#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

/// ScoreText contains the text components that displays the score
pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

// initializes the ScoreBoard
fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
        );

    let p1_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle,
        -50.0, -50.0, 1.0, 200.0, 50.0, 0,
        );

    let p2_transform = UiTransform::new(
        "P2".to_string(), Anchor::TopMiddle,
        50.0, -50.0, 1.0, 200.0, 50.0, 0,
        );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
                font.clone(),
                "0".to_string(),
                [1.0, 1.0, 1.0, 1.0],
                50.0,
                ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
                font.clone(),
                "0".to_string(),
                [1.0, 1.0, 1.0, 1.0],
                50.0,
                ))
        .build();

    world.add_resource(ScoreText {p1_score, p2_score});
}
