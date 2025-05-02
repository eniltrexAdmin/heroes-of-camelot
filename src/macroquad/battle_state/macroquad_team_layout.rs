use macroquad::math::Rect;
use macroquad::prelude::{screen_height, screen_width};
use crate::domain::{AttackParty, CaptainTeam, DefenseParty, SecondTeam, ShiaiPosition, ThirdTeam};

pub struct TeamLayout {
    pub background_rectangle: Rect,
    pub hp_rectangle: Rect,
    pub attack_rectangle: Rect,
}

impl TeamLayout {
    pub fn new(position: &ShiaiPosition) -> Self{
        let background_rectangle = calculate_stats_rectangle(position);
        let hp_rectangle = calculate_hp_rectangle(background_rectangle, position);
        let attack_rectangle = calculate_attack_rectangle(background_rectangle, position);
        Self{
            background_rectangle,
            hp_rectangle,
            attack_rectangle
        }
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