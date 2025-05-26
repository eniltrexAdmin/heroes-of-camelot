use macroquad::prelude::{screen_height, screen_width, Texture2D, Rect};
use crate::domain::{AttackParty, CaptainTeam, DefenseParty, SecondTeam, ShiaiPosition, ThirdTeam};
use crate::macroquad::utils::*;
use super::*;

pub const SPEED: f32 = 5.0;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardPosition{
    Captain,
    Second,
    Third,
    Fourth
}
pub struct MacroquadCard {
    team_position: ShiaiPosition,
    card_position: CardPosition,
    default_rectangle: (AnimatedRectangle, AnimatedRectangle),
    attacking_rectangle: (AnimatedRectangle, AnimatedRectangle),
    background_texture: Texture2D,
    template_texture: Texture2D,
    current_animation: Option<CardAnimation>,
    animation_finished: bool,
}
impl MacroquadCard {
    pub fn new(
        team_position: ShiaiPosition,
        card_position: CardPosition,
        background_texture: Texture2D,
        template_texture: Texture2D,
        team_layout_rectangle: Rect
    ) -> Self{
        let (
            background_rectangle,
            template_rectangle,
            active_target_rectangle_bg,
            active_target_rectangle
        ) = calculate_card_rectangles(
            &team_position, &card_position, team_layout_rectangle
        );
        // TODO that should be a single texture...
        let bg_rectangle = AnimatedRectangle::new(
            background_rectangle,
            active_target_rectangle_bg,
            SPEED,
            Default1920x1080
        );

        let main_rectangle = AnimatedRectangle::new(
            template_rectangle,
            active_target_rectangle,
            SPEED,
            Default1920x1080,
        );

        let attack_rectangle_target = calculate_attack_target_position(
            &team_position,
            active_target_rectangle
        );
        let attack_rectangle = AnimatedRectangle::new(
            active_target_rectangle,
            attack_rectangle_target,
            8.00,
            Default1920x1080,
        );
        let attack_rectangle_bg_target = calculate_attack_target_position(
            &team_position,
            active_target_rectangle_bg
        );
        let attack_rectangle_bg = AnimatedRectangle::new(
            active_target_rectangle_bg,
            attack_rectangle_bg_target,
            8.00,
            Default1920x1080,
        );
        Self{
            team_position,
            card_position,
            background_texture,
            template_texture,
            default_rectangle: (main_rectangle, bg_rectangle),
            attacking_rectangle: (attack_rectangle, attack_rectangle_bg),
            current_animation: None,
            animation_finished: true
        }
    }
}

impl MacroquadCard {

    pub fn animation_finished(&self) -> bool {
        self.animation_finished
    }
    pub fn set_animation(&mut self, anim: Option<CardAnimation>) {
        self.animation_finished = anim.is_none();
        self.current_animation = anim;
    }
    pub fn update(&mut self) {
        if let Some(anim) = &self.current_animation {
            match anim {
                CardAnimation::StartTurn => {
                    self.default_rectangle.0.animate();
                    self.default_rectangle.1.animate();
                    if !self.default_rectangle.0.is_moving() {
                        self.animation_finished = true;
                    }
                },
                CardAnimation::Attack => {
                    self.attacking_rectangle.0.animate();
                    self.attacking_rectangle.1.animate();
                    if !self.attacking_rectangle.0.is_moving() {
                        self.animation_finished = true;
                    }
                },
                CardAnimation::AttackReturn => {
                },
                CardAnimation::EndTurn => {

                }
            }
        } else {
            self.default_rectangle.0.reset();
            self.default_rectangle.1.reset();
        }
    }

    pub fn draw(&self) {
        if let Some(step) = &self.current_animation {
            match step {
                CardAnimation::Attack => {
                    draw_texture_in_animated_rectangle(
                        &self.background_texture,
                        &self.attacking_rectangle.1,
                    );
                    draw_texture_in_animated_rectangle(
                        &self.template_texture,
                        &self.attacking_rectangle.0,
                    );
                },
                _ => {
                    draw_texture_in_animated_rectangle(
                        &self.background_texture,
                        &self.default_rectangle.1,
                    );
                    draw_texture_in_animated_rectangle(
                        &self.template_texture,
                        &self.default_rectangle.0,
                    );
                }
            }
        } else {
            draw_texture_in_animated_rectangle(
                &self.background_texture,
                &self.default_rectangle.1,
            );
            draw_texture_in_animated_rectangle(
                &self.template_texture,
                &self.default_rectangle.0,
            );
        }
    }
}

fn calculate_card_rectangles(position: &ShiaiPosition, card_position: &CardPosition, team_layout_rectangle: Rect)
    -> (Rect, Rect, Rect, Rect) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    let w_parts = 46.0;

    let size = (screen_w/ w_parts * 3.0, screen_h/ 5.5);

    let team_x_offset = match position {
        AttackParty(team_position) | DefenseParty(team_position) => {
            match team_position {
                CaptainTeam => screen_w/ w_parts * 3.0,
                SecondTeam => screen_w/ w_parts * 17.0,
                ThirdTeam => screen_w/ w_parts * 31.0,
            }
        }
    };

    let card_x_offset = match card_position {
        CardPosition::Captain => 0.0,
        CardPosition::Second => size.0,
        CardPosition::Third => size.0 * 2.0,
        CardPosition::Fourth => size.0 * 3.0,
    };

    let team_layout_y_padding = team_layout_rectangle.h - team_layout_rectangle.h * 5.0/100.0;

    let party_y_offset = match position {
        AttackParty(_) => screen_h - ( team_layout_y_padding + size.1),
        DefenseParty(_) => team_layout_y_padding,
    };

    let background_rectangle = Rect::new(team_x_offset + card_x_offset, party_y_offset, size.0, size.1);
    let card_rectangle =  scale_rectangle(background_rectangle, 95.0/100.0);
    let active_bg = scale_rectangle(background_rectangle, 150.0/100.0);
    let active = scale_rectangle(active_bg, 95.0/100.0);
    (
        background_rectangle.clone(),
        card_rectangle,
        active_bg,
        active
    )
}

fn calculate_attack_target_position(position: &ShiaiPosition, current_rect: Rect) -> Rect {
    let new_y = match position {
        AttackParty(_) => 500.0,
        DefenseParty(_) => 300.0,
    };

    Rect::new(current_rect.x, new_y, current_rect.w, current_rect.h)
}