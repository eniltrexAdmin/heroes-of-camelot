use macroquad::prelude::Rect;
#[derive(Debug, Clone, PartialEq)]
pub struct RespRect {
    pub base_rect: Rect,         // This is in pixels (your design resolution)
    design_width: f32,       // e.g., 1920
    design_height: f32,      // e.g., 1080
}

// NOTE: this allows for a different base design, it's just comfortable to assume that this game
// is with 1920/1080 resolution, that's why I just hardcode it here. This can be reused with
// other base resolutions

#[derive(Debug, Clone, PartialEq)]
pub enum BaseResolution{
    Default1920x1080,
    Custom(Resolution),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Resolution{
    screen_width: f32,
    screen_height: f32,
}

impl RespRect {
    pub fn new(base_rect: Rect, res: BaseResolution) -> RespRect {
        let (design_width, design_height) = match res {
            BaseResolution::Custom(resolution) => {(resolution.screen_width, resolution.screen_height)},
            BaseResolution::Default1920x1080  => {(1920., 1080.)},
        };
        Self{
            base_rect,
            design_width,
            design_height,
        }
    }

    pub fn to_screen_rect(&self, current_width: f32, current_height: f32) -> Rect {
        let scale_x = current_width / self.design_width;
        let scale_y = current_height / self.design_height;
        Rect {
            x: self.base_rect.x * scale_x,
            y: self.base_rect.y * scale_y,
            w: self.base_rect.w * scale_x,
            h: self.base_rect.h * scale_y,
        }
    }
}