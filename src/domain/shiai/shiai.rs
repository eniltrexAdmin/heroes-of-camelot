use crate::domain::*;
use super::*;

pub struct Shiai {
    attack_party: BattleParty,
    defense_party: BattleParty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShiaiSelect {
    AttackParty(TeamSelect),
    DefenseParty(TeamSelect)
}

pub enum ShiaiEvent{
    Attack(TeamAttackedDomainEvent)
}

pub struct TeamAttackedDomainEvent{
    attacker: ShiaiSelect,
    defender: ShiaiSelect,
    attack_damage: TeamAttack,
    attack_party: BattleParty,
    defense_party: BattleParty,
}

pub struct ShiaiResult {
    winner: Party,
    shiai_events: Vec<ShiaiEvent>
}

pub fn battle(attacker: Party, defender: Party) -> ShiaiResult {
    let mut events = Vec::new();
    let mut turn = 1;


    let mut shiai = Shiai::new(attacker.clone(), defender.clone());

    while turn < 30 {
        let active_team = select_active_team(&attacker, &defender, turn);
        // let turn_events = play_turn(&mut shiai, active_team);
        let turn_events = shiai.play_turn(active_team);
        events.extend(turn_events);
        turn = turn + 1;
    }
    ShiaiResult{
        shiai_events: events,
        winner: attacker,
    }
}

impl Shiai {
    fn new(attacker: Party, defender: Party) -> Shiai {
        Self{
            attack_party: BattleParty::new(attacker),
            defense_party: BattleParty::new(defender),
        }
    }

    fn play_turn(&mut self, active_team: ShiaiSelect) -> Vec<ShiaiEvent>{
        // skills part
        //...
        // self.execute_attack(active_team);
        self.attack(&active_team);
        vec![]
    }

    fn attack(&mut self, attacker: &ShiaiSelect) {
        // time to make copies, and save the results!
        let mut attacker_team = self.select_team(&attacker);
        let mut defender_team = self.select_team(&attacker);

        attacker_team.attack(&mut defender_team);

        self.set_team(attacker, attacker_team);
        self.set_team(attacker, defender_team);
    }

    fn select_team(&self, selector: &ShiaiSelect) -> BattleTeam {
        match  selector {
            ShiaiSelect::AttackParty(team_select) => self.attack_party.get_team_clone(team_select),
            ShiaiSelect::DefenseParty(team_select) => self.defense_party.get_team_clone(team_select)
        }
    }

    fn set_team(&mut self, select: &ShiaiSelect, team: BattleTeam) {
        match select {
            ShiaiSelect::AttackParty(team_select) => self.attack_party.set_team(team_select, team),
            ShiaiSelect::DefenseParty(team_select) => self.defense_party.set_team(team_select, team),
        }
    }
}



fn select_active_team(attacker: &Party, defender: &Party, turn: u8) -> ShiaiSelect {
    match turn %2 {
        1 => ShiaiSelect::AttackParty(select_team(attacker, turn)),
        0 => ShiaiSelect::DefenseParty(select_team(defender, turn)),
        _ => unreachable!()
    }
}

fn select_team(party: &Party, turn: u8) -> TeamSelect {
    let mut available_teams = vec![CaptainTeam];
    if party.second_team().is_some() {
        available_teams.push(SecondTeam);
    }
    if party.third_team().is_some() {
        available_teams.push(ThirdTeam);
    }

    let index = (turn as usize) % available_teams.len();
    available_teams[index].clone()
}

#[cfg(test)]
mod tests {
    use crate::data::stub_party;
    use super::*;

    #[test]
    fn test_select_active_team() {
        let attacker = stub_party();
        let defender = stub_party();
        let selected_team = select_active_team(&attacker, &defender, 35);

        assert_eq!(ShiaiSelect::DefenseParty(TeamSelect::CaptainTeam), selected_team);

    }
    // #[test]
    // fn play_battle() {
    //     let attacker = stub_party();
    //     let defender = stub_party();
    //     let engine = ShiaiEngine::new(attacker, defender);
    //
    //     let result = engine.battle();
    //
    // }
}