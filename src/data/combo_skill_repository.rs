use std::rc::Rc;
use crate::data::*;
use crate::domain::*;

pub fn combo_skill_finder(card: &Vec<Card>) -> Vec<ComboSkill> {

    vec![]
}


pub fn stub_team()-> Team {
    let mut cards = vec![];
    cards.push(Card::new(Id::new(), Rc::new(apprentice_template())));
    cards.push(Card::new(Id::new(), Rc::new(spy_template())));
    cards.push(Card::new(Id::new(), Rc::new(crossbowman())));
    cards.push(Card::new(Id::new(), Rc::new(druid_initiae())));

    let team = team_factory(
        cards.clone(),
        combo_skill_finder
    ).unwrap();

    match team {
        DefaultTeam(team) => team,
    }
}

pub fn stub_team_2()-> Team {
    let mut cards = vec![];
    cards.push(Card::new(Id::new(), Rc::new(druid_marksman())));
    cards.push(Card::new(Id::new(), Rc::new(water_nymph())));
    cards.push(Card::new(Id::new(), Rc::new(druid_scout())));
    cards.push(Card::new(Id::new(), Rc::new(druid_sage())));

    let team = team_factory(
        cards.clone(),
        combo_skill_finder
    ).unwrap();

    match team {
        DefaultTeam(team) => team,
    }
}

pub fn stub_team_3()-> Team {
    let mut cards = vec![];
    cards.push(Card::new(Id::new(), Rc::new(druid_warrior())));
    cards.push(Card::new(Id::new(), Rc::new(wood_nymph())));
    cards.push(Card::new(Id::new(), Rc::new(unicorn())));
    cards.push(Card::new(Id::new(), Rc::new(hunter())));

    let team = team_factory(
        cards.clone(),
        combo_skill_finder
    ).unwrap();

    match team {
        DefaultTeam(team) => team,
    }
}


pub fn stub_party() -> Party {
    Party::new(
        stub_team(),
        Some(stub_team_2()),
        Some(stub_team_3()),
    )
}

pub fn stub_party_2() -> Party {
    Party::new(
        stub_team_2(),
        Some(stub_team_3()),
        Some(stub_team()),
    )
}