#![allow(dead_code)]
use raylib::prelude::*;
use std::collections::HashMap;

type Entity = u32;

pub struct ECS {
    next_id: Entity,
    position_pool: ComponentPool<Position>,
    velocity_pool: ComponentPool<Velocity>,
    sprite_pool: ComponentPool<Sprite>,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            position_pool: ComponentPool::new(),
            velocity_pool: ComponentPool::new(),
            sprite_pool: ComponentPool::new(),
        }
    }

    pub fn allocate_entity(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_position(&mut self, id: Entity, x: f32, y: f32) {
        self.position_pool.add(id, Position { x, y });
    }

    pub fn add_velocity(&mut self, id: Entity, dx: f32, dy: f32) {
        self.velocity_pool.add(id, Velocity { dx, dy });
    }
    pub fn add_sprite(&mut self, id: Entity, tex_id: TexHandle) {
        self.sprite_pool.add(id, Sprite { tex_id });
    }

    fn get_position(&self, id: Entity) -> Option<&Position> {
        self.position_pool.get(id)
    }
    fn get_position_mut(&mut self, id: Entity) -> Option<&mut Position> {
        self.position_pool.get_mut(id)
    }

    fn get_velocity(&self, id: Entity) -> Option<&Velocity> {
        self.velocity_pool.get(id)
    }
    fn get_velocity_mut(&mut self, id: Entity) -> Option<&mut Velocity> {
        self.velocity_pool.get_mut(id)
    }
    fn get_sprite(&self, id: Entity) -> Option<&Sprite> {
        self.sprite_pool.get(id)
    }
}

#[derive(Debug, Clone)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
struct Velocity {
    dx: f32,
    dy: f32,
}

// I don't really understand lifetimes. Just trusting the compiler suggestions at this point
struct Sprite {
    tex_id: TexHandle,
}

struct ComponentPool<T> {
    components: Vec<T>,
    entities: Vec<Entity>,
    index_map: HashMap<Entity, usize>,
}

impl<T> ComponentPool<T> {
    fn new() -> Self {
        Self {
            components: Vec::new(),
            entities: Vec::new(),
            index_map: HashMap::new(),
        }
    }

    fn add(&mut self, id: Entity, component: T) {
        if self.index_map.contains_key(&id) {
            return;
        }
        let index = self.components.len();
        self.index_map.insert(id, index);
        self.entities.push(id);
        self.components.push(component);
    }

    fn remove(&mut self, id: Entity) {
        if let Some(&index) = self.index_map.get(&id) {
            let last = self.components.len() - 1;
            self.components.swap(index, last);
            self.entities.swap(index, last);

            // Update index map
            let last_id = self.entities[index];
            self.index_map.insert(last_id, index);
            self.index_map.remove(&id);

            self.components.pop();
            self.entities.pop();
        }
    }

    fn get(&self, id: Entity) -> Option<&T> {
        if let Some(idx) = self.index_map.get(&id) {
            return Some(&self.components[*idx]);
        } else {
            return None;
        }
    }

    fn get_mut(&mut self, id: Entity) -> Option<&mut T> {
        if let Some(idx) = self.index_map.get(&id) {
            return Some(&mut self.components[*idx]);
        } else {
            return None;
        }
    }
}

//NOTE: might not wanan redo the mistake of having systems live in the ECS module
pub fn movement_system(ecs: &mut ECS) {
    for i in 0..ecs.velocity_pool.components.len() {
        let id = ecs.velocity_pool.entities[i];
        let vel = ecs.velocity_pool.components[i].clone();

        if let Some(pos) = ecs.get_position_mut(id) {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }
    }
}

pub fn render_system(
    ecs: &ECS,
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    texture_handler: &TextureHandler,
) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);

    for i in 0..ecs.sprite_pool.components.len() {
        let entity = ecs.sprite_pool.entities[i];

        if let Some(pos) = ecs.get_position(entity) {
            if let Some(sprite) = ecs.get_sprite(entity) {
                if let Some(tex) = texture_handler.get(sprite.tex_id.clone()) {
                    d.draw_texture(&tex, pos.x as i32, pos.y as i32, Color::WHITE);
                }
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum TexHandle {
    SPAM,
}

pub struct TextureHandler {
    textures: HashMap<TexHandle, Texture2D>,
}

impl TextureHandler {
    pub fn new() -> Self {
        TextureHandler {
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        id: TexHandle,
        path: &str,
    ) {
        let texture = rl
            .load_texture(thread, path)
            .expect("Failed to load texture");
        self.textures.insert(id, texture);
    }

    fn get(&self, id: TexHandle) -> Option<&Texture2D> {
        self.textures.get(&id)
    }
}
