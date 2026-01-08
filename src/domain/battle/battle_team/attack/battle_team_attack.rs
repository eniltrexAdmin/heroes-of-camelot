use super::*;

impl BattleTeam {
    fn select_target_party(&self) -> PartyPosition{
        match self.position{
            AttackParty(_) => PartyPosition::Defense,
            DefenseParty(_) => PartyPosition::Attack
        }
    }
    pub fn attack(&self, state: &BattleState) -> Vec<BattleEvent> {
        let target_pos = select_target(&state, self.select_target_party(), TargetStrategy::Default);
        let target = state.expect_team(&target_pos).clone();

        let event = BattleEvent::TeamAttacked(TeamAttackedDomainEvent {
            attacker: self.position().clone(),
            target: target_pos,
            damage_received: target.calculate_attack_damage(self.current_attack().clone()),
        });
        vec![event]
    }

    fn calculate_attack_damage(&self, attack: BattleTeamAttack) -> AttackDamage {
        // shields etc.
        AttackDamage::new_attack_damage(attack.value())
    }

    pub fn apply_team_attacked_domain_event(
        &mut self,
        team_attacked_domain_event: TeamAttackedDomainEvent)
    {
        self.current_hp = match team_attacked_domain_event.damage_received {
            AttackDamage::Physical(physical_damage) => {
                let amount = match physical_damage {
                    PhysicalDamage::RegularDamage(amount) => amount,
                    PhysicalDamage::ReflectedDamage(amount) => amount,
                };
                self.current_hp()
                    .apply_damage(amount)
            },
            AttackDamage::Magical => self.current_hp().clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{shiai_state_stub, stub_team};
    use super::*;

    #[test]
    fn test_attack_action() {
        let mut shiai_state = shiai_state_stub();
        let target_life = shiai_state.expect_team(&DefenseParty(CaptainTeam)).current_hp().value();
        let attacker_attack = shiai_state.expect_team(&AttackParty(CaptainTeam)).current_attack().value();

        let domain_event = shiai_state.expect_team(&AttackParty(CaptainTeam)).attack(&shiai_state);


        assert_eq!(1, domain_event.len());
        shiai_state.apply_domain_events(domain_event);

        assert_eq!(
            target_life - attacker_attack,
            shiai_state.expect_team(&DefenseParty(CaptainTeam)).current_hp().value()
        );
    }

    #[test]
    fn test_calculate_damage() {
        let team = stub_team();
        let battle_team = BattleTeam::new(team.clone(), AttackParty(CaptainTeam));

        let result = battle_team.calculate_attack_damage(battle_team.current_attack);
        assert_eq!(battle_team.current_attack().value(), result.value())
    }

    #[test]
    fn test_apply_team_attacked_domain_event() {
        let mut battle_team = BattleTeam::new( stub_team(), AttackParty(CaptainTeam));
        let original_life = battle_team.current_hp.clone();

        let event = TeamAttackedDomainEvent{
            attacker: AttackParty(CaptainTeam),
            target: DefenseParty(CaptainTeam),
            damage_received: AttackDamage::new_attack_damage(100),
        };

        battle_team.apply_team_attacked_domain_event(event);
        assert_eq!(original_life.value() - 100, battle_team.current_hp().value())
    }

    #[test]
    fn test_select_target_party() {
        let battle_team = BattleTeam::new( stub_team(), AttackParty(CaptainTeam));
        assert_eq!(PartyPosition::Defense, battle_team.select_target_party());

        let battle_team = BattleTeam::new( stub_team(), DefenseParty(CaptainTeam));
        assert_eq!(PartyPosition::Attack, battle_team.select_target_party())
    }
}