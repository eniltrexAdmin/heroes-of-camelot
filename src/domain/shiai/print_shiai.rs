use std::collections::HashMap;
use super::*;

//TODO this will be obviously out of domain!

pub fn print_shiai(shiai: &Shiai) {

}

pub fn print_state(state: &HashMap<ShiaiPosition, BattleTeam>) {
    let field_width = 10; // or whatever number you want
    let attack_field_width = 20;


    println!("{:^width$}/{:^width$} | {:^width$}/{:^width$} | {:^width$}/{:^width$}",
        get_team_current_hp_string(state, &DefenseParty(CaptainTeam)),
        get_team_total_hp_string(state, &DefenseParty(CaptainTeam)),
        get_team_current_hp_string(state, &DefenseParty(SecondTeam)),
        get_team_total_hp_string(state, &DefenseParty(SecondTeam)),
        get_team_current_hp_string(state, &DefenseParty(ThirdTeam)),
        get_team_total_hp_string(state, &DefenseParty(ThirdTeam)),
        width = field_width,
    );

    println!("{:^width$}  | {:^width$}  | {:^width$}",
         get_team_attack_string(state, &DefenseParty(CaptainTeam)),
         get_team_attack_string(state, &DefenseParty(SecondTeam)),
         get_team_attack_string(state, &DefenseParty(ThirdTeam)),
         width = attack_field_width,
    );

    println!();

    println!("{:^width$}  | {:^width$}  | {:^width$}",
         get_team_attack_string(state, &AttackParty(CaptainTeam)),
         get_team_attack_string(state, &AttackParty(SecondTeam)),
         get_team_attack_string(state, &AttackParty(ThirdTeam)),
         width = attack_field_width,
    );

    println!("{:^width$}/{:^width$} | {:^width$}/{:^width$} | {:^width$}/{:^width$}",
         get_team_current_hp_string(state, &AttackParty(CaptainTeam)),
         get_team_total_hp_string(state, &AttackParty(CaptainTeam)),
         get_team_current_hp_string(state, &AttackParty(SecondTeam)),
         get_team_total_hp_string(state, &AttackParty(SecondTeam)),
         get_team_current_hp_string(state, &AttackParty(ThirdTeam)),
         get_team_total_hp_string(state, &AttackParty(ThirdTeam)),
         width = field_width,
    );
}

fn get_team_attack_string(state: &HashMap<ShiaiPosition, BattleTeam>, position: &ShiaiPosition) -> String {
    if let Some(team) = state.get(&position) {
        team.current_attack().value().to_string()
    } else {
        "".to_string()
    }
}

fn get_team_current_hp_string(state: &HashMap<ShiaiPosition, BattleTeam>, position: &ShiaiPosition) -> String {
    if let Some(team) = state.get(&position) {
        team.current_hp().value().to_string()
    } else {
        "".to_string()
    }
}

fn get_team_total_hp_string(state: &HashMap<ShiaiPosition, BattleTeam>, position: &ShiaiPosition) -> String {
    if let Some(team) = state.get(&position) {
        team.original_team().health_points().value().to_string()
    } else {
        "".to_string()
    }
}
