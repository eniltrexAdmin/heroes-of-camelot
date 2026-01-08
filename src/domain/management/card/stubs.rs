use std::sync::Arc;
use crate::domain::*;

pub fn skill_builder(
    effect: Option<SkillEffect>,
    trigger: Option<SkillTrigger>,
    skill_target: Option<SkillTarget>
) -> CardSkill {
    let default_effect = SkillEffect::PhysicalDamage(BasedOnCardAttack(100));
    let default_trigger = SkillTrigger::PROC(50);
    let default_target = SkillTarget::Team(TeamTargetEnemyParty(Default));
    CardSkill::new(
        SkillName::new("Testing".to_string()),
        SkillDescription::new(
            "Testing purposes".to_string(),
        ),
        effect.unwrap_or(default_effect),
        trigger.unwrap_or(default_trigger),
        skill_target.unwrap_or(default_target)
    )
}

pub fn empty_template() -> CardTemplate {
    CardTemplate::new(
        CardType::Camelot,
        Stars::OneStar,
        Name::new("Apprentice".to_string()),
        HealthPoints::new(1200),
        Attack::new(285),
        CardSkill::new(
            SkillName::new("Magic Bolt".to_string()),
            SkillDescription::new("Magic damage to enemy team".to_string()),
            SkillEffect::MagicDamage(BasedOnCardAttack(200)),
            SkillTrigger::PROC(25),
            SkillTarget::Team(TeamTargetEnemyParty(Default))
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}

pub fn empty_card_with_skill(skill: CardSkill) -> Card {
   Card::stub_build(
        Arc::new(empty_template()),
        Attack::new(285),
        HealthPoints::new(1200),
        CardLevel::new(1),
        skill,
        Tier::Tier2,
        Stars::FourStars
    )
}