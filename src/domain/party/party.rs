use crate::domain::Team;

pub enum PartyType {
    DefaultParty(Party),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Party {
    captain_team: Team,
    second_team: Option<Team>,
    third_team: Option<Team>,
}

impl Party {
    pub fn new(captain_team: Team, second_team: Option<Team>, third_team: Option<Team>) -> Self {
        Self{
            captain_team,
            second_team,
            third_team,
        }
    }

    pub fn captain_team(&self) -> &Team {
        &self.captain_team
    }

    pub fn second_team(&self) -> Option<&Team> {
        self.second_team.as_ref()
    }

    pub fn third_team(&self) -> Option<&Team> {
        self.third_team.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::stub_team;
    use super::*;

    #[test]
    fn party_creation() {
        let default_team = stub_team();
        let party = Party::new(
            default_team.clone(),
            Some(default_team.clone()),
            None,
        );

        assert_eq!(party.captain_team(), &default_team);
        assert!(party.second_team().is_some());
        assert!(party.third_team().is_none());

        assert_eq!(party.second_team().unwrap(), &default_team);

    }
}