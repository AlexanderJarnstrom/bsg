use raylib::{color::Color, ffi::Rectangle, prelude::{RaylibDraw, RaylibDrawHandle}};

pub struct Quadtree {
    level: usize,
    bounds: Rectangle,

    pub nw: Option<Box<Quadtree>>,
    pub ne: Option<Box<Quadtree>>,
    pub sw: Option<Box<Quadtree>>,
    pub se: Option<Box<Quadtree>>,
}

impl Quadtree {
    pub fn new(bounds: Rectangle) -> Self {
        Quadtree {
            level: 0,
            bounds,
            nw: None,
            ne: None,
            sw: None,
            se: None,
        } 
    } 

    fn new_node(&self, bounds: Rectangle) -> Option<Box<Quadtree>> {
        Some(Box::new(Quadtree::new(bounds).set_level(self.level + 1)))
    }

    fn set_level(self, level: usize) -> Self {
        Quadtree { level, bounds: self.bounds, nw: self.nw, ne: self.ne, sw: self.sw, se: self.se }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        let Some(nw) = &self.nw else {
            eprint!("Node {} has un initialized child NW.", self.level);
            return;
        };

        let Some(ne) = &self.ne else {
            eprint!("Node {} has un initialized child NE.", self.level);
            return;
        };

        let Some(sw) = &self.sw else {
            eprint!("Node {} has un initialized child SW.", self.level);
            return;
        };

        let Some(se) = &self.se else {
            eprint!("Node {} has un initialized child SE.", self.level);
            return;
        };

        handle.draw_rectangle_rec(nw.bounds, Color::BLUE);
        handle.draw_rectangle_rec(ne.bounds, Color::RED);
        handle.draw_rectangle_rec(sw.bounds, Color::RED);
        handle.draw_rectangle_rec(se.bounds, Color::BLUE);
    }

    pub fn split(&mut self) {
        let width = self.bounds.width / 2.0;
        let height = self.bounds.height / 2.0;
        let x_off = self.bounds.x + width;
        let y_off = self.bounds.y + height;

        self.nw = self.new_node(Rectangle {
            x: self.bounds.x,
            y: self.bounds.y,
            width,
            height,
        });

        self.ne = self.new_node(Rectangle {
            x: x_off,
            y: self.bounds.y,
            width,
            height,
        });

        self.sw = self.new_node(Rectangle {
            x: self.bounds.x,
            y: y_off,
            width,
            height,
        });

        self.se = self.new_node(Rectangle {
            x: x_off,
            y: y_off,
            width,
            height,
        });
    }
}
