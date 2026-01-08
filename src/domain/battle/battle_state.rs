use std::collections::HashMap;
use crate::domain::*;
use super::*;


#[derive(Clone, Debug)]
pub enum BattleEvent {
    TeamAttacked(TeamAttackedDomainEvent),
    IncreasedThisTurnAttack(IncreasedThisTurnAttackDomainEvent),
}


#[derive(Clone, Debug)]
pub struct BattleState {
    pub state: HashMap<BattlePosition, BattleTeam>,
}

impl BattleState {
    pub fn new(attacker: Party, defender: Party) -> Self {
        let mut teams = HashMap::new();
        teams.insert(AttackParty(CaptainTeam), BattleTeam::new(
            attacker.captain_team().clone(), AttackParty(CaptainTeam),
        ));
        teams.insert(DefenseParty(CaptainTeam), BattleTeam::new(
            defender.captain_team().clone(), DefenseParty(CaptainTeam),
        ));

        if let Some(team) = attacker.second_team() {
            teams.insert(AttackParty(SecondTeam), BattleTeam::new(
                team.clone(), AttackParty(SecondTeam),
            ));
        }
        if let Some(team) = attacker.third_team() {
            teams.insert(AttackParty(ThirdTeam), BattleTeam::new(
                team.clone(), AttackParty(ThirdTeam),
            ));
        }

        if let Some(team) = defender.second_team() {
            teams.insert(DefenseParty(SecondTeam), BattleTeam::new(
                team.clone(), DefenseParty(SecondTeam),
            ));
        }
        if let Some(team) = defender.third_team() {
            teams.insert(DefenseParty(ThirdTeam), BattleTeam::new(
                team.clone(), DefenseParty(ThirdTeam),
            ));
        }
        Self { state: teams }
    }

    pub fn get(&self, position: &BattlePosition) -> Option<&BattleTeam>{
        self.state.get(&position)
    }

    pub fn expect_team(&self, position: &BattlePosition) -> &BattleTeam {
        self.state.get(&position).expect("if you use this function you know team exists!")
    }


    pub fn apply_domain_events(&mut self, events: Vec<BattleEvent>){
        for event in events {
            match event {
                BattleEvent::TeamAttacked(domain_event) => {
                    self.apply_team_attacked_domain_event(domain_event)
                }
            }
        }
    }

    fn apply_team_attacked_domain_event(&mut self, event: TeamAttackedDomainEvent)
    {
        if let Some(target_team) = self.state.get_mut(&event.target) {
            target_team.apply_team_attacked_domain_event(event);
        }
    }


}


#[cfg(test)]
mod tests {
    use crate::data::{stub_party, stub_party_2};
    use crate::domain::*;
    use super::*;

    #[test]
    fn test_start_state() {
        let attacker = stub_party();
        let defender = stub_party_2();

        let state = BattleState::new(attacker.clone(), defender.clone());
        assert_eq!(
            &AttackParty(CaptainTeam),
            state.state.get(&AttackParty(CaptainTeam)).unwrap().position()
        );
        assert_eq!(
            attacker.captain_team(),
            state.state.get(&AttackParty(CaptainTeam)).unwrap().original_team()
        );
        assert_eq!(
            &AttackParty(SecondTeam),
            state.state.get(&AttackParty(SecondTeam)).unwrap().position()
        );
        assert_eq!(
            attacker.second_team().unwrap(),
            state.state.get(&AttackParty(SecondTeam)).unwrap().original_team()
        );
        assert_eq!(
            &AttackParty(ThirdTeam),
            state.state.get(&AttackParty(ThirdTeam)).unwrap().position()
        );
        assert_eq!(
            attacker.third_team().unwrap(),
            state.state.get(&AttackParty(ThirdTeam)).unwrap().original_team()
        );

        assert_eq!(
            &DefenseParty(CaptainTeam),
            state.state.get(&DefenseParty(CaptainTeam)).unwrap().position()
        );
        assert_eq!(
            defender.captain_team(),
            state.state.get(&DefenseParty(CaptainTeam)).unwrap().original_team()
        );
        assert_eq!(
            &DefenseParty(SecondTeam),
            state.state.get(&DefenseParty(SecondTeam)).unwrap().position()
        );
        assert_eq!(
            defender.second_team().unwrap(),
            state.state.get(&DefenseParty(SecondTeam)).unwrap().original_team()
        );
        assert_eq!(
            &DefenseParty(ThirdTeam),
            state.state.get(&DefenseParty(ThirdTeam)).unwrap().position()
        );
        assert_eq!(
            defender.third_team().unwrap(),
            state.state.get(&DefenseParty(ThirdTeam)).unwrap().original_team()
        );
    }
}