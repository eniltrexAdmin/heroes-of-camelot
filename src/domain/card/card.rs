use std::rc::Rc;
use super::*;
use crate::domain::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    id: Id,
    template: Rc<CardTemplate>,
    attack: Attack,
    health_points: HealthPoints,
    current_level: CardLevel,
    active_skill: CardSkill,
    tier: Tier,
    stars: Stars,
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
            stars: template.stars().clone(),
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
    pub fn card_type(&self) -> &CardType {
        self.template.card_type()
    }
    pub fn tier(&self) -> &Tier {
        &self.tier
    }
    pub fn max_level(&self) -> &CardLevel {
        &self.max_level
    }
    pub fn current_level(&self) -> &CardLevel {
        &self.current_level
    }

}

#[cfg(test)]
mod tests {
    use crate::data::apprentice_template;
    use crate::domain::*;
    use super::*;
    #[test]
    fn test_new() {
        let apprentice_template = apprentice_template();

        let card = Card::new(Id::new(), Rc::new(apprentice_template));

        assert_eq!(card.template.name().value(), &"Apprentice".to_string());
        assert_eq!(&CardType::Camelot, card.card_type());
        assert_eq!(1200, card.health_points.value());
        assert_eq!(285, card.attack.value());
        assert_eq!(&Tier::Tier1, card.tier());
        assert_eq!(10, card.max_level().value());
        assert_eq!(1, card.current_level().value());
        assert_eq!("Magic Bolt", card.active_skill.name().value())
    }
}
