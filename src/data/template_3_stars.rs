
use crate::domain::*;

pub fn bard_template() -> Card {
    let id = Id::new();
    let name = Name::new("Bard".to_string());
    let attack = Attack::new(100);
    let hp = HealthPoints::new(100);
    Card::new(id, name, attack, hp, vec![passive_skill_25_hp()])
}

pub fn hunter_template() -> Card {
    let id = Id::new();
    let name = Name::new("Hunter".to_string());
    let attack = Attack::new(100);
    let hp = HealthPoints::new(100);
    Card::new(id, name, attack, hp, vec![passive_skill_25_attack()])
}

pub fn passive_skill_25_hp() -> Skill {
    let description = SkillDescription::new("Increases HP 25%".to_string());
    let name = SkillName::new("Heavy body".to_string());
    let effect = SkillEffect::Passive(HealthPointsIncrease(25));
    Skill::new(name, description, effect)
}


pub fn passive_skill_25_attack() -> Skill {
    let description = SkillDescription::new("Increases HP 25%".to_string());
    let name = SkillName::new("Heavy body".to_string());
    let effect = SkillEffect::Passive(AttackIncrease(25));
    Skill::new(name, description, effect)
}