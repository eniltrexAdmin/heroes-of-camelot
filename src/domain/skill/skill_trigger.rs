use crate::domain::*;

pub type ProcRate = u32;

#[derive(Debug, Clone, PartialEq)]
pub enum SkillTrigger {
    PROC(ProcRate),
    BasedOnCard(SkillTriggerValueFormula),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkillTriggerValueFormula{
    TriggerBasedOnCardLevel(Percentage),
    TriggerBasedOnCardTier(Percentage)
}

pub fn trigger_effective_chance(card: &Card) -> ProcRate{
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::data::apprentice_template;
    use crate::domain::*;
    use crate::domain::stubs::{empty_template, stub_build_skill};
    use super::*;
    use rand::Rng;

    #[test]
    fn test_trigger_effective_chance() {
        let mut rng = rand::rng();
        let random_number: u32 = rng.random_range( ..100);
        let skill =  stub_build_skill(None, Some(SkillTrigger::PROC(random_number)), None);
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

        let skill =  stub_build_skill(
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
        let apprentice_template = apprentice_template();
        let mut card = Card::stub_build(
            Arc::new(empty_template()),
            Attack::new(285),
            HealthPoints::new(1200),
            CardLevel::new(1),
            stub_build_skill(None, None, None), // not using this one
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
        let apprentice_template = apprentice_template();
        let card = Card::stub_build(
            Arc::new(empty_template()),
            Attack::new(285),
            HealthPoints::new(1200),
            CardLevel::new(1),
            stub_build_skill(None, None, None), // not using this one
            // and just going faster knowing the "innards" of this code
            // the proper interface of this module is tested above
            // but that's a lot of boilerplate here creating a card for each test...
            // that's why I bypass it.
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
            stub_build_skill(None, None, None), // not using this one
            // and just going faster knowing the "innards" of this code
            // the proper interface of this module is tested above
            // but that's a lot of boilerplate here creating a card for each test...
            // that's why I bypass it.
            Tier::Tier4,
            Stars::FourStars
        );

        assert_eq!(2,  evaluate_skill_trigger_formula(&formula, &card));
        assert_eq!(4,  evaluate_skill_trigger_formula(&formula2, &card));
        assert_eq!(6,  evaluate_skill_trigger_formula(&formula3, &card));
        assert_eq!(8,  evaluate_skill_trigger_formula(&formula4, &card));
    }
}