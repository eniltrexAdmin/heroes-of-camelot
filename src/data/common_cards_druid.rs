use crate::domain::*;

pub fn druid_initiate() -> CardTemplate {
    CardTemplate::new_replicate_active_skill(
        CardType::Druid,
        Stars::OneStar,
        Name::new("Druid Initiate".to_string()),
        HealthPoints::new(1200),
        Attack::new(285),
        CardSkill::new(
            SkillName::new("Alleviate Pain".to_string()),
            SkillDescription::new("Recover HP to this card's if enemy team has squire - NOT IMPLEMENTED".to_string()),
            SkillEffect::MagicDamage(BasedOnCardAttack(200))
        ),
        vec![],
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3)
    )
}