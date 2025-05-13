use std::collections::HashMap;
use crate::domain::*;


#[derive(Clone, Debug)]
pub enum ShiaiEvent {
    Attack(TeamAttackedDomainEvent),
}


#[derive(Clone)]
pub struct ShiaiState{
    pub state: HashMap<ShiaiPosition, BattleTeam>,
}

impl ShiaiState{
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

    pub fn get(&self, position: &ShiaiPosition) -> Option<&BattleTeam>{
        self.state.get(&position)
    }

    pub fn expect_team(&self, position: &ShiaiPosition) -> &BattleTeam {
        self.state.get(&position).expect("if you use this function you know team exists!")
    }


    pub fn apply_domain_events(self, events: Vec<ShiaiEvent>) -> Self {
        events.into_iter().fold(self, |applied_shiai, event| {
            match event {
                ShiaiEvent::Attack(domain_event) => {
                    applied_shiai.apply_team_attacked_domain_event(domain_event)
                }
            }
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::data::{stub_party, stub_party_2};
    use crate::domain::{AttackParty, CaptainTeam, DefenseParty, SecondTeam, ThirdTeam};
    use super::*;

    #[test]
    fn test_start_state() {
        let attacker = stub_party();
        let defender = stub_party_2();

        let state = ShiaiState::new(attacker.clone(), defender.clone());
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