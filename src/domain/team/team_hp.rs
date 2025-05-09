use crate::domain::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TeamHealthPoints(u128);

impl TeamHealthPoints {
    pub fn new(cards: &Vec<Card>, combo_skills: &Vec<ComboSkill>) -> Self {
        let base_hp: u128 = base_health_points(&cards);
        let skill_bonus: u32 = get_combo_skill_hp_bonus(combo_skills);
        let multiplier = 100 + (skill_bonus as u128);
        Self(base_hp * multiplier / 100)
    }
    pub fn value(&self) -> u128 {
        self.0
    }
}

fn base_health_points(cards: &Vec<Card>) -> u128 {
    cards.iter().map(|card| card.health_points().value() as u128).sum()
}

fn get_combo_skill_hp_bonus(skills: &Vec<ComboSkill>) -> u32 {
    skills
        .iter()
        .filter_map(|skill| match skill.effect() {
            ComboSkillEffect::Passive(HealthPointsIncrease(percent)) => Some(*percent),
            _ => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::apprentice_template;
    use crate::domain::{HealthPointsIncrease, Id};
    use std::sync::Arc;

    #[test]
    fn test_base_attack() {
        let mut cards = vec![];
        let apprentice_template = apprentice_template();
        let apprentice_card = Card::new(Id::new(), Arc::new(apprentice_template));
        cards.push(apprentice_card.clone());
        cards.push(apprentice_card.clone());
        cards.push(apprentice_card.clone());
        cards.push(apprentice_card.clone());

        let team_attack = base_health_points(&cards);
        assert_eq!(1200 * 4, team_attack);
    }
    #[test]
    fn test_get_combo_skill_hp_bonus() {
        let combo_skill = ComboSkill::new(
            SkillName::new("extra hp".to_string()),
            SkillDescription::new("Increments hp 25%".to_string()),
            ComboSkillEffect::Passive(HealthPointsIncrease(25)),
            vec![],
        );

        let combo_skills = vec![
            combo_skill.clone(),
            combo_skill.clone(),
            combo_skill.clone(),
        ];

        assert_eq!(75u32, get_combo_skill_hp_bonus(&combo_skills))
    }

    #[test]
    fn team_hp_creation() {
        let mut cards = vec![];
        let apprentice_template = apprentice_template();
        let combo_card = Card::new(Id::new(), Arc::new(apprentice_template));
        cards.push(combo_card.clone());
        cards.push(combo_card.clone());
        cards.push(combo_card.clone());
        cards.push(combo_card.clone());

        let combo_skill = ComboSkill::new(
            SkillName::new("extra hp".to_string()),
            SkillDescription::new("Increments hp 25%".to_string()),
            ComboSkillEffect::Passive(HealthPointsIncrease(25)),
            vec![],
        );

        let combo_skills = vec![
            combo_skill.clone(),
            combo_skill.clone(),
            combo_skill.clone(),
        ];

        let team_attack = TeamHealthPoints::new(&cards, &combo_skills);
        assert_eq!(8400, team_attack.value());
    }
}
