use crate::domain::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TeamAttack(u128);

impl TeamAttack {
    pub fn new(cards: Vec<Card>) -> Self {
        let base_attack: u128 = base_attack(&cards);
        // let skill_bonus: u32 = get_card_attack_bonus(&cards);
        // let multiplier = 100 + (skill_bonus as u128);
        // TeamAttack(base_attack * multiplier / 100)
        TeamAttack(base_attack)
    }
    pub fn value(&self) -> u128 {
        self.0
    }
}

fn base_attack(cards: &Vec<Card>) -> u128 {
    cards
        .iter()
        .map(|card| card.attack().value() as u128)
        .sum()
}

fn get_card_attack_bonus(cards: &Vec<Card>) -> u32 {
    cards
    .iter()
    .flat_map(|card| card.combo_skills().iter())
    .filter_map(|skill| match skill.effect()
    {
        ComboSkillEffect::Passive(AttackIncrease(percent)) => Some(*percent),
        _ => None,
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::data::{apprentice_template, card_with_combo};
    use crate::domain::Id;
    use super::*;

    #[test]
    fn attack_creation() {
        let mut cards = vec![];
        let apprentice_template = apprentice_template();
        let apprentice_card = Card::new(Id::new(), Rc::new(apprentice_template));
        cards.push(apprentice_card.clone());
        cards.push(apprentice_card.clone());
        cards.push(apprentice_card.clone());
        cards.push(apprentice_card.clone());

        let resources_number = TeamAttack::new(cards);
        assert_eq!(285 * 4, resources_number.value());
    }

    #[test]
    fn get_attack_bonus() {
        let mut cards = vec![];
        let apprentice_template = card_with_combo();
        let combo_card = Card::new(Id::new(), Rc::new(apprentice_template));
        cards.push(combo_card.clone());
        cards.push(combo_card.clone());
        cards.push(combo_card.clone());
        cards.push(combo_card.clone());

        let bonus = get_card_attack_bonus(&cards);
        assert_eq!(100, bonus);
    }
}
