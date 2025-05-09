use macroquad::math::{Rect, Vec2};
use macroquad::prelude::*;
use crate::domain::{AttackParty, CaptainTeam, DefenseParty, SecondTeam, ShiaiPosition, ThirdTeam};

const REMOVE_LIFE_SPEED: f64 = 2.0;

pub struct TeamLayout {
    background_rectangle: Rect,
    hp_rectangle: Rect,
    attack_rectangle: Rect,
    textures: TeamLayoutTextures,
    position: ShiaiPosition,
    rotation: f32,
    total_hp: u128,
    current_hp: u128,
    life_bar_width_percentage: f64,
    current_attack: u128
}

#[derive(Clone)]
pub struct TeamLayoutTextures {
    pub life_bar_container: Texture2D,
    pub life_bar: Texture2D,
    pub stats_background_texture: Texture2D,
    pub stats_border: Texture2D,
    pub stats_label_background: Texture2D,
}

impl TeamLayout {
    pub fn new(
        position: &ShiaiPosition,
        textures: TeamLayoutTextures,
        total_hp: u128,
        current_hp: u128,
        current_attack: u128,
    ) -> Self{
        let background_rectangle = calculate_stats_rectangle(position);
        let hp_rectangle = calculate_hp_rectangle(background_rectangle, position);
        let attack_rectangle = calculate_attack_rectangle(background_rectangle, position);
        let rotation = match position {
            AttackParty(_) => 0.0,
            DefenseParty(_) => std::f32::consts::PI
        };
        Self{
            background_rectangle,
            hp_rectangle,
            attack_rectangle,
            textures,
            position: position.clone(),
            rotation,
            life_bar_width_percentage: 1.0,
            total_hp,
            current_hp,
            current_attack
        }
    }

    pub fn update(&mut self, current_hp: u128) {
        // todo decide the resize yes or not:
        self.resize();

        self.current_hp = current_hp;
        let current =self.current_hp as f64;
        let max = self.total_hp as f64;
        let expected_percentage = current / max;
        if self.life_bar_width_percentage > expected_percentage {
            self.life_bar_width_percentage = self.life_bar_width_percentage - 0.01 * REMOVE_LIFE_SPEED;
        }
    }

    pub fn resize(&mut self) {
        self.background_rectangle = calculate_stats_rectangle(&self.position);
        self.hp_rectangle = calculate_hp_rectangle(self.background_rectangle, &self.position);
        self.attack_rectangle = calculate_attack_rectangle(self.background_rectangle, &self.position);
    }

    pub fn draw(&self) {
        self.draw_team_background(self.background_rectangle);
        self.draw_hp(self.hp_rectangle);
        self.draw_attack(self.attack_rectangle);
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
                    self.textures.life_bar.width(),
                    self.textures.life_bar.height())
                ),
                dest_size: Some(Vec2::new(hp_rectangle.w * self.life_bar_width_percentage as f32, hp_rectangle.h)),
                ..Default::default()
            },
        );
        let font_size = hp_rectangle.h * 6.0/10.0;
        draw_text(
            format!("HP: {}", self.current_hp.to_string()).as_str(),
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
            format!("Attack: {}",&self.current_attack.to_string()).as_str(),
            attack_rectangle.x + attack_rectangle.w * 5.0/100.0,
            attack_rectangle.y + attack_rectangle.h * 7.0/10.0,
            font_size,
            WHITE
        );
    }
}

fn calculate_stats_rectangle(position: &ShiaiPosition) -> Rect {
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
    Rect::new(top_left_corner.0, top_left_corner.1, size.0, size.1)
}

fn calculate_hp_rectangle(stats_rectangle: Rect, position: &ShiaiPosition)-> Rect {
    let stats_width = stats_rectangle.w;
    let w_parts = stats_width/100.0;

    let top_left_corner_x = w_parts * 6.5 + stats_rectangle.x;
    let size_x = w_parts * 87.0;

    let stats_height =  stats_rectangle.h;
    let h_parts = stats_height/100.0;

    let bar_y_position = 12.00;

    let size_y = h_parts * 30.0;
    let top_left_corner_y = match position {
        AttackParty(_) => h_parts * bar_y_position + stats_rectangle.y,
        DefenseParty(_) => h_parts * (100.0 - bar_y_position) - size_y,
    };

    Rect::new(top_left_corner_x, top_left_corner_y, size_x, size_y)
}

fn calculate_attack_rectangle(stats_rectangle: Rect, position: &ShiaiPosition) -> Rect {
    let stats_width = stats_rectangle.w;
    let w_parts = stats_width/100.0;

    let top_left_corner_x = w_parts * 7.5 + stats_rectangle.x;
    let size_x = w_parts * 85.0;

    let stats_height =  stats_rectangle.h;
    let h_parts = stats_height/100.0;

    let label_y_position = 40.00;

    let size_y = h_parts * 21.0;
    let top_left_corner_y = match position {
        AttackParty(_) => h_parts * label_y_position + stats_rectangle.y,
        DefenseParty(_) => h_parts * (100.0 - label_y_position) - size_y,
    };

    Rect::new(top_left_corner_x, top_left_corner_y, size_x, size_y)
}