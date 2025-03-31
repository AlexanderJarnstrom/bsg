use raylib::{RaylibHandle, ffi::KeyboardKey};

use super::{ECS, Entity};

pub struct Player {
    id: Entity,
    speed: f32,
}

impl Player {
    pub fn new(ecs: &mut ECS) -> Self {
        let id = ecs.allocate_entity();

        ecs.add_position(id, 40.0, 40.0);
        ecs.add_velocity(id, 0.0, 0.0);

        Player { id, speed: 10.0 }
    }

    pub fn input(&self, ecs: &mut ECS, rl: &mut RaylibHandle) {
        let Some(player_velocity) = ecs.get_velocity_mut(self.id) else {
            eprint!("Player should have a velocity.");
            return;
        };

        player_velocity.dy = if rl.is_key_down(KeyboardKey::KEY_W) {
            -self.speed
        } else if rl.is_key_down(KeyboardKey::KEY_S) {
            self.speed
        } else {
            0.0
        };

        player_velocity.dx = if rl.is_key_down(KeyboardKey::KEY_A) {
            -self.speed
        } else if rl.is_key_down(KeyboardKey::KEY_D) {
            self.speed
        } else {
            0.0
        };
    }

    pub fn get_id(&self) -> Entity {
        self.id
    }
}
