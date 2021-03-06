use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::pong::Ball;


pub struct MoveBallsSystem;

// This system handles the movement of balls
impl <'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        );

    fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
        for (ball, local) in (&balls, &mut locals).join() {
            // delta_seconds gives the delta time since the last frame
            local.translate_x(ball.velocity[0] * time.delta_seconds());
            local.translate_y(ball.velocity[1] * time.delta_seconds());
        }
    }
}
