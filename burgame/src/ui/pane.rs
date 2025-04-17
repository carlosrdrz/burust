use std::any::Any;

use burengine::graphics::Graphics;
use burengine::types::{Rect, Color};
use lazy_static::lazy_static;

use super::{Drawable, Widget};

// GUI constants
const CORNER_SIZE: u32 = 26;
const CORNER_OFFSET: u32 = CORNER_SIZE / 2;
const BORDER_OFFSET: i32 = 13;  // Half of CORNER_SIZE, used for border positioning
const BG_PADDING: i32 = 3;      // Padding for the background rectangle
const BG_COLOR: Color = Color::RGBA(114, 107, 82, 255);

// Sprite sheet coordinates
const VERTICAL_BORDER_X: i32 = 104;
const VERTICAL_BORDER_Y: i32 = 0;
const HORIZONTAL_BORDER_X: i32 = 130;
const HORIZONTAL_BORDER_Y: i32 = 0;
const CORNER_SPRITES: [(i32, i32); 4] = [
    (26, 0),  // Top-left
    (0, 0),   // Top-right
    (78, 0),  // Bottom-left
    (52, 0),  // Bottom-right
];

lazy_static! {
    static ref SPRITE_RECTS: (Rect, Rect, [Rect; 4], [(i32, i32); 4]) = {
        let vertical = Rect::new(VERTICAL_BORDER_X, VERTICAL_BORDER_Y, 1, CORNER_SIZE);
        let horizontal = Rect::new(HORIZONTAL_BORDER_X, HORIZONTAL_BORDER_Y, CORNER_SIZE, 1);
        let corners = [
            Rect::new(CORNER_SPRITES[0].0, CORNER_SPRITES[0].1, CORNER_SIZE, CORNER_SIZE),   // Top-left
            Rect::new(CORNER_SPRITES[1].0, CORNER_SPRITES[1].1, CORNER_SIZE, CORNER_SIZE),   // Top-right
            Rect::new(CORNER_SPRITES[2].0, CORNER_SPRITES[2].1, CORNER_SIZE, CORNER_SIZE),   // Bottom-left
            Rect::new(CORNER_SPRITES[3].0, CORNER_SPRITES[3].1, CORNER_SIZE, CORNER_SIZE),   // Bottom-right
        ];
        let corner_offsets = [
            (-(CORNER_OFFSET as i32), -(CORNER_OFFSET as i32)),
            (CORNER_OFFSET as i32, -(CORNER_OFFSET as i32)),
            (-(CORNER_OFFSET as i32), CORNER_OFFSET as i32),
            (CORNER_OFFSET as i32, CORNER_OFFSET as i32),
        ];
        (vertical, horizontal, corners, corner_offsets)
    };
}

pub struct WidgetWrapper {
    widget: Box<dyn Widget>,
}

impl WidgetWrapper {
    pub fn as_mut_widget<T>(&mut self) -> &mut T
    where
        T: Widget + 'static,
    {
        return self.widget.as_mut_any().downcast_mut::<T>().unwrap()
    }
}

impl Drawable for WidgetWrapper {
    fn draw(&self, graphics: &mut Graphics) {
        self.widget.draw(graphics);
    }
}

pub struct Pane {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: Color,
    widgets: Vec<WidgetWrapper>,
}

impl Pane {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Pane {
        Self { x, y, width, height, color: BG_COLOR, widgets: Vec::new() }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        let wrapper = WidgetWrapper { widget };
        self.widgets.push(wrapper)
    }

    pub fn get_widget_mut(&mut self, id: usize) -> &mut WidgetWrapper {
        return self.widgets.get_mut(id).unwrap()
    }

    fn draw_border(&self, graphics: &mut Graphics, is_horizontal: bool) {
        let (vertical, horizontal, _, _) = &*SPRITE_RECTS;
        let (src, positions) = if is_horizontal {
            let src = *vertical;
            let positions = [
                // Top border
                (
                    self.x + BORDER_OFFSET,
                    self.y - BORDER_OFFSET,
                ),
                // Bottom border
                (
                    self.x + BORDER_OFFSET,
                    self.y + (self.height as i32) - BORDER_OFFSET,
                ),
            ];
            (src, positions)
        } else {
            let src = *horizontal;
            let positions = [
                // Left border
                (
                    self.x - BORDER_OFFSET,
                    self.y + BORDER_OFFSET,
                ),
                // Right border
                (
                    self.x + (self.width as i32) - BORDER_OFFSET,
                    self.y + BORDER_OFFSET,
                ),
            ];
            (src, positions)
        };

        let size = if is_horizontal {
            self.width - CORNER_SIZE
        } else {
            self.height - CORNER_SIZE
        };

        for (x, y) in positions.iter() {
            let dst = if is_horizontal {
                Rect::new(*x, *y, size, CORNER_SIZE)
            } else {
                Rect::new(*x, *y, CORNER_SIZE, size)
            };
            graphics.draw_texture("resources/sprites/gui.png", src, dst);
        }
    }

    fn draw_corner(&self, graphics: &mut Graphics, corner_index: usize, x: i32, y: i32) {
        let (_, _, corners, corners_offsets) = &*SPRITE_RECTS;
        let src = corners[corner_index];
        let offsets = corners_offsets[corner_index];
        let dst = Rect::new(x + offsets.0, y + offsets.1, CORNER_SIZE, CORNER_SIZE);
        graphics.draw_texture("resources/sprites/gui.png", src, dst);
    }
}

impl Widget for Pane {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Drawable for Pane {
    fn draw(&self, graphics: &mut Graphics) {
        // Draw background
        let bg_rect = Rect::new(
            self.x + BG_PADDING,
            self.y + BG_PADDING,
            self.width - (BG_PADDING as u32 * 2),
            self.height - (BG_PADDING as u32 * 2)
        );
        graphics.draw_rect(bg_rect, self.color);

        // Draw borders
        self.draw_border(graphics, true);  // Horizontal borders
        self.draw_border(graphics, false); // Vertical borders

        // Draw corners
        self.draw_corner(graphics, 0,  // Top-left
            self.x, 
            self.y
        );
        self.draw_corner(graphics, 1,  // Top-right
            self.x + (self.width as i32) - (CORNER_SIZE as i32),
            self.y
        );
        self.draw_corner(graphics, 2,  // Bottom-left
            self.x,
            self.y + (self.height as i32) - (CORNER_SIZE as i32)
        );
        self.draw_corner(graphics, 3,  // Bottom-right
            self.x + (self.width as i32) - (CORNER_SIZE as i32),
            self.y + (self.height as i32) - (CORNER_SIZE as i32)
        );

        // Draw child widgets
        for widget in self.widgets.iter() {
            widget.draw(graphics);
        }
    }
}
