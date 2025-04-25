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
}

#[derive(Debug, Clone, PartialEq)]
pub enum PartyPosition {
    Attack,
    Defense,
}

