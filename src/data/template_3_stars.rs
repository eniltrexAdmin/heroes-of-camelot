
use crate::domain::*;

pub fn card_with_combo() -> CardTemplate {
    let combo_skill = ComboSkill::new(
        SkillName::new("Heavy body".to_string()),
        SkillDescription::new("Increases HP 25%".to_string()),
        ComboSkillEffect::Passive(AttackIncrease(25))
    );
    CardTemplate::new_replicate_active_skill(
        CardType::Camelot,
        Stars::OneStar,
        Name::new("Spy".to_string()),
        HealthPoints::new(5000),
        Attack::new(205),
        CardSkill::new(
            SkillName::new("Back stab".to_string()),
            SkillDescription::new("Lower ATK of enemy team with the highest ATK - NOT IMPLEMENTED".to_string()),
            SkillEffect::MagicDamage(BasedOnCardAttack(200))
        ),
        vec![combo_skill],
        GrowthCurve::Percentage(3),
        GrowthCurve::Percentage(3)
    )
}
//
// pub fn hunter_template() -> Card {
//     let id = Id::new();
//     let name = Name::new("Hunter".to_string());
//     let attack = Attack::new(100);
//     let hp = HealthPoints::new(100);
//     Card::new(id, name, attack, hp, vec![])
// }

// pub fn passive_skill_25_hp() -> ActiveSkillEffect {
//     let description = SkillDescription::new("Increases HP 25%".to_string());
//     let name = SkillName::new("Heavy body".to_string());
//     let effect = SkillEffect::Passive(HealthPointsIncrease(25));
//     ActiveSkillEffect::new(name, description, effect)
// }
//
//
// pub fn passive_skill_25_attack() -> ActiveSkillEffect {
//     let description = SkillDescription::new("Increases HP 25%".to_string());
//     let name = SkillName::new("Heavy body".to_string());
//     let effect = SkillEffect::Passive(AttackIncrease(25));
//     ActiveSkillEffect::new(name, description, effect)
// }