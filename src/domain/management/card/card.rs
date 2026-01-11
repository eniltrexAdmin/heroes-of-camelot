use super::*;
use crate::domain::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    id: Id,
    template: Arc<CardTemplate>,
    attack: Attack,
    health_points: HealthPoints,
    current_level: CardLevel,
    active_skill: CardSkill,
    tier: Tier,
    stars: Stars,
    max_level: CardLevel,
}
impl Card {
    pub fn new(id: Id, template: Arc<CardTemplate>) -> Self {
        let tier = Tier::init();
        let max_level = max_level(template.stars(), &tier);
        Card {
            id,
            attack: template.attack().clone(),
            health_points: template.health_points().clone(),
            stars: template.stars().clone(),
            active_skill: template.active_skills().clone(),
            template,
            current_level: CardLevel::new(1),
            tier,
            max_level,
        }
    }

    // TODO for now is stub, but later if I need to build from BE or DB, I might modify that
    pub fn stub_build(
        template: Arc<CardTemplate>,
        attack: Attack,
        health_points: HealthPoints,
        current_level: CardLevel,
        active_skill: CardSkill,
        tier: Tier,
        stars: Stars,
    ) -> Self {
        let max_level = max_level(&stars, &tier);
        Card {
            id: Id::new(),
            attack,
            health_points,
            stars,
            active_skill,
            template,
            current_level,
            tier,
            max_level,
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

    pub fn active_skill(&self) -> &CardSkill {
        &self.active_skill
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

    pub fn skill_effect_base_value(&self) -> SkillBaseValue {
        // TODO test
        let formula = self.active_skill.effect().formula();
        let base = match formula {
            BasedOnCardAttack(p) => {
                self.attack().value() * p / 100
            }
            BasedOnCardHealthPoints(p) => {
                self.health_points().value() * p / 100
            }
            EffectBasedOnCardLevel(p) => {
                self.current_level().value() as u32 * p / 100
            }
        };
        SkillBaseValue::new(base as u128)
    }

    pub fn level_up(&mut self, num_levels: u8) -> Result<(), CardManagementError> {
        if self.current_level.value() + num_levels > self.max_level.value() {
            return Err(CardManagementError::ExceededMaxLevel);
        }
        let lvl = self.current_level;
        self.current_level = lvl.level_up(num_levels);
        self.attack = self.projected_leveled_up_attack(self.current_level.value());
        self.health_points = self.projected_leveled_up_health_points(self.current_level.value());

        Ok(())
    }

    fn projected_leveled_up_attack(&self, projected_level: u8) -> Attack {
        match self.template.attack_growth_curve() {
            GrowthCurve::Percentage(percentage) => {
                let base = self.template.attack().value();
                let percentage_increment = percentage * (projected_level - 1);
                let increase = base * percentage_increment as u32 / 100;
                Attack::new(base + increase)
            }
        }
    }

    fn projected_leveled_up_health_points(&self, projected_level: u8) -> HealthPoints {
        match self.template.hp_growth_curve() {
            GrowthCurve::Percentage(percentage) => {
                let base = self.template.health_points().value();
                let percentage_increment = percentage * (projected_level - 1);
                let increase = base * percentage_increment as u32 / 100;

                println!(
                    "Result is {} + {} (+{}%)",
                    base, increase, percentage_increment
                );
                HealthPoints::new(base + increase)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CardManagementError {
    ExceededMaxLevel,
}

// TODO maybe refactor to do calculation instead of data? move that inside struct.
pub fn max_level(stars: &Stars, tier: &Tier) -> CardLevel {
    let mut ten_levels = stars.value() + tier.int_value() - 1;

    if tier == &Tier::Tier4 {
        ten_levels = ten_levels + 1
    }
    CardLevel::new(ten_levels * 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::apprentice_template;
    use crate::domain::*;
    #[test]
    fn test_new() {
        let apprentice_template = apprentice_template();
        let card = Card::new(Id::new(), Arc::new(apprentice_template));

        assert_eq!(card.template.name().value(), &"Apprentice".to_string());
        assert_eq!(&CardType::Camelot, card.card_type());
        assert_eq!(1200, card.health_points.value());
        assert_eq!(285, card.attack.value());
        assert_eq!(&Tier::Tier1, card.tier());
        assert_eq!(10, card.max_level().value());
        assert_eq!(1, card.current_level().value());
        assert_eq!("Magic Bolt", card.active_skill.name().value())
    }

    #[test]
    fn test_max_level() {
        let level = max_level(&Stars::OneStar, &Tier::Tier1);
        assert_eq!(level.value(), 10);

        let level = max_level(&Stars::OneStar, &Tier::Tier2);
        assert_eq!(level.value(), 20);

        let level = max_level(&Stars::TwoStars, &Tier::Tier3);
        assert_eq!(level.value(), 40);

        let level = max_level(&Stars::SevenStars, &Tier::Tier4);
        assert_eq!(level.value(), 110);
    }

    #[test]
    fn test_exceeded_max_level() {
        let apprentice_template = apprentice_template();
        let mut card = Card::new(Id::new(), Arc::new(apprentice_template));

        let result = card.level_up(100);
        assert!(result.is_err());
    }

    #[test]
    fn test_level_up() {
        let apprentice_template = apprentice_template();
        let mut card = Card::new(Id::new(), Arc::new(apprentice_template));

        card.level_up(1).unwrap();

        assert_eq!(1236, card.health_points().value());
        assert_eq!(293, card.attack.value());
        assert_eq!(2, card.current_level().value());

        card.level_up(1).unwrap();

        assert_eq!(1272, card.health_points().value());
        assert_eq!(302, card.attack.value());
        assert_eq!(3, card.current_level().value());
    }
}
