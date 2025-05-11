#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShiaiPosition {
    AttackParty(TeamPosition),
    DefenseParty(TeamPosition)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TeamPosition {
    CaptainTeam,
    SecondTeam,
    ThirdTeam,
}
impl ShiaiPosition {
    pub fn from_parts(party: &PartyPosition, team: TeamPosition) -> Self {
        match party {
            PartyPosition::Attack => ShiaiPosition::AttackParty(team),
            PartyPosition::Defense => ShiaiPosition::DefenseParty(team),
        }
    }

    pub fn active_team(turn: u8) -> ShiaiPosition {
        match turn % 6 {
            1 => ShiaiPosition::AttackParty(TeamPosition::CaptainTeam),
            2 => ShiaiPosition::DefenseParty(TeamPosition::CaptainTeam),
            3 => ShiaiPosition::AttackParty(TeamPosition::SecondTeam),
            4 => ShiaiPosition::DefenseParty(TeamPosition::SecondTeam),
            5 => ShiaiPosition::AttackParty(TeamPosition::ThirdTeam),
            0 => ShiaiPosition::DefenseParty(TeamPosition::ThirdTeam),
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

        let selected_team = ShiaiPosition::active_team(35);

        assert_eq!(AttackParty(ThirdTeam), selected_team);
    }
}