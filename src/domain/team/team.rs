use super::*;
use crate::domain::{Card, ComboSkill};

pub enum TeamType {
    DefaultTeam(Team),
}

//Yes verbose, but safe! Can't iterate, but it's OK.
pub struct Team {
    captain: Card,
    second: Option<Card>,
    third: Option<Card>,
    fourth: Option<Card>,
    attack: TeamAttack,
    combo_skills: Vec<ComboSkill>,
}

impl Team {
    pub fn new(
        captain: Card,
        second: Option<Card>,
        third: Option<Card>,
        fourth: Option<Card>,
        combo_skills: Vec<ComboSkill>,
    ) -> Result<Self, TeamCreationError> {
        let mut cards = vec![captain.clone()];
        if let Some(ref c) = second {
            cards.push(c.clone());
        }
        if let Some(ref c) = third {
            cards.push(c.clone());
        }
        if let Some(ref c) = fourth {
            cards.push(c.clone());
        }

        let attack = TeamAttack::new(&cards, &combo_skills);

        Ok(Team {
            captain,
            second,
            third,
            fourth,
            attack,
            combo_skills,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum TeamCreationError {
    NotEnoughCards,
    TooManyCards
}