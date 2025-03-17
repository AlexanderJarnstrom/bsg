use raylib::ffi::TraceLogLevel::LOG_NONE;
use raylib::prelude::*;
pub mod ecs;
use ecs::*;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

fn main() {
    set_trace_log(LOG_NONE);

    let (mut rl, mut thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("AWESOME GAME")
        .build();

    let mut ecs = ECS::new();

    let e1 = ecs.allocate_entity();
    let e2 = ecs.allocate_entity();

    ecs.add_position(e1, 0.0, 0.0);
    ecs.add_velocity(e1, 1.0, 0.5);

    ecs.add_position(e2, 5.0, 5.0);
    ecs.add_velocity(e2, -0.5, -1.0);

    let mut texture_handler = TextureHandler::new();
    texture_handler.load_texture(&mut rl, &thread, TexHandle::SPAM, "resources/Spam.png");
    ecs.add_sprite(e1, TexHandle::SPAM);
    ecs.add_sprite(e2, TexHandle::SPAM);

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        /* INPUT */

        // todo...

        /* UPDATE */
        movement_system(&mut ecs);

        /* RENDERING */
render_system(&ecs, &mut rl, &thread, &texture_handler);
    }
}
