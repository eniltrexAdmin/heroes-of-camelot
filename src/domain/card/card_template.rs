use crate::domain::*;
use super::*;

pub struct CardTemplate {
    name: Name,
    attack: Attack,
    health_points: HealthPoints,
    active_skills: TemplateActiveSkills,
    combo_skills: Vec<ComboSkill>, // one exercise would be to have different of those per tier!
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
        attack: Attack,
        health_points: HealthPoints,
        active_skills: ActiveSkill,
        combo_skills: Vec<ComboSkill>,
        hp_growth_curve: GrowthCurve,
        attack_growth_curve: GrowthCurve,
    ) -> Self {
        let active_skills = TemplateActiveSkills::new_from_one(
            active_skills,
            Tier::vec_tier(&stars)
        );
        Self{
            card_type,
            stars,
            name,
            health_points,
            attack,
            active_skills,
            combo_skills,
            hp_growth_curve,
            attack_growth_curve,
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn start(&self) -> &Stars {
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

    pub fn active_skills(&self) -> &TemplateActiveSkills {
        &self.active_skills
    }

    pub fn combo_skills(&self) -> &[ComboSkill] {
        &self.combo_skills
    }

    pub fn hp_growth_curve(&self) -> &GrowthCurve {
        &self.hp_growth_curve
    }

    pub fn attack_growth_curve(&self) -> &GrowthCurve {
        &self.attack_growth_curve
    }
}

#[derive(Debug, PartialEq)]
pub enum CardType{
    Camelot,
    Druid,
    Demon
}

#[cfg(test)]
mod tests {
    use crate::domain::*;
    use super::*;

    fn test_create_template() {
        let name = Name::new("nymph".to_string());
        let attack = Attack::new(300);
        let health_points = HealthPoints::new(1080);
        let active_skill = ActiveSkill::new(
            SkillName::new("Some skill".to_string()),
            SkillDescription::new("Some description".to_string()),
            IncreaseThisTurnAttack(300)
        );

        let template = CardTemplate::new(
            CardType::Camelot,
            Stars::OneStar,
            name.clone(),
            attack.clone(),
            health_points.clone(),
            active_skill.clone(),
            vec![],
            GrowthCurve::Percentage(2),
            GrowthCurve::Percentage(2)
        );

        assert_eq!(template.name(), &name);
        assert_eq!(template.attack(), &attack);
        assert_eq!(template.health_points(), &health_points);
        assert_eq!(template.card_type(), &CardType::Camelot);
        assert_eq!(template.name(), &name);
        assert!(template.active_skills().value().values().all(|s| s == &active_skill));
    }
}