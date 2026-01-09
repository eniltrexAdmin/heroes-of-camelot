use super::*;


impl BattleTeam {
    pub fn perform_skill(&self, state: &BattleState) -> Vec<BattleEvent>
    {
        let chosen_skill = self.choose_active_skill();

        if let Some(card_performing) = chosen_skill {
            let event = self.execute_card_skill(state, card_performing);
            return vec![event];
        }
        vec![]
    }

    fn execute_card_skill(&self, state: &BattleState, card_performing: &Card) -> BattleEvent {
        match card_performing.active_skill().effect() {
            SkillEffect::IncreaseThisTurnAttack(formula) => {
                self.increase_turn_attack(card_performing, state)
            }
            SkillEffect::DecreaseThisTurnAttack(_) => {}
            SkillEffect::MagicDamage(_) => {}
            SkillEffect::PhysicalDamage(_) => {}
            SkillEffect::Heal(_) => {}
        };

    }
}


