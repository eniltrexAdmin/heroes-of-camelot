use macroquad::math::Rect;
use macroquad::prelude::Texture2D;
use crate::domain::{ShiaiPosition};
use super::*;


#[derive(Clone)]
pub struct CardTextures {
    pub background: Texture2D,
    pub captain_template_texture: Texture2D,
    pub second_card_texture: Option<Texture2D>,
    pub third_card_texture: Option<Texture2D>,
    pub fourth_card_texture: Option<Texture2D>,
}

pub struct CardTeam{
    cards: Vec<MacroquadCard>,
}
impl CardTeam {
    pub fn new(card_textures: CardTextures, shiai_position: &ShiaiPosition)-> Self {
        let mut cards = Vec::new();
        cards.push(MacroquadCard::new(
            shiai_position.clone(),
            CardPosition::Captain,
            card_textures.background.clone(),
            card_textures.captain_template_texture,
        ));
        if let Some(second_card_texture) = card_textures.second_card_texture {
            cards.push(MacroquadCard::new(
                shiai_position.clone(),
                CardPosition::Second,
                card_textures.background.clone(),
                second_card_texture,
            ));
        }
        if let Some(third_card_texture) = card_textures.third_card_texture {
            cards.push(MacroquadCard::new(
                shiai_position.clone(),
                CardPosition::Third,
                card_textures.background.clone(),
                third_card_texture,
            ));
        }
        if let Some(fourth_card_texture) = card_textures.fourth_card_texture {
            cards.push(MacroquadCard::new(
                shiai_position.clone(),
                CardPosition::Fourth,
                card_textures.background,
                fourth_card_texture,
            ));
        }

        Self{cards}
    }

    pub fn set_animation(&mut self, anim: CardAnimationKind) {
        self.cards.iter_mut().for_each(|card| {
            card.set_animation(anim.clone());
        })
    }

    pub fn update(&mut self) {
       self.cards.iter_mut().for_each(|card| {
           card.update();
       })
    }

    pub fn animation_finished(&self) -> bool{
        self.cards.iter().all(|card| {
            card.animation_finished()
        })
    }
    pub fn draw(&self) {
        self.cards.iter().for_each(|card| {
            card.draw();
        })
    }
}
