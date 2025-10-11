use crate::domain::*;

pub fn apprentice_template() -> CardTemplate {
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

pub fn spy_template() -> CardTemplate {
    CardTemplate::new(
        CardType::Camelot,
        Stars::OneStar,
        Name::new("Spy".to_string()),
        HealthPoints::new(830),
        Attack::new(205),
        CardSkill::new(
            SkillName::new("Back stab".to_string()),
            SkillDescription::new(
                "Lower ATK of enemy team with the highest ATK".to_string(),
            ),
            SkillEffect::DecreaseThisTurnAttack(BasedOnCardHealthPoints(50)),
            SkillTrigger::PROC(75),
            SkillTarget::Team(TeamTargetEnemyParty(HighestAttack))
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}

pub fn crossbowman() -> CardTemplate {
    CardTemplate::new(
        CardType::Camelot,
        Stars::OneStar,
        Name::new("Crossbowman".to_string()),
        HealthPoints::new(1800),
        Attack::new(440),
        CardSkill::new(
            SkillName::new("Piercing Bolt".to_string()),
            SkillDescription::new(
                "Damage enemy team with the highest HP".to_string(),
            ),
            SkillEffect::PhysicalDamage(BasedOnCardAttack(500)),
            SkillTrigger::BasedOnCard(TriggerBasedOnCardTier(200)),
            SkillTarget::Team(TeamTargetEnemyParty(LowestCurrentHp))
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}

pub fn unicorn() -> CardTemplate {
    CardTemplate::new(
        CardType::Camelot,
        Stars::OneStar,
        Name::new("Unicorn".to_string()),
        HealthPoints::new(1480),
        Attack::new(365),
        CardSkill::new(
            SkillName::new("Essence of Alicorn".to_string()),
            SkillDescription::new(
                "Raise ATK of your team with the highest attack".to_string(),
            ),
            SkillEffect::IncreaseThisTurnAttack(BasedOnCardHealthPoints(200)),
            SkillTrigger::PROC(75),
            SkillTarget::Team(TeamTargetOwnParty(HighestAttack))
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}

pub fn hunter() -> CardTemplate {
    CardTemplate::new(
        CardType::Camelot,
        Stars::OneStar,
        Name::new("Hunter".to_string()),
        HealthPoints::new(1500),
        Attack::new(360),
        CardSkill::new(
            SkillName::new("Trap".to_string()),
            SkillDescription::new(
                "Reduce attack of enemy team".to_string(),
            ),
            SkillEffect::DecreaseThisTurnAttack(BasedOnCardAttack(200)),
            SkillTrigger::BasedOnCard(TriggerBasedOnCardLevel(100)),
            SkillTarget::Team(TeamTargetEnemyParty(Default))
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}

pub fn footman() -> CardTemplate {
    CardTemplate::new(
        CardType::Camelot,
        Stars::OneStar,
        Name::new("Footman".to_string()),
        HealthPoints::new(1180),
        Attack::new(290),
        CardSkill::new(
            SkillName::new("Bravery".to_string()),
            SkillDescription::new(
                "Raise ATK of this card's team".to_string(),
            ),
            SkillEffect::IncreaseThisTurnAttack(BasedOnCardAttack(1000)),
            SkillTrigger::PROC(10),
            SkillTarget::Team(Itself)
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}

pub fn squire() -> CardTemplate {
    CardTemplate::new(
        CardType::Camelot,
        Stars::OneStar,
        Name::new("Squire".to_string()),
        HealthPoints::new(850),
        Attack::new(200),
        CardSkill::new(
            SkillName::new("Bravery".to_string()),
            SkillDescription::new(
                "Recover HP yo your team with the highest HP".to_string(),
            ),
            SkillEffect::Heal(BasedOnCardAttack(200)),
            SkillTrigger::PROC(100),
            SkillTarget::Team(TeamTargetOwnParty(HighestCurrentHp))
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3),
    )
}
