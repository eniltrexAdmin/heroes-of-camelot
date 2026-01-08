use rand::Rng;
use crate::domain::*;



pub fn choose_skill(team: &Team) -> Option<&Card> {
    std::iter::once(&team.captain())
        .chain(team.second())
        .chain(team.third())
        .chain(team.fourth())
        .find(|card| skill_procs(card))
}

fn skill_procs(card: &Card) -> bool {
    let effective_chance = trigger_effective_chance(card);
    // there could be extra modifiers here on "effective_chance".
    // needs "battle", the current game for that though!
    let mut rng = rand::rng();
    does_it_proc(effective_chance, &mut rng)
}

fn does_it_proc(proc_rate: ProcRate, rng: &mut impl Rng) -> bool {
    rng.random_range(0..100) < proc_rate
}



#[cfg(test)]
mod tests {
    use crate::domain::stubs::{empty_card_with_skill, skill_builder};
    use super::*;

    #[test]
    fn test_skill_procs() {
            // this should test the "other modifiers".
    }

    #[test]
    fn test_choose_skill() {
        let always_trigger_skill = CardSkill::new(
            SkillName::new("always triggers".to_string()),
            SkillDescription::new("always triggers".to_string()),
            SkillEffect::PhysicalDamage(BasedOnCardAttack(100)),
            SkillTrigger::PROC(100),
            SkillTarget::Team(TeamTargetEnemyParty(Default))
        );
        let card_trigger = empty_card_with_skill(always_trigger_skill);
        let never_trigger_skill = CardSkill::new(
            SkillName::new("never triggers".to_string()),
            SkillDescription::new("never triggers".to_string()),
            SkillEffect::PhysicalDamage(BasedOnCardAttack(100)),
            SkillTrigger::PROC(0),
            SkillTarget::Team(TeamTargetEnemyParty(Default))
        );

        let card_no_trigger = empty_card_with_skill(never_trigger_skill);
        let team = Team::new(
            card_trigger.clone(),
            Some(card_no_trigger.clone()),
            Some(card_no_trigger.clone()),
            Some(card_no_trigger.clone()),
            vec![]
        ).unwrap();

        let skill = choose_skill(&team).unwrap();


    }
}