use std::rc::Rc;
use crate::data::*;
use crate::domain::*;

pub fn combo_skill_finder(card: &Vec<Card>) -> Vec<ComboSkill> {

    vec![]
}


pub fn stub_team()-> Team {
    let mut cards = vec![];
    let apprentice_template = apprentice_template();
    let apprentice_card = Card::new(Id::new(), Rc::new(apprentice_template));
    cards.push(apprentice_card.clone());
    cards.push(apprentice_card.clone());
    cards.push(apprentice_card.clone());
    cards.push(apprentice_card.clone());

    let team = team_factory(
        cards.clone(),
        combo_skill_finder
    ).unwrap();

    match team {
        DefaultTeam(team) => team,
    }
}


pub fn stub_party() -> Party {
    let default_team = stub_team();
    Party::new(
        default_team.clone(),
        Some(default_team.clone()),
        Some(default_team.clone()),
    )
}