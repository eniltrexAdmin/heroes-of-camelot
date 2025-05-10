use super::*;
#[derive(Debug, Clone, PartialEq)]
pub enum Damage {
    Physical(PhysicalDamage),
    Magical
}

// TODO refactor to value objects in the future
#[derive(Debug, Clone, PartialEq)]
pub enum PhysicalDamage{
    AttackDamage(u128),
    ReflectedDamage(u128)
}

impl Damage {
    pub fn new_attack_damage(amount: BattleTeamAttack) -> Self {
        Damage::Physical(
            AttackDamage(amount.value())
        )
    }

    // TODO this constructor should need the skill.
    pub fn new_reflected_damage(amount: BattleTeamAttack) -> Self {
        Damage::Physical(
            ReflectedDamage(amount.value())
        )
    }

    pub fn value(&self) -> u128 {
        match self {
            Damage::Physical(physical_damage) => {
                match physical_damage {
                    AttackDamage(amount) => *amount,
                    ReflectedDamage(amount) => *amount
                }
            }
            Damage::Magical => 0
        }
    }
}


impl PhysicalDamage {
    pub fn new_attack_damage(amount: BattleTeamAttack) -> Self {
        AttackDamage(amount.value())
    }

    // TODO this constructor should need the skill.
    pub fn new_reflected_damage(amount: BattleTeamAttack) -> Self {
        ReflectedDamage(amount.value())
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
            assert!(matches!(inner, AttackDamage(_)));
        }
    }

    #[test]
    fn test_reflected_damage() {
        let damage = Damage::new_reflected_damage(BattleTeamAttack::new(200));
        assert_eq!(damage.value(), 200);

        assert!(matches!(damage, Damage::Physical(_)));
        if let Damage::Physical(inner) = damage {
            assert!(matches!(inner, ReflectedDamage(_)));
        }
    }
}