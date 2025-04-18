
use crate::domain::*;


// TODO string should be i18n, outsourced and imported.
pub fn apprentice_template() -> CardTemplate {
    CardTemplate::new_replicate_active_skill(
        CardType::Camelot,
        Stars::OneStar,
        Name::new("Apprentice".to_string()),
        HealthPoints::new(1200),
        Attack::new(285),
        CardSkill::new(
            SkillName::new("Magic Bolt".to_string()),
            SkillDescription::new("Magic damage to enemy team".to_string()),
            SkillEffect::MagicDamage(BasedOnCardAttack(200))
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3)
    )
}

pub fn spy_template() -> CardTemplate {
    CardTemplate::new_replicate_active_skill(
        CardType::Camelot,
        Stars::OneStar,
        Name::new("Spy".to_string()),
        HealthPoints::new(830),
        Attack::new(205),
        CardSkill::new(
            SkillName::new("Back stab".to_string()),
            SkillDescription::new("Lower ATK of enemy team with the highest ATK - NOT IMPLEMENTED".to_string()),
            SkillEffect::MagicDamage(BasedOnCardAttack(200))
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3)
    )
}

pub fn crossbowman() -> CardTemplate {
    CardTemplate::new_replicate_active_skill(
        CardType::Camelot,
        Stars::OneStar,
        Name::new("Crossbowman".to_string()),
        HealthPoints::new(1800),
        Attack::new(440),
        CardSkill::new(
            SkillName::new("Piercing Bolt".to_string()),
            SkillDescription::new("Damage enemy team with the highest HP - NOT IMPLEMENTED".to_string()),
            SkillEffect::MagicDamage(BasedOnCardAttack(200))
        ),
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3)
    )
}

// pub fn wood_nymph_template() -> Card {
//     let id = Id::new();
//     let name = Name::new("Wood Nymph".to_string());
//     let hp = HealthPoints::new(1050);
//     let attack = Attack::new(340);
//
//     Card::new(id, name, attack, hp, vec![])
// }

// 1 level up:
// hp +31   (3%)
// attack +10 (3%)

// 2 level up
// hp +63
// attack +20

// 2 level up
// hp +97
// attack +31


// Mara of the sand: 11330
// attack 4013.
// +1 level
// +226 (2%)
// +80  (2%)  (up to star 9% is 2%)