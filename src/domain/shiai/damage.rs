use super::*;
#[derive(Debug, Clone, PartialEq)]
pub enum Damage {
    Physical(PhysicalDamage),
    Magical
}

// TODO refactor to value objects in the future
#[derive(Debug, Clone, PartialEq)]
pub enum PhysicalDamage{
    Attack(u128),
    Reflected(u128)
}

#[derive(Debug, Clone, PartialEq)]
pub struct DamageReceived {
    pub team: ShiaiPosition,
    pub damage: Damage
}

impl Damage {
    pub fn new_attack_damage(amount: BattleTeamAttack) -> Self {
        Damage::Physical(
            Attack(amount.value())
        )
    }

    // TODO this constructor should need the skill.
    pub fn new_reflected_damage(amount: BattleTeamAttack) -> Self {
        Damage::Physical(
            Reflected(amount.value())
        )
    }

    pub fn value(&self) -> u128 {
        match self {
            Damage::Physical(physical_damage) => {
                match physical_damage {
                    Attack(amount) => *amount,
                    Reflected(amount) => *amount
                }
            }
            Damage::Magical => 0
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_damage() {
        let damage = Damage::new_attack_damage(BattleTeamAttack::new(200));
        assert_eq!(damage.value(), 200);

        assert!(matches!(damage, Damage::Physical(_)));
        if let Damage::Physical(inner) = damage {
            assert!(matches!(inner, Attack(_)));
        }
    }

    #[test]
    fn test_reflected_damage() {
        let damage = Damage::new_reflected_damage(BattleTeamAttack::new(200));
        assert_eq!(damage.value(), 200);

        assert!(matches!(damage, Damage::Physical(_)));
        if let Damage::Physical(inner) = damage {
            assert!(matches!(inner, Reflected(_)));
        }
    }
}