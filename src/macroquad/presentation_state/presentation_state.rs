use macroquad::color::{BLACK, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::{clear_background, draw_texture_ex, screen_height, screen_width, DrawTextureParams, FilterMode, Texture2D};
use crate::macroquad::*;

pub struct PresentationState {
    fade_in: f32,
    logo_texture: Texture2D,
}

impl PresentationState {
    pub async fn new() -> Self {
        let bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/logo.png"));
        let texture = Texture2D::from_file_with_format(bytes, None);
        texture.set_filter(FilterMode::Linear); // Smoother scaling
        Self {
            fade_in: 0.0,
            logo_texture: texture,
        }
    }
}


impl State for PresentationState {
    fn debug(&self) -> &str {
        "PresentationState"
    }

    fn update(&mut self) -> StateTransition {
        self.fade_in += 0.01;
        if self.fade_in > 1.0 {
            self.fade_in = 1.0;
            let battle_state = BattleState::new();

            // Wrap it and return
            StateTransition::Switch(Box::new(battle_state))
        } else {
            StateTransition::None
        }
    }

    fn draw(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let tex_w = self.logo_texture.width();
        let tex_h = self.logo_texture.height();
        let scale = screen_w / tex_w;
        let new_height = tex_h * scale;
        let offset_y = (screen_h - new_height) / 2.0;

        clear_background(BLACK);

        draw_texture_ex(
            &self.logo_texture,
            0.0,
            offset_y,
            WHITE.with_alpha(self.fade_in),
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_w, new_height)),
                ..Default::default()
            },
        );
    }
}