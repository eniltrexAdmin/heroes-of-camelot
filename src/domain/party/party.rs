use crate::domain::Team;

pub enum PartyType {
    DefaultParty(Party),
}

pub struct Party {
    captain_team: Team,
    second_team: Option<Team>,
    third_team: Option<Team>,
}

