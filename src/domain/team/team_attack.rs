use crate::domain::{Card, PassiveSkill, SkillEffect};

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

// fn get_card_attack_bonus(cards: &Vec<Card>) -> u32 {
//     cards
//     .iter()
//     .flat_map(|card| card.skills().iter())
//     .filter_map(|skill| match skill.effect()
//     {
//         SkillEffect::Passive(PassiveSkill::AttackIncrease(percent)) => Some(*percent),
//         _ => None,
//     })
//     .sum()
// }

#[cfg(test)]
mod tests {
    use crate::data::{bard_template, hunter_template};
    use super::*;

    // #[test]
    // fn attack_creation() {
    //     let mut cards = vec![];
    //     cards.push(bard_template());
    //     cards.push(bard_template());
    //     cards.push(hunter_template());
    //     cards.push(hunter_template());
    //
    //     let resources_number = TeamAttack::new(cards);
    //     assert_eq!(600, resources_number.value());
    // }

    #[test]
    fn attack_creation_no_bonus() {
        let mut cards = vec![];
        cards.push(bard_template());
        cards.push(bard_template());
        cards.push(bard_template());
        cards.push(bard_template());

        let resources_number = TeamAttack::new(cards);
        assert_eq!(400, resources_number.value());
    }

    #[test]
    fn get_base_attack() {
        let mut cards = vec![];
        cards.push(bard_template());
        cards.push(bard_template());
        cards.push(hunter_template());
        cards.push(hunter_template());

        let bonus = base_attack(&cards);
        assert_eq!(400, bonus);
    }

    // #[test]
    // fn get_attack_bonus() {
    //     let mut cards = vec![];
    //     cards.push(bard_template());
    //     cards.push(bard_template());
    //     cards.push(hunter_template());
    //     cards.push(hunter_template());
    //
    //     let bonus = get_card_attack_bonus(&cards);
    //     assert_eq!(50, bonus);
    // }
}
