#[derive(Debug, Clone, PartialEq)]
pub enum ShiaiPosition {
    AttackParty(TeamPosition),
    DefenseParty(TeamPosition)
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

#[derive(Debug, Clone, PartialEq)]
pub enum TeamPosition {
    CaptainTeam,
    SecondTeam,
    ThirdTeam,
}