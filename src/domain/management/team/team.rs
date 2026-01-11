use super::*;
use crate::domain::*;

pub enum TeamType {
    DefaultTeam(Team),
}

//Yes verbose, but safe! Can't iterate, but it's OK.
#[derive(Debug, Clone, PartialEq)]
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

    pub fn second(&self) -> Option<&Card> {
        self.second.as_ref()
    }

    pub fn third(&self) -> Option<&Card> {
        self.third.as_ref()
    }

    pub fn fourth(&self) -> Option<&Card> {
        self.fourth.as_ref()
    }

    pub fn card_iterator(&self) -> impl Iterator<Item = &Card> {
        [
            Some(&self.captain), // captain is always present
            self.second.as_ref(),
            self.third.as_ref(),
            self.fourth.as_ref(),
        ]
            .into_iter()
            .flatten()
    }

    pub fn combo_skills(&self) -> &[ComboSkill] {
        &self.combo_skills
    }

    pub fn attack(&self) -> &TeamAttack {
        &self.attack
    }

    pub fn health_points(&self) -> &TeamHealthPoints {
        &self.health_points
    }
}

fn check_combo_skill(cards: &Vec<Card>, combo_skills:  &Vec<ComboSkill>) -> Result<(), TeamCreationError> {
    combo_skills.iter().try_for_each(|combo_skill| {
        check_single_combo_skill(cards, combo_skill)
    })?;

    Ok(())
}

fn check_single_combo_skill(cards: &Vec<Card>, combo_skill: &ComboSkill) -> Result<(), TeamCreationError> {
    let templates_present: Vec<&Name> = cards.iter().map(|card| card.name()).collect();


    let missing: Vec<&Name> = combo_skill
        .required_templates()
        .iter()
        .filter(|required| !templates_present.contains(required))
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(TeamCreationError::ComboNotAllowed {
            combo_skill: combo_skill.name().clone(),
            missing_templates: missing.into_iter().cloned().collect(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum TeamCreationError {
    NotEnoughCards,
    TooManyCards,
    ComboNotAllowed{
        combo_skill: SkillName,
        missing_templates: Vec<Name>,
    },
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::data::apprentice_template;
    use crate::domain::*;

    #[test]
    fn test_sunny_case() {
        let apprentice_template = apprentice_template();
        let apprentice_card = Card::new(Id::new(), Arc::new(apprentice_template));

        let result = Team::new(
            apprentice_card.clone(),
            None,
            None,
            None,
            vec![]
        );
        assert!(result.is_ok());
        let team = result.unwrap();
        assert_eq!(team.captain(), &apprentice_card);
        assert_eq!(apprentice_card.attack().value() as u128, team.attack().value());
        assert_eq!(apprentice_card.health_points().value() as u128, team.health_points().value())
    }

    #[test]
    fn test_wrong_combo_skills() {
        let combo_skill = ComboSkill::new(
            SkillName::new("a skill".to_string()),
            SkillDescription::new("a description".to_string()),
            ComboSkillEffect::Passive(AttackIncrease(30)),
            vec![Name::new("Card that has skill".to_string())]
        );

        let apprentice_template = apprentice_template();
        let apprentice_card = Card::new(Id::new(), Arc::new(apprentice_template));

        let result = Team::new(
            apprentice_card.clone(),
            None,
            None,
            None,
            vec![combo_skill]
        );

        assert!(result.is_err());
    }
}