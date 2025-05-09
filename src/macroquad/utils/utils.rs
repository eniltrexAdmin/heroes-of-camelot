use macroquad::prelude::*;

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