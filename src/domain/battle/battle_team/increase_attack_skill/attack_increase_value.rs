use crate::domain::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AttackIncreaseValue(u128);


impl AttackIncreaseValue {
    // TODO test
    pub fn new(formula: &SkillEffectValueFormula, card: &Card) -> Self {
        let base = match formula {
            BasedOnCardAttack(p) => {
                card.attack().value() * p.value() / 100
            }
            BasedOnCardHealthPoints(p) => {
                card.health_points().value() * p.value() / 100
            }
            EffectBasedOnCardLevel(p) => {
                card.level() as u128 * p.value() / 100
            }
        };

        Self(base)
    }
    pub fn value(&self) -> u128 {
        self.0
    }
}