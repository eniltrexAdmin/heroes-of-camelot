use macroquad::prelude::*;
use crate::macroquad::AnimatedRectangle;

pub fn macroquad_draw_background(background: &Texture2D) {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let tex_w = background.width();
    let tex_h = background.height();
    let scale = screen_w / tex_w;
    let new_height = tex_h * scale;
    let offset_y = (screen_h - new_height) / 2.0;

    clear_background(BLACK);

    draw_texture_ex(
        background,
        0.0,
        offset_y,
        WHITE.with_alpha(1.0),
        DrawTextureParams {
            dest_size: Some(Vec2::new(screen_w, new_height)),
            ..Default::default()
        },
    );
}

pub fn draw_texture_in_rectangle(texture: &Texture2D, rect: Rect) {
    draw_texture_ex(
        texture,
        rect.x,
        rect.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(rect.w, rect.h)),
            ..Default::default()
        },
    );
}

pub fn draw_texture_in_animated_rectangle(texture: &Texture2D, rect: &AnimatedRectangle) {
    draw_texture_in_rectangle(
        texture,
        rect.rectangle().to_screen_rect(
            screen_width(),
            screen_height()
        )
    );
}

// TODO test
pub fn scale_rectangle(input: Rect, factor: f32) -> Rect {
    let (w,h) = (input.w * factor, input.h * factor);
    Rect::new(
        input.x + (input.w - w) / 2.0,
        input.y + (input.h - h) / 2.0,
        w,
        h
    )
}

// TODO test
pub fn modify_rectangle(current: Rect, target: Rect, speed: f32) -> Rect {
    let dx = target.x - current.x;
    let dy = target.y - current.y;
    let dw = target.w - current.w;
    let dh = target.h - current.h;

    let distance = (dx * dx + dy * dy + dw * dw + dh * dh).sqrt();

    if distance < speed || distance == 0.0 {
        return target; // Close enough, snap to target
    }


    // Normalize direction vector and scale by speed
    let scale = speed / distance;

    Rect::new(
        current.x + dx * scale,
        current.y + dy * scale,
        current.w + dw * scale,
        current.h + dh * scale,
    )
}

fn target_attribute(current: f32, target:f32, speed: f32) -> f32 {
    let mut result = current;
    if result < target {
        result = result + speed;
        if result > target {
            result = target;
        }
    } else {
        result = result - speed;
        if result < target {
            result = target;
        }
    }
    result
}