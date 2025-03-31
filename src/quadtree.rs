use raylib::{color::Color, ffi::Rectangle, prelude::{RaylibDraw, RaylibDrawHandle}};

static MAX_LEVEL: usize = 20;

struct Children (Quadtree, Quadtree, Quadtree, Quadtree);

pub struct Quadtree {
    level: usize,
    bounds: Rectangle,

    children: Option<Box<Children>>,
}

impl Quadtree {
    pub fn new(bounds: Rectangle) -> Self {
        Quadtree {
            level: 0,
            bounds,
            children: None,
        } 
    } 

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        let Some(children) = &self.children else {
            return;
        };

        handle.draw_rectangle_rec(children.0.bounds, Color::BLUE);
        handle.draw_rectangle_rec(children.1.bounds, Color::RED);
        handle.draw_rectangle_rec(children.2.bounds, Color::RED);
        handle.draw_rectangle_rec(children.3.bounds, Color::BLUE);

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

    pub fn test(&mut self, level: usize) {
        if level == MAX_LEVEL {
            return;
        }

        self.split();

        let Some(children) = &mut self.children else {
            return;
        };

        children.0.test(level + 1);
    }
}

