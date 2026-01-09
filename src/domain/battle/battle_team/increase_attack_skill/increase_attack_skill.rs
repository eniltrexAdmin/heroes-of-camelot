use super::*;

impl BattleTeam{
    pub fn increase_turn_attack(&self, card: &Card, state: &BattleState) -> IncreasedThisTurnAttackDomainEvent {
        let value_increased = AttackIncreaseValue::new(
            card.skill_effect_base_value().value());
        // more modifiers depending on battle state, increase more/less, etc.
        IncreasedThisTurnAttackDomainEvent{
            card: card.clone(),
            target: self.position.clone(),
            increase_by: value_increased,
        }
    }
}

// Card seems to be needed only because animation.

// All skills will have Card, it's after all a Card that
// performs.

// I will need a "match" at some point, but I'd like a
// smarter way to call the appropriate function

// also to "englobe" it in the variant of BattleEvent enum.

// I don't like passing card, as it's a "given", and I find it
// wrong that I pass card, card has the skill, so it knows
// which funciton I should call, and then in the funciton
// I am calling, it needs card -> it gives tentation to add
// the typical check: "if card->skill()->skillEffect is not
// the expected, panic. But I'd like a way to call the function
// without worrying about that. Not making it that flexible.
// even if I pass the skill, it's also an enum and
// I can still have the check "if skill()->SkillEffect".
// it's no the expected, panic.
//
// On the other hand, if I don't apss card, and I just apply
// the "increase attack" blindly to "self", feels unsafe/confusing
// I am increasing the attack of a team just because I receive
// a "skill effect" disattached from a card? and I need to pass
// the card anyway to calculate the formula.

// the way I have it now though it my least unliked.  (how to get the formula?)