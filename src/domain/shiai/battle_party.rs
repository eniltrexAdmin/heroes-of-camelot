use crate::domain::*;
use super::*;

pub struct BattleParty {
    captain_team: BattleTeam,
    second_team: Option<BattleTeam>,
    third_team: Option<BattleTeam>,
}

impl BattleParty {

    // TODO test creator
    pub fn new(party: Party, position: &PartyPosition) -> BattleParty {
        BattleParty{
            captain_team: BattleTeam::new(
                party.captain_team().clone(),
                ShiaiPosition::from_parts(position, CaptainTeam)
            ),
            second_team: party.second_team().map(|t|BattleTeam::new(
                t.clone(),
                ShiaiPosition::from_parts(position, SecondTeam)
            )),
            third_team: party.third_team().map(|t|BattleTeam::new(
                t.clone(),
                ShiaiPosition::from_parts(position, ThirdTeam)
            ))
        }
    }

    pub fn captain_team(&self) -> &BattleTeam {
        &self.captain_team
    }

    pub fn second_team(&self) -> Option<&BattleTeam> {
        self.second_team.as_ref()
    }

    pub fn third_team(&self) -> Option<&BattleTeam> {
        self.third_team.as_ref()
    }

    pub fn get_team_mut_ref(&mut self, team_select: &TeamPosition) -> &mut BattleTeam {
        match team_select {
            CaptainTeam => &mut self.captain_team,
            SecondTeam => {
                let team = self.second_team.as_mut().expect("Second team missing");
                team
            },
            ThirdTeam => {
                let team = self.third_team.as_mut().expect("Third team missing");
                team
            }
        }
    }

    pub fn get_team_clone(&self, team_select: &TeamPosition) -> BattleTeam {
        match team_select {
            CaptainTeam => self.captain_team.clone(),
            SecondTeam => self.second_team.clone().unwrap(),
            ThirdTeam => self.third_team.clone().unwrap()
        }
    }

    pub fn set_team(&mut self, team_select: &TeamPosition, team: BattleTeam) {
        match team_select {
            CaptainTeam => self.captain_team = team,
            SecondTeam => self.second_team = Some(team),
            ThirdTeam => self.third_team = Some(team)
        }
    }

    pub fn choose_attack_target(&mut self) -> &mut BattleTeam {
        if self.captain_team().is_alive() {
            return self.get_team_mut_ref(&CaptainTeam)
        }
        if let Some(second_team) = self.second_team() {
            if second_team.is_alive() {
                return self.get_team_mut_ref(&SecondTeam)
            }
        }
        if let Some(third_team) = self.third_team() {
            if third_team.is_alive() {
                return self.get_team_mut_ref(&ThirdTeam)
            }
        }
        panic!("No team to attack!")
    }

    pub fn format_pretty(&self, defender: bool) -> String {
        let mut out = String::new();

        let mut rows = vec![];

        // Gather teams in order
        let teams = vec![
            Some(&self.captain_team),
            self.second_team.as_ref(),
            self.third_team.as_ref(),
        ];

        // Collect attacks and HPs
        let attacks: Vec<String> = teams.iter()
            .map(|team| {
                team.map(|t| format!(
                    "{:^10}", format!("{:?}", t.current_attack().value()))) // centered
                    .unwrap_or_else(|| "   -   ".to_string())
            })
            .collect();

        let hps: Vec<String> = teams.iter()
            .map(|team| {
                team.map(|t| {
                    let total_hp = t.original_team().health_points(); // Assuming you can get this
                    format!("{:^10}", format!("{}/{}", t.current_hp().value(), total_hp.value()))
                }).unwrap_or_else(|| "   -   ".to_string())
            })
            .collect();

        // Build rows in desired order
        if defender {
            rows.push(format!("HPs:    {}", hps.join(" | ")));
            rows.push(format!("Attack: {}", attacks.join(" | ")));
        } else {
            rows.push(format!("Attack: {}", attacks.join(" | ")));
            rows.push(format!("HPs:    {}", hps.join(" | ")));
        }

        for row in rows {
            out.push_str(&row);
            out.push('\n');
        }

        out
    }
}