use macroquad::prelude::{Texture2D};
use crate::domain::{ShiaiPosition};
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
    animation_list: CardAnimationsList,
    current_animation: CardAnimation,
}
impl MacroquadCard {
    pub fn new(
        team_position: ShiaiPosition,
        card_position: CardPosition,
        background_texture: Texture2D,
        template_texture: Texture2D,
    ) -> Self{
        let animation_list = CardAnimationsList::new(
            team_position.clone(),
            card_position.clone(),
            background_texture,
            template_texture,
            SPEED
        );
        Self{
            team_position,
            card_position,
            current_animation: animation_list.passive.clone(),
            animation_list
        }
    }
}

impl MacroquadCard {

    pub fn is_animation_finished(&self) -> bool {
        self.current_animation.is_finished()
    }
    pub fn set_animation(&mut self, anim: CardAnimationKind) {
        match anim {
            CardAnimationKind::StartTurn => {
                self.current_animation = self.animation_list.start_turn.clone();
            },
            CardAnimationKind::Attack => {
                self.current_animation = self.animation_list.attack.clone();
            },
            CardAnimationKind::AttackReturn => {
                self.current_animation = self.animation_list.attack_return.clone();
            },
            CardAnimationKind::EndTurn => {
                self.current_animation = self.animation_list.end_turn.clone();
            },
            CardAnimationKind::Passive => {
                self.current_animation = self.animation_list.passive.clone();
            }
        }
    }
    pub fn update(&mut self) {
        self.current_animation.update();
    }

    pub fn draw(&self) {
       self.current_animation.draw();
    }
}
