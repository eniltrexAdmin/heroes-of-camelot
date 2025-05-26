use macroquad::math::Rect;
use macroquad::prelude::{screen_height, screen_width, Texture2D};
use crate::domain::{AttackParty, DefenseParty, ShiaiPosition, CaptainTeam, SecondTeam, ThirdTeam};
use crate::macroquad::battle_state::macroquad_card::CardPosition;
use crate::macroquad::CardAnimationKind::StartTurn;
use crate::macroquad::utils::*;

pub struct CardAnimationsList{
    pub start_turn: CardAnimation,
    pub attack: CardAnimation,
    pub attack_return: CardAnimation,
    pub end_turn: CardAnimation,
    pub passive: CardAnimation,
}

impl CardAnimationsList{
    pub fn new(
        team_position: ShiaiPosition,
        card_position: CardPosition,
        background_texture: Texture2D,
        template_texture: Texture2D,
        speed: f32
    ) -> Self {
        let (
            background_rectangle,
            template_rectangle,
            active_target_rectangle_bg,
            active_target_rectangle
        ) = calculate_card_rectangles(
            &team_position, &card_position, 100.0
        );

        let attack_rectangle_target = calculate_attack_target_position(
            &team_position,
            active_target_rectangle
        );
        let attack_rectangle_bg_target = calculate_attack_target_position(
            &team_position,
            active_target_rectangle_bg
        );

        let start_turn = CardAnimation::new(
            StartTurn,
            background_texture.clone(),
            template_texture.clone(),
            template_rectangle,
            active_target_rectangle,
            background_rectangle,
            active_target_rectangle_bg,
            speed
        );

        let attack: CardAnimation = CardAnimation::new(
            CardAnimationKind::Attack,
            background_texture.clone(),
            template_texture.clone(),
            active_target_rectangle,
            attack_rectangle_target,
            active_target_rectangle_bg,
            attack_rectangle_bg_target,
            speed * 2.0
        );

        let attack_return: CardAnimation = CardAnimation::new(
            CardAnimationKind::AttackReturn,
            background_texture.clone(),
            template_texture.clone(),
            attack_rectangle_target,
            active_target_rectangle,
            attack_rectangle_bg_target,
            active_target_rectangle_bg,
            speed
        );

        let end_turn = CardAnimation::new(
            CardAnimationKind::EndTurn,
            background_texture.clone(),
            template_texture.clone(),
            active_target_rectangle,
            template_rectangle,
            active_target_rectangle_bg,
            background_rectangle,
            speed
        );

        let passive = CardAnimation::new(
            CardAnimationKind::Passive,
            background_texture.clone(),
            template_texture.clone(),
            template_rectangle,
            active_target_rectangle,
            background_rectangle,
            active_target_rectangle_bg,
            speed
        );

        Self{
            start_turn,
            attack,
            attack_return,
            end_turn,
            passive,
        }
    }
}


#[derive(Clone, Debug)]
pub enum CardAnimationKind {
    StartTurn,
    Attack,
    AttackReturn,
    EndTurn,
    Passive
}

#[derive(Clone, Debug)]
pub struct CardAnimation {
    kind: CardAnimationKind,
    main_rectangle: AnimatedRectangle,
    bg_rectangle: AnimatedRectangle,
    background_texture: Texture2D,
    template_texture: Texture2D,
}

impl CardAnimation {

    fn new(
        kind: CardAnimationKind,
        background_texture: Texture2D,
        template_texture: Texture2D,
        main_rectangle_init: Rect,
        main_rectangle_target: Rect,
        bg_rectangle_init: Rect,
        bg_rectangle_target: Rect,
        speed: f32
    ) -> Self {
        let main_rectangle = AnimatedRectangle::new(
            main_rectangle_init,
            main_rectangle_target,
            speed,
            Default1920x1080
        );
        let bg_rectangle = AnimatedRectangle::new(
            bg_rectangle_init,
            bg_rectangle_target,
            speed,
            Default1920x1080
        );
        Self{
            kind,
            main_rectangle,
            bg_rectangle,
            background_texture,
            template_texture,
        }
    }
    pub fn update(&mut self) {
        self.main_rectangle.animate();
        self.bg_rectangle.animate();
    }

    pub fn is_finished(&self) -> bool {
        !self.main_rectangle.is_moving()
    }

    pub fn draw(&self) {
        draw_texture_in_animated_rectangle(&self.background_texture, &self.bg_rectangle);
        draw_texture_in_animated_rectangle(&self.template_texture, &self.main_rectangle);
    }
}


fn calculate_card_rectangles(
    position: &ShiaiPosition,
    card_position: &CardPosition,
    team_layout_rectangle_height: f32
) -> (Rect, Rect, Rect, Rect) {
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

    let team_layout_y_padding = team_layout_rectangle_height - team_layout_rectangle_height * 5.0/100.0;

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