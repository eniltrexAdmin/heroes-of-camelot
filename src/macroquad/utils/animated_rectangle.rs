use macroquad::math::Rect;
use crate::macroquad::{modify_rectangle, BaseResolution, RespRect};

pub struct AnimatedRectangle {
    init_rectangle: Rect,
    current_rectangle: RespRect,
    target_rectangle: Rect,
    pub animation_speed: f32,
}
impl AnimatedRectangle {
    pub fn new(default_rectangle: Rect, target_rectangle: Rect, speed: f32, res: BaseResolution)
        -> AnimatedRectangle {
        Self{
            current_rectangle: RespRect::new(default_rectangle.clone(), res),
            init_rectangle: default_rectangle,
            target_rectangle,
            animation_speed: speed,
        }
    }

    pub fn animate(&mut self) {
        self.current_rectangle.base_rect = modify_rectangle(
            self.current_rectangle.base_rect,
            self.target_rectangle, self.animation_speed,
        );
    }

    pub fn is_moving(&self) -> bool {
        self.current_rectangle.base_rect != self.target_rectangle
    }

    pub fn reset(&mut self) {
        self.current_rectangle.base_rect = self.init_rectangle;
    }

    pub fn rectangle(&self) -> &RespRect {
        &self.current_rectangle
    }
}