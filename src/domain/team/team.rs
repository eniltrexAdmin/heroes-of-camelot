use super::*;
use crate::domain::{Card, ComboSkill};
use crate::domain::team::team_hp::TeamHealthPoints;

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
    health_points: TeamHealthPoints,
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
        check_combo_skill(&cards, &combo_skills)?;

        let attack = TeamAttack::new(&cards, &combo_skills);
        let health_points = TeamHealthPoints::new(&cards, &combo_skills);

        Ok(Team {
            captain,
            second,
            third,
            fourth,
            attack,
            health_points,
            combo_skills,
        })
    }

    pub fn captain(&self) -> &Card {
        &self.captain
    }
}

fn check_combo_skill(card: &Vec<Card>, combo_skills:  &Vec<ComboSkill>) -> Result<(), TeamCreationError> {
    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum TeamCreationError {
    NotEnoughCards,
    TooManyCards,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sunny_case() {

    }

    #[test]
    fn test_wrong_combo_skills() {

    }
}