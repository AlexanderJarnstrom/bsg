use ecs::{ECS, player::Player};
use quadtree::Quadtree;
use raylib::{ffi::TraceLogLevel::LOG_NONE, misc::AsF32};
use std::sync::Arc;

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

    ecs.add_position(e1, 10.0, 20.0);
    ecs.add_velocity(e1, 1.0, 1.0);

    ecs.add_position(e2, 20.0, 20.0);
    ecs.add_velocity(e2, 0.2, 0.8);

    // Load texture once and wrap in Arc
    let spam_texture = Arc::new(
        rl.load_texture(&thread, "resources/Spam.png")
            .expect("Failed to load texture"),
    );

    // Share the Arc<SpamTexture> with all entities that use it
    ecs.add_sprite(e1, Arc::clone(&spam_texture));
    ecs.add_sprite(e2, Arc::clone(&spam_texture));

    let p = Player::new(&mut ecs);
    ecs.add_sprite(p.get_id(), Arc::clone(&spam_texture));

    rl.set_target_fps(60);

    let mut tree = Quadtree::new(raylib::ffi::Rectangle {
        x: 0.0,
        y: 0.0,
        width: SCREEN_WIDTH.as_f32(),
        height: SCREEN_HEIGHT.as_f32(),
    });
    tree.add_entity(&ecs, &e1);
    tree.add_entity(&ecs, &e2);
    tree.add_entity(&ecs, &p.get_id());

    while !rl.window_should_close() {
        /* INPUT */
        p.input(&mut ecs, &mut rl);

        /* UPDATE */
        ecs::movement_system(&mut ecs);
        tree.update(&ecs);

        /* RENDERING */
        ecs::render_system(&ecs, &mut rl, &thread, &tree);
    }
}
