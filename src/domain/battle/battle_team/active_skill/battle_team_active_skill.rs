use super::*;

impl BattleTeam {
    pub fn perform_skill(&self, state: &BattleState) -> Vec<BattleEvent>
    {
        let chosen_card = self.choose_active_skill();

        if let Some(card_performing) = chosen_card {
            let event = card_performing.execute_skill(state, self.position.clone());
            return vec![BattleEvent::ActiveSkillExecuted(event)];
        }
        vec![]
    }

    pub fn apply_active_skill_executed_domain_event(&mut self, event: ActiveSkillExecutedDomainEvent) {
        match event.effect {
            ActiveSkillExecutedEffect::IncreasedThisTurnAttack(domain_event) => {
                self.apply_attack_increased(domain_event);
            }
        }
    }
}


impl Card {
    pub fn execute_skill(&self, state: &BattleState, battle_team: BattlePosition) -> ActiveSkillExecutedDomainEvent{
        let base = self.skill_effect_base_value();
        let effect = match self.active_skill().effect() {
            SkillEffect::IncreaseThisTurnAttack(_)=> {
                let event = increase_turn_attack(state, battle_team.clone(), base);
                ActiveSkillExecutedEffect::IncreasedThisTurnAttack(event)
            },
            SkillEffect::DecreaseThisTurnAttack(_)=> {
                let event = increase_turn_attack(state, battle_team.clone(), base);
                ActiveSkillExecutedEffect::IncreasedThisTurnAttack(event)
            },
            SkillEffect::MagicDamage(_)=> {
                let event = increase_turn_attack(state, battle_team.clone(), base);
                ActiveSkillExecutedEffect::IncreasedThisTurnAttack(event)
            },
            SkillEffect::PhysicalDamage(_)=> {
                let event = increase_turn_attack(state, battle_team.clone(), base);
                ActiveSkillExecutedEffect::IncreasedThisTurnAttack(event)
            },
            SkillEffect::Heal(_)=> {
                let event = increase_turn_attack(state, battle_team.clone(), base);
                ActiveSkillExecutedEffect::IncreasedThisTurnAttack(event)
            },
        };

        ActiveSkillExecutedDomainEvent{
            card: self.clone(),
            battle_team: battle_team.clone(),
            effect,
        }
    }
}

// TODO test module, even though this is mainly orchestrator.