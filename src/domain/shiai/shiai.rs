use crate::domain::*;
use crate::domain::shiai::print_shiai::print_shiai;
use super::*;

#[derive(Debug)]
pub enum ShiaiError{
    SubjectMissingError,
    TargetMissingError
}


pub struct Shiai{
    pub current_state: ShiaiState,
    pub result: Vec<ShiaiTurn>,
}

impl Shiai{
    pub fn new(attacker: Party, defender: Party) -> Self {
        Self{
            current_state: ShiaiState::new(attacker, defender),
            result: vec![]
        }
    }

    pub fn battle(mut self) -> Self{
        let mut turn = 1;
        while turn < 10 {
            let active_team = select_turn_team(turn);
            self = self.play_turn(active_team);
            turn = turn + 1;
        }
        print_shiai(&self);
        self
    }

    fn play_turn(self, subject: ShiaiPosition) -> Self {
        match self.current_state.state.get(&subject) {
            Some(team) if team.is_alive() => team,
            _ => return self,
        };

        let mut actions = vec![];

        // combo skills
        // active skills

        //attack
        let attack_action = match attack_action(&self.current_state, subject) {
            Ok(action) => action,
            Err(_) => return self,
        };
        actions.push(attack_action.clone());

        // finally apply the stuff, by creating the turn.
        match ShiaiTurn::new(actions, self.current_state.clone()){
            Ok(turn) =>{
                let mut turns = self.result.clone();
                turns.push(turn.clone());
                Self{
                    current_state: turn.state_result.clone(),
                    result: turns,
                }
            },
            Err(error) => panic!("Error while applying shiai: {:?}", error),
        }
    }
}

fn select_turn_team(turn: u8) -> ShiaiPosition {
    match turn % 6 {
        1 => AttackParty(CaptainTeam),
        2 => DefenseParty(CaptainTeam),
        3 => AttackParty(SecondTeam),
        4 => DefenseParty(SecondTeam),
        5 => AttackParty(ThirdTeam),
        0 => DefenseParty(ThirdTeam),
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{stub_party};
    use super::*;

    #[test]
    fn test_select_active_team() {
        let attacker = stub_party();
        let defender = stub_party();
        let selected_team = select_turn_team(35);

        assert_eq!(AttackParty(ThirdTeam), selected_team);
    }
}