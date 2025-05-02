use macroquad::prelude::{draw_text, draw_texture_ex, DrawTextureParams, Rect, Texture2D, Vec2, WHITE};
use crate::domain::*;
use super::*;

#[derive(Clone)]
pub struct TeamTextures {
    pub life_bar_container: Texture2D,
    pub life_bar: Texture2D,
    pub stats_background_texture: Texture2D,
    pub stats_border: Texture2D,
    pub stats_label_background: Texture2D,
}

pub struct MacroquadTeam {
    game_team: BattleTeam,
    textures: TeamTextures,
    team_layout: TeamLayout,
    rotation: f32,
    life_bar_width_percentage: f64

}

const REMOVE_LIFE_SPEED: f64 = 2.0;


impl MacroquadTeam {
    pub fn new(game_team: &BattleTeam, textures: TeamTextures) -> Self {
        let rotation = match game_team.position() {
            AttackParty(_) => 0.0,
            DefenseParty(_) => std::f32::consts::PI
        };
        Self{
            game_team: game_team.clone(),
            textures,
            team_layout: TeamLayout::new(game_team.position()),
            rotation,
            life_bar_width_percentage: 1.0
        }
    }

    fn team_layout(&self) -> TeamLayout {
        // let stats_rectangle_to_use = self.stats_rectangle;
        TeamLayout::new(self.game_team.position())
    }

    pub fn update(&mut self) {
        let current = self.game_team.current_hp().value() as f64;
        let max = self.game_team.original_team().health_points().value() as f64;
        let expected_percentage = current / max;
        if self.life_bar_width_percentage > expected_percentage {
            self.life_bar_width_percentage = self.life_bar_width_percentage - 0.01 * REMOVE_LIFE_SPEED;
        }
    }

    pub fn draw(&self) {
        // TODO decide whether do realtime resize or not.
        let team_layout = self.team_layout();
        self.draw_team_background(team_layout.background_rectangle);
        self.draw_hp(team_layout.hp_rectangle);
        self.draw_attack(team_layout.attack_rectangle);
    }


    fn draw_team_background(&self, background_rectangle: Rect) {
        draw_texture_ex(
            &self.textures.stats_background_texture,
            background_rectangle.x,
            background_rectangle.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(background_rectangle.w, background_rectangle.h)),
                rotation: self.rotation,
                ..Default::default()
            },
        );

        draw_texture_ex(
            &self.textures.stats_border,
            background_rectangle.x,
            background_rectangle.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(background_rectangle.w, background_rectangle.h)),
                rotation: self.rotation,
                ..Default::default()
            },
        );
    }

    fn draw_hp(&self, hp_rectangle: Rect) {
        draw_texture_ex(
            &self.textures.life_bar_container,
            hp_rectangle.x,
            hp_rectangle.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(hp_rectangle.w, hp_rectangle.h)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &self.textures.life_bar,
            hp_rectangle.x,
            hp_rectangle.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    0.0,
                    0.0,
                    self.textures.life_bar.width() * self.life_bar_width_percentage as f32,
                    self.textures.life_bar.height())
                ),
                dest_size: Some(Vec2::new(hp_rectangle.w * self.life_bar_width_percentage as f32, hp_rectangle.h)),
                ..Default::default()
            },
        );
        let font_size = hp_rectangle.h * 6.0/10.0;
        draw_text(
            format!("HP: {}", self.game_team.current_hp().value().to_string()).as_str(),
            hp_rectangle.x + hp_rectangle.w * 30.0/100.0,
            hp_rectangle.y + hp_rectangle.h * 7.0/10.0,
            font_size,
            WHITE
        );
    }

    fn draw_attack(&self, attack_rectangle: Rect) {
        draw_texture_ex(
            &self.textures.stats_label_background,
            attack_rectangle.x,
            attack_rectangle.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(attack_rectangle.w, attack_rectangle.h)),
                ..Default::default()
            },
        );

        let font_size = attack_rectangle.h * 9.0/10.0;
        draw_text(
            format!("Attack: {}",&self.game_team.current_attack().value().to_string()).as_str(),
            attack_rectangle.x + attack_rectangle.w * 5.0/100.0,
            attack_rectangle.y + attack_rectangle.h * 7.0/10.0,
            font_size,
            WHITE
        );
    }
}



