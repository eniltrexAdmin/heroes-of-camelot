use crate::domain::*;

pub fn druid_initiae() -> CardTemplate {
    CardTemplate::new(
        CardType::Druid,
        Stars::OneStar,
        Name::new("Druid Initiate".to_string()),
        HealthPoints::new(765),
        Attack::new(245),
        CardSkill::new(
            SkillName::new("Alleviate Pain".to_string()),
            SkillDescription::new(
                "Recover HP".to_string(),
            ),
            SkillEffect::Heal(BasedOnCardAttack(200)),
            SkillTrigger::PROC(50),
            SkillTarget::Team(Itself)
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}

pub fn druid_marksman() -> CardTemplate {
    CardTemplate::new(
        CardType::Druid,
        Stars::OneStar,
        Name::new("Druid Marksman".to_string()),
        HealthPoints::new(1680),
        Attack::new(518),
        CardSkill::new(
            SkillName::new("Poisoned Arrow".to_string()),
            SkillDescription::new(
                "Damage enemy team ".to_string(),
            ),
            SkillEffect::PhysicalDamage(BasedOnCardAttack(200)),
            SkillTrigger::PROC(50),
            SkillTarget::Team(TeamTargetEnemyParty(Default))
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}

pub fn water_nymph() -> CardTemplate {
    CardTemplate::new(
        CardType::Druid,
        Stars::OneStar,
        Name::new("Water Nymph".to_string()),
        HealthPoints::new(1360),
        Attack::new(430),
        CardSkill::new(
            SkillName::new("Soak".to_string()),
            SkillDescription::new(
                "Reduce attack of enemy team ".to_string(),
            ),
            SkillEffect::DecreaseThisTurnAttack(BasedOnCardAttack(200)),
            SkillTrigger::PROC(100),
            SkillTarget::Team(TeamTargetEnemyParty(Default)),
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}

pub fn druid_scout() -> CardTemplate {
    CardTemplate::new(
        CardType::Druid,
        Stars::OneStar,
        Name::new("Druid Scout".to_string()),
        HealthPoints::new(750),
        Attack::new(250),
        CardSkill::new(
            SkillName::new("Intercept".to_string()),
            SkillDescription::new(
                "Lower attack of enemy team".to_string(),
            ),
            SkillEffect::MagicDamage(BasedOnCardAttack(200)),
            SkillTrigger::PROC(50),
            SkillTarget::Team(TeamTargetEnemyParty(Default)),
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}

pub fn druid_sage() -> CardTemplate {
    CardTemplate::new(
        CardType::Druid,
        Stars::OneStar,
        Name::new("Druid Sage".to_string()),
        HealthPoints::new(1375),
        Attack::new(425),
        CardSkill::new(
            SkillName::new("Healing Tonic".to_string()),
            SkillDescription::new(
                "Recover HP to this card's team".to_string(),
            ),
            SkillEffect::Heal(BasedOnCardAttack(200)),
            SkillTrigger::PROC(75),
            SkillTarget::Team(Itself),
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}

pub fn druid_warrior() -> CardTemplate {
    CardTemplate::new(
        CardType::Druid,
        Stars::OneStar,
        Name::new("Druid Warrior".to_string()),
        HealthPoints::new(1065),
        Attack::new(335),
        CardSkill::new(
            SkillName::new("Agility".to_string()),
            SkillDescription::new(
                "Raise ATK of this card's team".to_string(),
            ),
            SkillEffect::IncreaseThisTurnAttack(BasedOnCardAttack(200)),
            SkillTrigger::PROC(50),
            SkillTarget::Team(Itself),
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}

pub fn nymph_skill() -> CardSkill {
    CardSkill::new(
        SkillName::new("Heal".to_string()),
        SkillDescription::new(
            "Recover HP to your party".to_string(),
        ),
        SkillEffect::Heal(BasedOnCardAttack(200)),
        SkillTrigger::PROC(100),
        SkillTarget::Party(OwnParty)
    )
}

pub fn wood_nymph() -> CardTemplate {
    CardTemplate::new(
        CardType::Druid,
        Stars::OneStar,
        Name::new("Wood Nymph".to_string()),
        HealthPoints::new(1050),
        Attack::new(340),
        nymph_skill(),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}