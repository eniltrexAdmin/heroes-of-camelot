use crate::domain::Card;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TeamAttack(u128);

impl TeamAttack {
    pub fn new(cards: Vec<Card>) -> Self {
        let total_attack = cards
            .iter()
            .map(|card| card.attack().value() as u128)
            .sum();

        TeamAttack(total_attack)
    }
    pub fn value(&self) -> u128 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::TeamAttack;

    #[test]
    fn attack_creation() {
        let value = 3000;
        let resources_number = TeamAttack::new(value);
        assert_eq!(value, resources_number.value());
    }
}
