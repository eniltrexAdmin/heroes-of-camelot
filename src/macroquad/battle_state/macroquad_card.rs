use macroquad::math::Rect;
use macroquad::prelude::{screen_height, screen_width, Texture2D};
use crate::domain::{AttackParty, CaptainTeam, DefenseParty, SecondTeam, ShiaiPosition, ThirdTeam};
use crate::macroquad::{draw_texture_in_rectangle, modify_rectangle, scale_rectangle};

pub const SPEED: f32 = 1.0;

pub enum CardPosition{
    Captain,
    Second,
    Third,
    Fourth
}
pub struct MacroquadCard {
    team_position: ShiaiPosition,
    card_position: CardPosition,
    rectangle_bg: Rect,
    rectangle: Rect,
    background_texture: Texture2D,
    template_texture: Texture2D,
    active_target_rectangle: Rect,
    active_target_rectangle_bg: Rect,
    current_rectangle: Rect,
    current_rectangle_bg: Rect,
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
        Self{
            team_position,
            card_position,
            background_texture,
            template_texture,
            rectangle_bg: background_rectangle,
            rectangle: template_rectangle,
            current_rectangle: template_rectangle,
            current_rectangle_bg: background_rectangle,
            active_target_rectangle,
            active_target_rectangle_bg
        }
    }
}

impl MacroquadCard {
    pub fn update(&mut self, team_layout_rectangle: Rect, is_active: bool) {
       if is_active {
           self.current_rectangle_bg = modify_rectangle(
               self.current_rectangle_bg,
               self.active_target_rectangle_bg, SPEED
           );
           self.current_rectangle =modify_rectangle(
               self.current_rectangle,
               self.active_target_rectangle, SPEED
           );
       } else {
           self.current_rectangle_bg = self.rectangle_bg;
           self.current_rectangle = self.rectangle;
       }
       self.resize(team_layout_rectangle);
    }

    fn resize(&mut self, team_layout_rectangle: Rect) {
        let (background_rectangle, template_rectangle,
            active_target_rectangle_bg,
            active_target_rectangle) = calculate_card_rectangles(
            &self.team_position, &self.card_position, team_layout_rectangle
        );
        self.rectangle_bg = background_rectangle;
        self.rectangle = template_rectangle;
        self.active_target_rectangle_bg = active_target_rectangle_bg;
        self.active_target_rectangle = active_target_rectangle;
    }

    pub fn draw(&self) {
        draw_texture_in_rectangle(&self.background_texture, self.current_rectangle_bg);
        draw_texture_in_rectangle(&self.template_texture, self.current_rectangle);
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
    (background_rectangle.clone(), card_rectangle, active_bg, active)
}

