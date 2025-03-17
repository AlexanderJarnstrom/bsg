use raylib::ffi::TraceLogLevel::LOG_NONE;
use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 64;
const SCREEN_HEIGHT: i32 = 32;

fn main() {
    set_trace_log(LOG_NONE);
    
    let (mut rl, thread) = raylib::init().size(640, 480).title("AWESOME GAME").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
    }
}
