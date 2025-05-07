use macroquad::color::WHITE;
use macroquad::math::Vec2;
use macroquad::prelude::{draw_texture_ex, screen_height, screen_width, DrawTextureParams, Rect, Texture2D};
use crate::domain::{AttackParty, CaptainTeam, Card, DefenseParty, SecondTeam, ShiaiPosition, ThirdTeam};

pub enum CardTeam{
    Attack(AttackCardTeam),
    Defense(AttackCardTeam)
}
impl CardTeam {
    pub fn new(cards: CardTextures, shiai_position: &ShiaiPosition)-> Self {
        let attack_card_team = AttackCardTeam{
            captain_rectangle: calculate_captain_rectangle(shiai_position),
            textures: cards,
            second_card: None,
            third_card: None,
            fourth_card: None,
        };
        CardTeam::Attack(attack_card_team)
    }
    pub fn update(&mut self) {
        match self {
            CardTeam::Attack(AttackCardTeam) => AttackCardTeam.update(),
            CardTeam::Defense(DefenseCardTeam) => DefenseCardTeam.update(),
        }
    }
    pub fn draw(&self) {
        match self {
            CardTeam::Attack(AttackCardTeam) => AttackCardTeam.draw(),
            CardTeam::Defense(DefenseCardTeam) => DefenseCardTeam.draw(),
        }
    }
}

pub struct AttackCardTeam{
    textures: CardTextures,
    captain_rectangle: Rect,
    second_card: Option<Rect>,
    third_card: Option<Rect>,
    fourth_card: Option<Rect>,
}

#[derive(Clone)]
pub struct CardTextures {
    pub background: Texture2D,
    pub captain_template_texture: Texture2D,
}

impl AttackCardTeam {
    pub fn update(&mut self) {

    }
    pub fn draw(&self) {
        draw_texture_ex(
            &self.textures.background,
            self.captain_rectangle.x,
            self.captain_rectangle.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.captain_rectangle.w, self.captain_rectangle.h)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &self.textures.captain_template_texture,
            self.captain_rectangle.x+5.0,
            self.captain_rectangle.y+5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.captain_rectangle.w-5.0, self.captain_rectangle.h-5.0)),
                ..Default::default()
            },
        );
    }
}

fn calculate_captain_rectangle(position: &ShiaiPosition) -> Rect {
    let screen_w = screen_width();
    let screen_h = screen_height();

    let w_parts = 46.0;

    let size = (screen_w/ w_parts * 3.0, screen_h/ 7.0);

    let top_left_corner = match position {
        AttackParty(team_position) => {
            match team_position {
                CaptainTeam => (screen_w/ w_parts * 3.0, screen_h - (size.1 * 2.0)),
                SecondTeam => (screen_w/ w_parts * 17.0, screen_h - (size.1 * 2.0)),
                ThirdTeam => (screen_w/ w_parts * 31.0, screen_h - (size.1 * 2.0)),
            }
        },
        DefenseParty(team_position) => {
            match team_position {
                CaptainTeam => (screen_w/ w_parts * 3.0, size.1),
                SecondTeam => (screen_w/ w_parts * 17.0, size.1),
                ThirdTeam => (screen_w/ w_parts * 31.0, size.1),
            }
        }
    };
    Rect::new(top_left_corner.0, top_left_corner.1, size.0, size.1)
}