#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BattlePosition {
    AttackParty(TeamPosition),
    DefenseParty(TeamPosition)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TeamPosition {
    CaptainTeam,
    SecondTeam,
    ThirdTeam,
}
impl BattlePosition {
    pub fn from_parts(party: &PartyPosition, team: TeamPosition) -> Self {
        match party {
            PartyPosition::Attack => BattlePosition::AttackParty(team),
            PartyPosition::Defense => BattlePosition::DefenseParty(team),
        }
    }

    pub fn active_team(turn: u8) -> BattlePosition {
        match turn % 6 {
            1 => BattlePosition::AttackParty(TeamPosition::CaptainTeam),
            2 => BattlePosition::DefenseParty(TeamPosition::CaptainTeam),
            3 => BattlePosition::AttackParty(TeamPosition::SecondTeam),
            4 => BattlePosition::DefenseParty(TeamPosition::SecondTeam),
            5 => BattlePosition::AttackParty(TeamPosition::ThirdTeam),
            0 => BattlePosition::DefenseParty(TeamPosition::ThirdTeam),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PartyPosition {
    Attack,
    Defense,
}

#[cfg(test)]
mod tests {
    use crate::domain::{AttackParty, ThirdTeam};
    use super::*;

    #[test]
    fn test_select_active_team() {

        let selected_team = BattlePosition::active_team(35);

        assert_eq!(AttackParty(ThirdTeam), selected_team);
    }
}