use ecs::{ECS, TexHandle, TextureHandler, player::Player};
use quadtree::Quadtree;
use raylib::{ffi::TraceLogLevel::LOG_NONE, misc::AsF32};

mod ecs;
mod quadtree;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

pub fn run() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("AWESOME GAME")
        .build();

    rl.set_trace_log(LOG_NONE);

    let mut ecs = ECS::new();

    let e1 = ecs.allocate_entity();
    let e2 = ecs.allocate_entity();

    ecs.add_position(e1, 0.0, 0.0);
    ecs.add_velocity(e1, 1.0, 0.5);

    ecs.add_position(e2, 5.0, 5.0);
    ecs.add_velocity(e2, 0.5, 1.0);

    let mut texture_handler = TextureHandler::new();
    texture_handler.load_texture(&mut rl, &thread, TexHandle::SPAM, "resources/Spam.png");
    ecs.add_sprite(e1, TexHandle::SPAM);
    ecs.add_sprite(e2, TexHandle::SPAM);

    let p = Player::new(&mut ecs);
    ecs.add_sprite(p.get_id(), TexHandle::SPAM);

    rl.set_target_fps(60);

    let mut tree = Quadtree::new(raylib::ffi::Rectangle { x: 0.0, y: 0.0, width: SCREEN_WIDTH.as_f32(), height: SCREEN_HEIGHT.as_f32() });
    tree.split();

    while !rl.window_should_close() {
        /* INPUT */
        p.input(&mut ecs, &mut rl);

        /* UPDATE */
        ecs::movement_system(&mut ecs);

        /* RENDERING */
        ecs::render_system(&ecs, &mut rl, &thread, &texture_handler, &tree);
    }
}
