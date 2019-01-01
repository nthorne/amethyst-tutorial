use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

pub struct PaddleSystem;

fn mov(amount: f64, transform: &mut Transform) {
    // scale the movement to make it seem smooth
    let scaled = 1.2 * amount as f32;

    let paddle_y = transform.translation().y;

    // set the y position of the paddle, clamping it to be within the arena
    // size, adjusted for the paddle height.
    transform.set_y((paddle_y+scaled)
                    .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                    .max(PADDLE_HEIGHT * 0.5),
                    );
}

// Declares the System for the paddles
impl<'s> System<'s> for PaddleSystem {
    // These are the data that the System operates on
    type SystemData = (
        WriteStorage<'s, Transform>,            // we mutate Transform components
        ReadStorage<'s, Paddle>,                // .. we read Paddle components
        Read<'s, InputHandler<String, String>>, // .. and we read from InputHandler
                                                //    type parameters need to match
                                                //    with main.rs
        );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        // iterate over entities that have both a Paddle and Transform attached
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            // use the Paddle's side attribute to determine which axis
            // we're supposed to fetch the movement from
            let amount = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };

            // map the movement function over the axis value, if Some
            amount.map(|mv_amount| mov(mv_amount, transform));
        }
    }
}
