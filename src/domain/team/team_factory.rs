use crate::domain::*;

pub fn team_factory(
    cards: Vec<Card>,
    skill_finder: ComboSkillFinder,
) -> Result<TeamType, TeamCreationError> {
    if cards.is_empty() {
        return Err(TeamCreationError::NotEnoughCards);
    }
    if cards.len() > 4 {
        return Err(TeamCreationError::TooManyCards);
    }

    let combo_skills = skill_finder(&cards);

    let mut iter = cards.into_iter();
    let captain = iter.next().expect("Expected at least 1 card");
    let second = iter.next();
    let third = iter.next();
    let fourth = iter.next();

    let default_team = Team::new(captain, second, third, fourth, combo_skills)?;

    Ok(DefaultTeam(default_team))
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::data::{apprentice_template, combo_skill_finder};
    use crate::domain::*;

    #[test]
    fn test_team_creation_with_data_combo_skill_repository() {
        let mut cards = vec![];
        let apprentice_template = apprentice_template();
        let apprentice_card = Card::new(Id::new(), Rc::new(apprentice_template));
        cards.push(apprentice_card.clone());
        cards.push(apprentice_card.clone());
        cards.push(apprentice_card.clone());
        cards.push(apprentice_card.clone());

        let team = team_factory(
            cards,
            combo_skill_finder
        );

        assert!(team.is_ok());
        let team = team.unwrap();
        match team {
            DefaultTeam(default_team) => {
                assert_eq!(default_team.captain(), &apprentice_card)
            }
        }
    }
}