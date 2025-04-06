use crate::domain::attack::Attack;
use crate::domain::card::name::Name;
use crate::domain::health_points::HealthPoints;
use crate::domain::*;

pub fn bard_template() -> Card {
    let id = Id::new();
    let name = Name::new("Bard".to_string());
    let attack = Attack::new(100);
    let hp = HealthPoints::new(100);
    Card::new(id, name, attack, hp, vec![passive_skill_25_hp()])
}

pub fn passive_skill_25_hp() -> Skill {
    let description = SkillDescription::new("Increases HP 25%".to_string());
    let name = SkillName::new("Heavy body".to_string());
    Skill::new_passive(name, description)
}
