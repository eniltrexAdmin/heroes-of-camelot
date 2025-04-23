use crate::domain::*;
use super::*;

pub struct Shiai {
    attack_party: BattleParty,
    defense_party: BattleParty,
}



pub enum ShiaiEvent{
    Attack(TeamAttackedDomainEvent),
    DamageReceived(DamageReceived)
}

pub struct ShiaiResult {
    winner: Party,
    shiai_events: Vec<ShiaiEvent>
}

pub fn battle(attacker: Party, defender: Party) -> ShiaiResult {
    let mut events = Vec::new();
    let mut turn = 1;


    let mut shiai = Shiai::new(attacker.clone(), defender.clone());

    println!("{}", shiai.pretty_print());

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
            attack_party: BattleParty::new(attacker, &PartyPosition::Attack),
            defense_party: BattleParty::new(defender, &PartyPosition::Defense),
        }
    }

    fn play_turn(&mut self, selected_team: ShiaiPosition) -> Vec<ShiaiEvent>{
        // skills part
        self.attack(&selected_team);
        vec![]
    }

    fn attack(&mut self, attacker: &ShiaiPosition) {
        // time to make copies, and save the results!
        // let mut attacker_team = self.select_team(&attacker);
        //
        // attacker_team.attack_party(&mut self.defense_party);
        //
        // self.set_team(attacker, attacker_team);
        // self.set_team(attacker, defender_team);
    }

    // fn select_team_ref(&mut self, selector: &ShiaiPosition) -> &mut BattleTeam {
    //     match  selector {
    //         ShiaiPosition::AttackParty(team_select) => self.attack_party.get_team_mut_ref(team_select),
    //         ShiaiPosition::DefenseParty(team_select) => self.defense_party.get_team_mut_ref(team_select)
    //     }
    // }

    fn select_team(&self, selector: &ShiaiPosition) -> BattleTeam {
        match  selector {
            ShiaiPosition::AttackParty(team_select) => self.attack_party.get_team_clone(team_select),
            ShiaiPosition::DefenseParty(team_select) => self.defense_party.get_team_clone(team_select)
        }
    }

    fn set_team(&mut self, select: &ShiaiPosition, team: BattleTeam) {
        match select {
            ShiaiPosition::AttackParty(team_select) => self.attack_party.set_team(team_select, team),
            ShiaiPosition::DefenseParty(team_select) => self.defense_party.set_team(team_select, team),
        }
    }

    pub fn pretty_print(&self) -> String {
        format!(
            "Defender Party:\n{}\nAttacker Party:\n{}\n",
             self.defense_party.format_pretty(true),
            self.attack_party.format_pretty(false),

        )
    }
}



fn select_active_team(attacker: &Party, defender: &Party, turn: u8) -> ShiaiPosition {
    match turn %2 {
        1 => ShiaiPosition::AttackParty(select_team(attacker, turn)),
        0 => ShiaiPosition::DefenseParty(select_team(defender, turn)),
        _ => unreachable!()
    }
}

fn select_team(party: &Party, turn: u8) -> TeamPosition {
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

        assert_eq!(ShiaiPosition::DefenseParty(CaptainTeam), selected_team);

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