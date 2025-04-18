use crate::domain::*;

pub fn factory(
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
