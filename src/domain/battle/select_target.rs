use super::*;

pub enum TargetStrategy {
    Default,
}

pub fn select_target(current_state: &BattleState, target_party: PartyPosition, strategy: TargetStrategy) -> BattlePosition {
    match strategy {
        _ => select_default_target(current_state, &target_party),
    }
}

fn select_default_target(current_state: &BattleState, target_party: &PartyPosition) -> BattlePosition {
    match target_party {
        PartyPosition::Attack => get_defense_party_alive_team(current_state),
        PartyPosition::Defense => get_attack_party_alive_team(current_state),
    }
}

fn get_defense_party_alive_team(current_state: &BattleState) -> BattlePosition {
    [DefenseParty(CaptainTeam), DefenseParty(SecondTeam), DefenseParty(ThirdTeam)]
        .into_iter()
        .find(|position| current_state.get(position).map_or(
            false, |team| team.is_alive()))
        .expect("No alive team found in defense party")

}

fn get_attack_party_alive_team(current_state:&BattleState) -> BattlePosition {
    [AttackParty(CaptainTeam), AttackParty(SecondTeam), AttackParty(ThirdTeam)]
        .into_iter()
        .find(|position| current_state.get(position).map_or(
            false, |team| team.is_alive()))
        .expect("No alive team found in defense party")
}


#[cfg(test)]
mod tests {
// TEst with empty teams etc, we might want to return REsults instead in here.
}