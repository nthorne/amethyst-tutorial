use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadExpect, System, Write, WriteStorage},
    ui::UiText,
};

use crate::pong::{Ball, ScoreBoard, ScoreText, ARENA_WIDTH};


pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,          // use Write rather than WriteStorage to
                                        // get mutable access to resource item
        ReadExpect<'s, ScoreText>,      // ReadExpect panics if the ScoreText
                                        // does not exist
        );

    fn run(&mut self, (mut balls, mut locals, mut ui_text, mut scores, score_text): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                scores.score_right = (scores.score_right + 1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                scores.score_left = (scores.score_left + 1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                // reverse travel direction
                ball.velocity[0] = -ball.velocity[0];
                // move ball to middle of the playing field
                transform.set_x(ARENA_WIDTH/2.0);

                println!("Score: {:^3} {:^3}", scores.score_left, scores.score_right);
            }
        }
    }
}
