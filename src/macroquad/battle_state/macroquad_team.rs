use macroquad::prelude::{draw_text, draw_texture_ex, screen_height, screen_width, DrawTextureParams, Rect, Texture2D, Vec2, WHITE};
use crate::domain::*;


#[derive(Clone)]
pub struct StatsTextures {
    pub stats_background_texture: Texture2D,
    pub stats_border: Texture2D,
    pub stats_label_background: Texture2D,
}

pub struct MacroquadTeam {
    game_team: BattleTeam,
    stats_textures: StatsTextures,
    stats_rectangle: (f32, f32, f32, f32),
    rotation: f32
}

impl MacroquadTeam {
    pub fn new(game_team: &BattleTeam, textures: StatsTextures) -> Self {
        let rotation = match game_team.position() {
            AttackParty(_) => 0.0,
            DefenseParty(_) => std::f32::consts::PI
        };
        let stats_rectangle = Self::stats_rectangle(game_team.position());
        Self{
            game_team: game_team.clone(),
            stats_textures: textures,
            stats_rectangle,
            rotation
        }
    }

    pub fn draw(&self) {
        // let stats_rectangle_to_use = self.stats_rectangle;
        let stats_rectangle_to_use = Self::stats_rectangle(self.game_team.position());
        draw_texture_ex(
            &self.stats_textures.stats_background_texture,
            stats_rectangle_to_use.0,
            stats_rectangle_to_use.1,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(stats_rectangle_to_use.2, stats_rectangle_to_use.3)),
                rotation: self.rotation,
                ..Default::default()
            },
        );

        draw_texture_ex(
            &self.stats_textures.stats_border,
            stats_rectangle_to_use.0,
            stats_rectangle_to_use.1,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(stats_rectangle_to_use.2, stats_rectangle_to_use.3)),
                rotation: self.rotation,
                ..Default::default()
            },
        );
        let label_rectangle = Self::attack_rectangle(stats_rectangle_to_use, self.game_team.position());
        draw_texture_ex(
            &self.stats_textures.stats_label_background,
            label_rectangle.0,
            label_rectangle.1,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(label_rectangle.2, label_rectangle.3)),
                ..Default::default()
            },
        );

        let font_size = label_rectangle.3 * 7.0/10.0;
        draw_text(
            format!("Attack: {}",&self.game_team.current_attack().value().to_string()).as_str(),
            label_rectangle.0 + label_rectangle.2 * 5.0/100.0,
            label_rectangle.1+font_size,
            font_size,
            WHITE
        );
    }

    fn stats_rectangle(position: &ShiaiPosition) -> (f32, f32, f32, f32) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        let w_parts = 46.0;

        let size = (screen_w/ w_parts * 12.0, screen_h/ 7.0);

        let top_left_corner = match position {
            AttackParty(team_position) => {
                match team_position {
                    CaptainTeam => (screen_w/ w_parts * 3.0, screen_h - size.1),
                    SecondTeam => (screen_w/ w_parts * 17.0, screen_h - size.1),
                    ThirdTeam => (screen_w/ w_parts * 31.0, screen_h - size.1),
                }
            },
            DefenseParty(team_position) => {
                match team_position {
                    CaptainTeam => (screen_w/ w_parts * 3.0, 0.0),
                    SecondTeam => (screen_w/ w_parts * 17.0, 0.0),
                    ThirdTeam => (screen_w/ w_parts * 31.0, 0.0),
                }
            }
        };
        (top_left_corner.0, top_left_corner.1, size.0, size.1)
    }

    fn attack_rectangle(stats_rectangle: (f32, f32, f32, f32), position: &ShiaiPosition) -> (f32, f32, f32, f32) {
        let stats_width = stats_rectangle.2;
        let w_parts = stats_width/100.0;

        let top_left_corner_x = w_parts * 7.5 + stats_rectangle.0;
        let size_x = w_parts * 85.0;

        let stats_height =  stats_rectangle.3;
        let h_parts = stats_height/100.0;

        let label_height = 35.00;

        let size_y = h_parts * 21.0;
        let top_left_corner_y = match position {
            AttackParty(_) => h_parts * label_height + stats_rectangle.1,
            DefenseParty(_) => h_parts * (100.0 - label_height) - size_y,
        };



        (top_left_corner_x, top_left_corner_y, size_x, size_y)
    }
}