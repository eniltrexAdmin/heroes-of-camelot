use crate::domain::{Card, ComboSkill};
use super::*;

enum Team {
    Default(DefaultTeam),
}

//Yes verbose, but safe! Can't iterate, but it's OK.
pub struct DefaultTeam{
    captain: Card,
    second: Option<Card>,
    third: Option<Card>,
    fourth: Option<Card>,
    attack: TeamAttack,
    combo_skills: Vec<ComboSkill>
}

impl DefaultTeam {
    pub fn new(captain: Card, second: Option<Card>, third: Option<Card>, fourth: Option<Card>) -> Self {
        let mut cards = vec![captain.clone()];
        if let Some(ref c) = second { cards.push(c.clone()); }
        if let Some(ref c) = third { cards.push(c.clone()); }
        if let Some(ref c) = fourth { cards.push(c.clone()); }

        let attack = TeamAttack::new(cards);

        DefaultTeam {
            captain,
            second,
            third,
            fourth,
            attack,
            combo_skills: vec![]
        }
    }
}