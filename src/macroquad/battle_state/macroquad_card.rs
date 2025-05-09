use macroquad::math::Rect;
use macroquad::prelude::{screen_height, screen_width, Texture2D};
use crate::domain::{AttackParty, CaptainTeam, DefenseParty, SecondTeam, ShiaiPosition, ThirdTeam};
use crate::macroquad::{draw_texture_in_rectangle, scale_rectangle};


pub enum CardPosition{
    Captain,
    Second,
    Third,
    Fourth
}
pub struct MacroquadCard {
    team_position: ShiaiPosition,
    card_position: CardPosition,
    background_rectangle: Rect,
    template_rectangle: Rect,
    background_texture: Texture2D,
    template_texture: Texture2D,
}
impl MacroquadCard {
    pub fn new(
        team_position: ShiaiPosition,
        card_position: CardPosition,
        background_texture: Texture2D,
        template_texture: Texture2D,
    ) -> Self{
        let (background_rectangle, template_rectangle) = calculate_card_rectangles(
            &team_position, &card_position
        );
        Self{
            team_position,
            card_position,
            background_texture,
            template_texture,
            background_rectangle,
            template_rectangle,
        }
    }
}

impl MacroquadCard {
    pub fn update(&mut self) {
       self.resize();
    }

    fn resize(&mut self) {
        let (background_rectangle, template_rectangle) = calculate_card_rectangles(
            &self.team_position, &self.card_position
        );
        self.background_rectangle = background_rectangle;
        self.template_rectangle = template_rectangle;
    }

    pub fn draw(&self) {
        draw_texture_in_rectangle(&self.background_texture, self.background_rectangle);
        draw_texture_in_rectangle(&self.template_texture, self.template_rectangle);
    }
}

fn calculate_card_rectangles(position: &ShiaiPosition, card_position: &CardPosition) -> (Rect, Rect) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    let w_parts = 46.0;

    let size = (screen_w/ w_parts * 3.0, screen_h/ 7.0);

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

    let party_y_offset = match position {
        AttackParty(_) => screen_h - (size.1 * 2.0),
        DefenseParty(_) => size.1,
    };

    let background_rectangle = Rect::new(team_x_offset + card_x_offset, party_y_offset, size.0, size.1);

    (background_rectangle.clone(), scale_rectangle(background_rectangle, 95.0/100.0))
}