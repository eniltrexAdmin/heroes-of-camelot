use macroquad::math::{Rect, Vec2};
use macroquad::prelude::*;
use crate::domain::{AttackParty, CaptainTeam, DefenseParty, SecondTeam, BattlePosition, ThirdTeam};
use crate::macroquad::{draw_texture_in_animated_rectangle, scale_rectangle, AnimatedRectangle, Default1920x1080};

const REMOVE_LIFE_SPEED: f64 = 2.0;

#[derive(Debug)]
pub enum TeamLayoutAnimation{
    Active,
    Damage
}
pub struct TeamLayout {
    background_animated_rectangle: AnimatedRectangle,
    hp_rectangle_animated: AnimatedRectangle,
    attack_rectangle_animated: AnimatedRectangle,
    textures: TeamLayoutTextures,
    position: BattlePosition,
    rotation: f32,
    total_hp: u128,
    current_hp: u128,
    life_bar_width_percentage: f64,
    current_attack: u128,
    current_animation: Option<TeamLayoutAnimation>,
    animation_finished: bool,
}

#[derive(Clone)]
pub struct TeamLayoutTextures {
    pub life_bar_container: Texture2D,
    pub life_bar: Texture2D,
    pub stats_background_texture: Texture2D,
    pub stats_border: Texture2D,
    pub stats_label_background: Texture2D,
}

pub const SPEED: f32 = 5.0;

impl TeamLayout {
    pub fn new(
        position: &BattlePosition,
        textures: TeamLayoutTextures,
        total_hp: u128,
        current_hp: u128,
        current_attack: u128,
    ) -> Self{
        let background_rectangle = calculate_stats_rectangle(position);
        let hp_rectangle = calculate_hp_rectangle(background_rectangle, position);
        let attack_rectangle = attack_rectangle_hardcoded_default(position);
        let rotation = match position {
            AttackParty(_) => 0.0,
            DefenseParty(_) => std::f32::consts::PI
        };

        let background_animated_rectangle =  AnimatedRectangle::new(
            background_rectangle,
            scale_rectangle(background_rectangle, 120.0/100.0),
            SPEED,
            Default1920x1080
        );

        let attack_rectangle_animated =  AnimatedRectangle::new(
            attack_rectangle,
            scale_rectangle(attack_rectangle, 120.0/100.0),
            SPEED,
            Default1920x1080
        );

        let hp_rectangle_animated =  AnimatedRectangle::new(
            hp_rectangle,
            scale_rectangle(hp_rectangle, 120.0/100.0),
            SPEED,
            Default1920x1080
        );

        Self{
            background_animated_rectangle,
            hp_rectangle_animated,
            attack_rectangle_animated,
            textures,
            position: position.clone(),
            rotation,
            life_bar_width_percentage: 1.0,
            total_hp,
            current_hp,
            current_attack,
            current_animation: None,
            animation_finished: true,
        }
    }

    pub fn background_rectangle(&self) -> Rect {
        self.background_animated_rectangle.rectangle().base_rect
    }

    pub fn set_animation(&mut self, anim: Option<TeamLayoutAnimation>) {
        self.animation_finished = anim.is_none();
        self.current_animation = anim;

    }

    pub fn animation_finished(&self)-> bool {
        self.animation_finished
    }

    pub fn update(&mut self, current_hp: u128) {
        if let Some(anim) = &self.current_animation {
            match anim {
                TeamLayoutAnimation::Active => {
                    self.background_animated_rectangle.animate();
                    self.hp_rectangle_animated.animate();
                    self.attack_rectangle_animated.animate();
                    if    !self.background_animated_rectangle.is_moving() &&
                        !self.hp_rectangle_animated.is_moving() &&
                        !self.attack_rectangle_animated.is_moving() {
                        self.animation_finished = true
                    }
                },
                TeamLayoutAnimation::Damage => {
                    self.current_hp = current_hp;
                    let current =self.current_hp as f64;
                    let max = self.total_hp as f64;
                    let expected_percentage = current / max;
                    if self.life_bar_width_percentage > expected_percentage {
                        self.life_bar_width_percentage =
                            self.life_bar_width_percentage - 0.01 * REMOVE_LIFE_SPEED;
                    }
                    if self.life_bar_width_percentage != expected_percentage {
                        self.animation_finished = true;
                    }
                }
            }
        } else {
            self.background_animated_rectangle.reset();
            self.hp_rectangle_animated.reset();
            self.attack_rectangle_animated.reset();
        }
    }

    pub fn draw(&self) {
        self.draw_team_background();
        self.draw_hp();
        self.draw_attack();
    }

    fn draw_team_background(&self) {
        let rect_to_draw = self.background_animated_rectangle.rectangle().to_screen_rect(
            screen_width(), screen_height()
        );
        draw_texture_ex(
            &self.textures.stats_background_texture,
            rect_to_draw.x,
            rect_to_draw.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(rect_to_draw.w, rect_to_draw.h)),
                rotation: self.rotation,
                ..Default::default()
            },
        );

        draw_texture_ex(
            &self.textures.stats_border,
            rect_to_draw.x,
            rect_to_draw.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(rect_to_draw.w, rect_to_draw.h)),
                rotation: self.rotation,
                ..Default::default()
            },
        );
    }

    fn draw_hp(&self) {
        let rect_to_draw = self.hp_rectangle_animated.rectangle().to_screen_rect(
            screen_width(), screen_height()
        );
        draw_texture_in_animated_rectangle(
            &self.textures.life_bar_container,
            &self.hp_rectangle_animated
        );

        draw_texture_ex(
            &self.textures.life_bar,
            rect_to_draw.x,
            rect_to_draw.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    0.0,
                    0.0,
                    self.textures.life_bar.width(),
                    self.textures.life_bar.height())
                ),
                dest_size: Some(Vec2::new(
                    rect_to_draw.w * self.life_bar_width_percentage as f32,
                    rect_to_draw.h
                )),
                ..Default::default()
            },
        );
        let font_size = rect_to_draw.h * 6.0/10.0;
        draw_text(
            format!("HP: {}", self.current_hp.to_string()).as_str(),
            rect_to_draw.x + rect_to_draw.w * 30.0/100.0,
            rect_to_draw.y + rect_to_draw.h * 7.0/10.0,
            font_size,
            WHITE
        );
    }

    fn draw_attack(&self) {
        draw_texture_in_animated_rectangle(
            &self.textures.stats_label_background,
            &self.attack_rectangle_animated
        );
        let rect_to_draw = self.attack_rectangle_animated.rectangle().to_screen_rect(
            screen_width(), screen_height()
        );

        let font_size = rect_to_draw.h * 9.0/10.0;
        draw_text(
            format!("Attack: {}",&self.current_attack.to_string()).as_str(),
            rect_to_draw.x + rect_to_draw.w * 5.0/100.0,
            rect_to_draw.y + rect_to_draw.h * 7.0/10.0,
            font_size,
            WHITE
        );
    }
}

fn calculate_stats_rectangle(position: &BattlePosition) -> Rect {
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

fn calculate_hp_rectangle(stats_rectangle: Rect, position: &BattlePosition) -> Rect {
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

fn calculate_attack_rectangle(stats_rectangle: Rect, position: &BattlePosition) -> Rect {
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

    println!("Attack rectangle is ({},{})-{}-{}", top_left_corner_x, top_left_corner_y, size_x, size_y);

    Rect::new(top_left_corner_x, top_left_corner_y, size_x, size_y)
}

fn attack_rectangle_hardcoded_default(shiai_position: &BattlePosition) -> Rect {
    let top_left_corner_y = match shiai_position {
        AttackParty(_) => 995.5,
        DefenseParty(_) => 60.2,
    };

    let top_left_corner_x = match shiai_position {
        AttackParty(team_position) => {
            match team_position {
                CaptainTeam => 162.8,
                SecondTeam => 747.13,
                ThirdTeam => 1331.47
            }
        },
        DefenseParty(team_position) => {
            match team_position {
                CaptainTeam => 162.8,
                SecondTeam => 747.13,
                ThirdTeam => 1331.47
            }
        }
    };

    Rect::new(top_left_corner_x, top_left_corner_y, 425.74, 32.4)
}