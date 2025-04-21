use crate::domain::*;
use super::*;

pub struct BattleParty {
    captain_team: BattleTeam,
    second_team: Option<BattleTeam>,
    third_team: Option<BattleTeam>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TeamSelect{
    CaptainTeam,
    SecondTeam,
    ThirdTeam,
}

impl BattleParty {
    pub fn new(party: Party) -> BattleParty {
        BattleParty{
            captain_team: BattleTeam::new(party.captain_team().clone()),
            second_team: party.second_team().map(|t|BattleTeam::new(t.clone())),
            third_team: party.third_team().map(|t|BattleTeam::new(t.clone()))
        }
    }

    pub fn second_team(&self) -> Option<&BattleTeam> {
        self.second_team.as_ref()
    }

    pub fn third_team(&self) -> Option<&BattleTeam> {
        self.third_team.as_ref()
    }

    pub fn get_team_clone(&self, team_select: &TeamSelect) -> BattleTeam {
        match team_select {
            TeamSelect::CaptainTeam => self.captain_team.clone(),
            TeamSelect::SecondTeam => self.second_team.clone().unwrap(),
            TeamSelect::ThirdTeam => self.third_team.clone().unwrap()
        }
    }

    pub fn set_team(&mut self,  team_select: &TeamSelect, team: BattleTeam) {
        match team_select {
            TeamSelect::CaptainTeam => self.captain_team = team,
            TeamSelect::SecondTeam => self.second_team = Some(team),
            TeamSelect::ThirdTeam => self.third_team = Some(team)
        }
    }
}