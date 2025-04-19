use super::*;
use crate::domain::*;

#[derive(Debug, PartialEq)]
pub struct CardTemplate {
    identifier: Name,
    attack: Attack,
    health_points: HealthPoints,
    card_skills: TemplateSkills,
    card_type: CardType,
    stars: Stars,
    hp_growth_curve: GrowthCurve,
    attack_growth_curve: GrowthCurve,
}

impl CardTemplate {
    pub fn new(
        card_type: CardType,
        stars: Stars,
        name: Name,
        health_points: HealthPoints,
        attack: Attack,
        card_skills: TemplateSkills,
        hp_growth_curve: GrowthCurve,
        attack_growth_curve: GrowthCurve,
    ) -> Self {
        Self {
            card_type,
            stars,
            identifier: name,
            health_points,
            attack,
            card_skills,
            hp_growth_curve,
            attack_growth_curve,
        }
    }
    pub fn new_replicate_active_skill(
        card_type: CardType,
        stars: Stars,
        name: Name,
        health_points: HealthPoints,
        attack: Attack,
        card_skill: CardSkill,
        hp_growth_curve: GrowthCurve,
        attack_growth_curve: GrowthCurve,
    ) -> Self {
        let card_skills = TemplateSkills::new_from_one(card_skill, Tier::vec_tier(&stars));
        CardTemplate::new(
            card_type,
            stars,
            name,
            health_points,
            attack,
            card_skills,
            hp_growth_curve,
            attack_growth_curve,
        )
    }

    pub fn name(&self) -> &Name {
        &self.identifier
    }

    pub fn stars(&self) -> &Stars {
        &self.stars
    }

    pub fn card_type(&self) -> &CardType {
        &self.card_type
    }

    pub fn health_points(&self) -> &HealthPoints {
        &self.health_points
    }

    pub fn attack(&self) -> &Attack {
        &self.attack
    }

    pub fn active_skills(&self) -> &TemplateSkills {
        &self.card_skills
    }

    pub fn hp_growth_curve(&self) -> &GrowthCurve {
        &self.hp_growth_curve
    }

    pub fn attack_growth_curve(&self) -> &GrowthCurve {
        &self.attack_growth_curve
    }
}

#[derive(Debug, PartialEq)]
pub enum CardType {
    Camelot,
    Druid,
    Demon,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::*;

    fn test_create_template() {
        let name = Name::new("nymph".to_string());
        let attack = Attack::new(300);
        let health_points = HealthPoints::new(1080);
        let active_skill = CardSkill::new(
            SkillName::new("Some skill".to_string()),
            SkillDescription::new("Some description".to_string()),
            SkillEffect::IncreaseThisTurnAttack(BasedOnCardAttack(300)),
        );

        let template = CardTemplate::new_replicate_active_skill(
            CardType::Camelot,
            Stars::OneStar,
            name.clone(),
            health_points.clone(),
            attack.clone(),
            active_skill.clone(),
            GrowthCurve::Percentage(2),
            GrowthCurve::Percentage(2),
        );
        assert_eq!(template.stars(), &Stars::OneStar);
        assert_eq!(template.name(), &name);
        assert_eq!(template.attack(), &attack);
        assert_eq!(template.health_points(), &health_points);
        assert_eq!(template.card_type(), &CardType::Camelot);
        assert_eq!(template.name(), &name);
        assert!(template
            .active_skills()
            .value()
            .values()
            .all(|s| s == &active_skill));
    }
}
