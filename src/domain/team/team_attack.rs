use crate::domain::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TeamAttack(u128);

impl TeamAttack {
    pub fn new(cards: &Vec<Card>, combo_skills: &Vec<ComboSkill>) -> Self {
        let base_attack: u128 = base_attack(&cards);
        let skill_bonus: u32 = get_combo_skill_attack_bonus(combo_skills);
        let multiplier = 100 + (skill_bonus as u128);
        TeamAttack(base_attack * multiplier / 100)
    }
    pub fn value(&self) -> u128 {
        self.0
    }
}

fn base_attack(cards: &Vec<Card>) -> u128 {
    cards.iter().map(|card| card.attack().value() as u128).sum()
}

fn get_combo_skill_attack_bonus(skills: &Vec<ComboSkill>) -> u32 {
    skills
        .iter()
        .filter_map(|skill| match skill.effect() {
            ComboSkillEffect::Passive(AttackIncrease(percent)) => Some(*percent),
            _ => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::apprentice_template;
    use crate::domain::Id;
    use std::rc::Rc;

    #[test]
    fn test_base_attack() {
        let mut cards = vec![];
        let apprentice_template = apprentice_template();
        let apprentice_card = Card::new(Id::new(), Rc::new(apprentice_template));
        cards.push(apprentice_card.clone());
        cards.push(apprentice_card.clone());
        cards.push(apprentice_card.clone());
        cards.push(apprentice_card.clone());

        let team_attack = base_attack(&cards);
        assert_eq!(285 * 4, team_attack);
    }
    #[test]
    fn test_get_combo_skill_attack_bonus() {
        let combo_skill = ComboSkill::new(
            SkillName::new("extra attack".to_string()),
            SkillDescription::new("Increments attack 25%".to_string()),
            ComboSkillEffect::Passive(AttackIncrease(25)),
            vec![],
        );

        let combo_skills = vec![
            combo_skill.clone(),
            combo_skill.clone(),
            combo_skill.clone(),
        ];

        assert_eq!(75u32, get_combo_skill_attack_bonus(&combo_skills))
    }

    #[test]
    fn get_attack_bonus() {
        let mut cards = vec![];
        let apprentice_template = apprentice_template();
        let combo_card = Card::new(Id::new(), Rc::new(apprentice_template));
        cards.push(combo_card.clone());
        cards.push(combo_card.clone());
        cards.push(combo_card.clone());
        cards.push(combo_card.clone());

        let combo_skill = ComboSkill::new(
            SkillName::new("extra attack".to_string()),
            SkillDescription::new("Increments attack 25%".to_string()),
            ComboSkillEffect::Passive(AttackIncrease(25)),
            vec![],
        );

        let combo_skills = vec![
            combo_skill.clone(),
            combo_skill.clone(),
            combo_skill.clone(),
        ];

        let team_attack = TeamAttack::new(&cards, &combo_skills);
        assert_eq!(1995, team_attack.value());
    }
}
