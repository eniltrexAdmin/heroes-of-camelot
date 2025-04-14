use std::rc::Rc;
use super::*;
use crate::domain::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    id: Id,
    template: Rc<CardTemplate>,
    attack: Attack,
    health_points: HealthPoints,
    current_level: CardLevel,
    active_skill: ActiveSkill,
    tier: Tier,
    max_level: CardLevel,
}
impl Card {
    pub fn new(
        id: Id,
        template: Rc<CardTemplate>,
    ) -> Self {
        let tier = Tier::init();
        let active_skill = template.active_skills().value().get(&tier).unwrap().clone();
        let max_level = CardLevel::max_level(template.stars(), &tier);
        Card {
            id,
            attack: template.attack().clone(),
            health_points: template.health_points().clone(),
            template,
            current_level: CardLevel::init(),
            tier,
            active_skill,
            max_level
        }
    }
    pub fn id(&self) -> &Id {
        &self.id
    }
    pub fn name(&self) -> &Name {
        &self.template.name()
    }
    pub fn attack(&self) -> &Attack {
        &self.attack
    }
    pub fn health_points(&self) -> &HealthPoints {
        &self.health_points
    }

}

#[cfg(test)]
mod tests {
    use crate::domain::PassiveSkill::HealthPointsIncrease;
    use super::*;
    #[test]
    fn test_new() {
        // let id = Id::new();
        // let name = Name::new("Bard".to_string());
        // let attack = Attack::new(100);
        // let hp = HealthPoints::new(100);
        // // let passive_skill = ActiveSkillEffect::new(
        // //     SkillName::new("Heavy body".to_string()),
        // //     SkillDescription::new("Increases HP 100%".to_string()),
        // //     SkillEffect::Passive(HealthPointsIncrease(25))
        // // );
        //
        // let skills = vec![];
        //
        // let card = Card::new(id.clone(), name.clone(), attack, hp, skills.clone());
        //
        // assert_eq!(card.id, id);
        // assert_eq!(card.name(), &name);
        // assert_eq!(card.attack, attack);
        // assert_eq!(card.health_points, hp);
        // assert_eq!(card.skills, skills);
    }
}
