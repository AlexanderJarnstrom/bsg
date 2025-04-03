use std::clone;

use raylib::{color::Color, ffi::Rectangle, prelude::{RaylibDraw, RaylibDrawHandle}};

use crate::ecs::{Entity, Position, ECS};

static MAX_LEVEL: usize = 20;

struct Children (Quadtree, Quadtree, Quadtree, Quadtree);

pub struct Quadtree {
    level: usize,
    bounds: Rectangle,

    children: Option<Box<Children>>,
    entities: Vec<Entity>,
}

impl Quadtree {
    pub fn new(bounds: Rectangle) -> Self {
        Quadtree {
            level: 0,
            bounds,
            children: None,
            entities: Vec::new(),
        } 
    } 

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        let Some(children) = &self.children else {
            return;
        };

        handle.draw_rectangle_lines_ex(children.0.bounds, 0.5, Color::WHITE);
        handle.draw_rectangle_lines_ex(children.1.bounds, 0.5, Color::WHITE);
        handle.draw_rectangle_lines_ex(children.2.bounds, 0.5, Color::WHITE);
        handle.draw_rectangle_lines_ex(children.3.bounds, 0.5, Color::WHITE);

        children.0.draw(handle);
        children.1.draw(handle);
        children.2.draw(handle);
        children.3.draw(handle);
    }

    pub fn split(&mut self) {
        let x = self.bounds.x;
        let y = self.bounds.y;
        let width = self.bounds.width / 2.0;
        let height = self.bounds.height / 2.0;
        let x_off = self.bounds.x + width;
        let y_off = self.bounds.y + height;

        self.children = Some(Box::new(Children(
            Quadtree::new(Rectangle { x, y, width, height }).set_level(self.level + 1),
            Quadtree::new(Rectangle { x: x_off, y, width, height }).set_level(self.level + 1),
            Quadtree::new(Rectangle { x, y: y_off, width, height }).set_level(self.level + 1),
            Quadtree::new(Rectangle { x: x_off, y: y_off, width, height }).set_level(self.level + 1),
        )));
    }

    fn set_level(mut self, level: usize) -> Self {
        self.level = level;
        self
    }

    pub fn add_entity(&mut self, ecs: &ECS, entity: &Entity) {
        let Some(entity_bounds) = ecs.get_position(entity.clone()) else {
            return;
        };

        if let Some(children) = &mut self.children {

            if children.0.is_overlapping(&entity_bounds) {
                children.0.add_entity(&ecs, &entity);
            }else if children.1.is_overlapping(&entity_bounds) {
                children.1.add_entity(&ecs, &entity);
            }else if children.2.is_overlapping(&entity_bounds) {
                children.2.add_entity(&ecs, &entity);
            }else if children.3.is_overlapping(&entity_bounds) {
                children.3.add_entity(&ecs, &entity);
            }
        } else {
            if self.entities.len() > 0 {
                self.split();
                self.add_entity(ecs, entity);
                self.add_entity(ecs, &self.entities.first().unwrap().clone());

                self.entities.clear();
            } else {
                self.entities.push(entity.clone());
            }
        }

    }

    fn get_all_entities(&self) -> Vec<Entity> {
        let Some(children) = &self.children else {

            if let Some(entity) = self.entities.first() {
                return vec![entity.clone()];
            }

            return Vec::new();
        };

        let mut entities: Vec<Entity> = Vec::new();

        entities.append(&mut children.0.get_all_entities());
        entities.append(&mut children.1.get_all_entities());
        entities.append(&mut children.2.get_all_entities());
        entities.append(&mut children.3.get_all_entities());

        return entities;
    }

    pub fn update(&mut self, ecs: &ECS) {
        let entities = self.get_all_entities();

        self.children = None;

        for entity in entities {
            self.add_entity(ecs, &entity);
        }
    }

    fn is_overlapping(&self, entity_bounds: &Position) -> bool {
        if entity_bounds.x < self.bounds.x {
            return false;
        } else if entity_bounds.x > self.bounds.x + self.bounds.width {
            return false;
        } else if entity_bounds.y < self.bounds.y {
            return false;
        } else if entity_bounds.y > self.bounds.y + self.bounds.height {
            return false;
        }

        true
    }
}

