use crate::domain::*;
use super::*;


impl BattleTeam {
    pub fn perform_skill(&self, state: &BattleState) -> Vec<BattleEvent>
    {
        let chosen_skill = choose_skill(self.original_team());

        if let Some(card_performing) = chosen_skill {
            let event = match card_performing.active_skill().effect() {
                SkillEffect::IncreaseThisTurnAttack(formula) => {
                    increase_turn_attack(card_performing, self.position().clone(), formula)
                }
                SkillEffect::DecreaseThisTurnAttack(_) => {}
                SkillEffect::MagicDamage(_) => {}
                SkillEffect::PhysicalDamage(_) => {}
                SkillEffect::Heal(_) => {}
            }
            return vec![event];
        }
        vec![]
    }
}


