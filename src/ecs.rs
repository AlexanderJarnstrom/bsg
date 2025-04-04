#![allow(dead_code)]
use raylib::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

use crate::quadtree::Quadtree;

pub mod player;

pub type Entity = u32;

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

    pub fn add_sprite(&mut self, id: Entity, texture: Arc<Texture2D>) {
        self.sprite_pool.add(id, Sprite { texture });
    }

    pub fn get_position(&self, id: Entity) -> Option<&Position> {
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
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
struct Velocity {
    dx: f32,
    dy: f32,
}

struct Sprite {
    texture: Arc<Texture2D>,
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

            let last_id = self.entities[index];
            self.index_map.insert(last_id, index);
            self.index_map.remove(&id);

            self.components.pop();
            self.entities.pop();
        }
    }

    fn get(&self, id: Entity) -> Option<&T> {
        self.index_map.get(&id).map(|&idx| &self.components[idx])
    }

    fn get_mut(&mut self, id: Entity) -> Option<&mut T> {
        self.index_map
            .get(&id)
            .map(|&idx| &mut self.components[idx])
    }
}

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

pub fn render_system(ecs: &ECS, rl: &mut RaylibHandle, thread: &RaylibThread, tree: &Quadtree) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);

    tree.draw(&mut d);

    for i in 0..ecs.sprite_pool.components.len() {
        let entity = ecs.sprite_pool.entities[i];

        if let Some(pos) = ecs.get_position(entity) {
            if let Some(sprite) = ecs.get_sprite(entity) {
                d.draw_texture(&*sprite.texture, pos.x as i32, pos.y as i32, Color::WHITE);
            }
        }
    }
}
