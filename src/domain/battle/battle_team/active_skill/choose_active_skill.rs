use rand::Rng;
use super::*;

impl BattleTeam{
    pub fn choose_active_skill(&self) -> Option<&Card> {
        self.original_team.card_iterator().find(|card| skill_procs(card))
    }
}

fn skill_procs(card: &Card) -> bool {
    let effective_chance = trigger_effective_chance(card);
    // there could be extra modifiers here on "effective_chance".
    // needs "battle", the current game for that though!
    let mut rng = rand::rng();
    does_it_proc(effective_chance, &mut rng)
}


fn trigger_effective_chance(card: &Card) -> ProcRate{
    match card.active_skill().trigger() {
        SkillTrigger::PROC(proc) => {
            *proc
        },
        SkillTrigger::BasedOnCard(formula) => {
            evaluate_skill_trigger_formula(formula, card)
        }
    }
}

fn evaluate_skill_trigger_formula(formula: &SkillTriggerValueFormula, card: &Card) -> ProcRate {
    let (value, percentage) = match formula {
        TriggerBasedOnCardLevel(percent) =>  {
            (card.current_level().value(), percent)
        },
        TriggerBasedOnCardTier(percent) => {
            (card.tier().int_value(), percent)
        }
    };

    let final_percentage = value as u32 * percentage / 100;
    final_percentage as ProcRate
}

fn does_it_proc(proc_rate: ProcRate, rng: &mut impl Rng) -> bool {
    rng.random_range(0..100) < proc_rate
}



#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::domain::stubs::{empty_card_with_skill, empty_template, skill_builder};
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

        let battle_team = BattleTeam::new(team, AttackParty(CaptainTeam));

        let triggering_card = battle_team.choose_active_skill().unwrap();
        assert_eq!(&card_trigger, triggering_card);
    }

    #[test]
    fn test_trigger_effective_chance() {
        let mut rng = rand::rng();
        let random_number: u32 = rng.random_range( ..100);
        let skill =  skill_builder(None, Some(SkillTrigger::PROC(random_number)), None);
        let card = Card::stub_build(
            Arc::new(empty_template()),
            Attack::new(285),
            HealthPoints::new(1200),
            CardLevel::new(40),
            skill,
            Tier::Tier4,
            Stars::FourStars
        );
        let result = trigger_effective_chance(&card);
        assert_eq!(result, random_number);

        let skill =  skill_builder(
            None,
            Some(SkillTrigger::BasedOnCard(TriggerBasedOnCardLevel(100))),
            None
        );
        let card = Card::stub_build(
            Arc::new(empty_template()),
            Attack::new(285),
            HealthPoints::new(1200),
            CardLevel::new(40),
            skill,
            Tier::Tier4,
            Stars::FourStars
        );
        let result = trigger_effective_chance(&card);
        assert_eq!(40, result);
    }


    fn test_evaluate_formula_card_level() {
        let mut card = Card::stub_build(
            Arc::new(empty_template()),
            Attack::new(285),
            HealthPoints::new(1200),
            CardLevel::new(1),
            skill_builder(None, None, None), // not using this one
            // and just going faster knowing the "innards" of this code
            // the proper interface of this module is tested above
            // but that's a lot of boilerplate here creating a card for each test...
            // that's why I bypass it.
            Tier::Tier4,
            Stars::FourStars
        );

        let formula = TriggerBasedOnCardLevel(50);
        let formula2 = TriggerBasedOnCardLevel(100);
        let formula3 = TriggerBasedOnCardLevel(150);
        let formula4 = TriggerBasedOnCardLevel(200);

        assert_eq!(0,  evaluate_skill_trigger_formula(&formula, &card));
        assert_eq!(1,  evaluate_skill_trigger_formula(&formula2, &card));
        assert_eq!(1,  evaluate_skill_trigger_formula(&formula3, &card));
        assert_eq!(2,  evaluate_skill_trigger_formula(&formula4, &card));


        card.level_up(39).unwrap(); //lvl 40

        assert_eq!(20,  evaluate_skill_trigger_formula(&formula, &card));
        assert_eq!(40,  evaluate_skill_trigger_formula(&formula2, &card));
        assert_eq!(60,  evaluate_skill_trigger_formula(&formula3, &card));
        assert_eq!(80,  evaluate_skill_trigger_formula(&formula4, &card));

        card.level_up(40).unwrap();  // lvl 80
        assert_eq!(40,  evaluate_skill_trigger_formula(&formula, &card));
        assert_eq!(80,  evaluate_skill_trigger_formula(&formula2, &card));
        assert_eq!(120,  evaluate_skill_trigger_formula(&formula3, &card));
        assert_eq!(160,  evaluate_skill_trigger_formula(&formula4, &card));


    }

    #[test]
    fn test_evaluate_formula_card_tier() {
        let card = Card::stub_build(
            Arc::new(empty_template()),
            Attack::new(285),
            HealthPoints::new(1200),
            CardLevel::new(1),
            skill_builder(None, None, None), // not using this one
            Tier::Tier2,
            Stars::FourStars
        );

        let formula = TriggerBasedOnCardTier(50);
        let formula2 = TriggerBasedOnCardTier(100);
        let formula3 = TriggerBasedOnCardTier(150);
        let formula4 = TriggerBasedOnCardTier(200);

        assert_eq!(1,  evaluate_skill_trigger_formula(&formula, &card));
        assert_eq!(2,  evaluate_skill_trigger_formula(&formula2, &card));
        assert_eq!(3,  evaluate_skill_trigger_formula(&formula3, &card));
        assert_eq!(4,  evaluate_skill_trigger_formula(&formula4, &card));

        // TODO evolve card and test again.
        let card = Card::stub_build(
            Arc::new(empty_template()),
            Attack::new(285),
            HealthPoints::new(1200),
            CardLevel::new(1),
            skill_builder(None, None, None), // not using this one-testing the
            // other one
            Tier::Tier4,
            Stars::FourStars
        );

        assert_eq!(2,  evaluate_skill_trigger_formula(&formula, &card));
        assert_eq!(4,  evaluate_skill_trigger_formula(&formula2, &card));
        assert_eq!(6,  evaluate_skill_trigger_formula(&formula3, &card));
        assert_eq!(8,  evaluate_skill_trigger_formula(&formula4, &card));
    }
}