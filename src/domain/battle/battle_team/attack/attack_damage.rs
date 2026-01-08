use super::*;
#[derive(Debug, Clone, PartialEq)]
pub enum AttackDamage {
    Physical(PhysicalDamage),
    Magical
}

// TODO refactor to value objects in the future
#[derive(Debug, Clone, PartialEq)]
pub enum PhysicalDamage{
    RegularDamage(u128),
    ReflectedDamage(u128)
}

impl AttackDamage {
    pub fn new_attack_damage(amount: u128) -> Self {
        AttackDamage::Physical(
            PhysicalDamage::RegularDamage(amount)
        )
    }

    // for the future.
    pub fn new_reflected_damage(amount: BattleTeamAttack) -> Self {
        AttackDamage::Physical(
            PhysicalDamage::ReflectedDamage(amount.value())
        )
    }

    pub fn value(&self) -> u128 {
        match self {
            AttackDamage::Physical(physical_damage) => {
                match physical_damage {
                    PhysicalDamage::RegularDamage(amount) => *amount,
                    PhysicalDamage::ReflectedDamage(amount) => *amount
                }
            }
            AttackDamage::Magical => 0
        }
    }
}


impl PhysicalDamage {
    pub fn new_attack_damage(amount: BattleTeamAttack) -> Self {
        PhysicalDamage::RegularDamage(amount.value())
    }

    // for the future
    pub fn new_reflected_damage(amount: BattleTeamAttack) -> Self {
        PhysicalDamage::ReflectedDamage(amount.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_damage() {
        let damage = AttackDamage::new_attack_damage(200);
        assert_eq!(damage.value(), 200);

        assert!(matches!(damage, AttackDamage::Physical(_)));
        if let AttackDamage::Physical(inner) = damage {
            assert!(matches!(inner, PhysicalDamage::RegularDamage(_)));
        }
    }

    // #[test]
    // fn test_reflected_damage() {
    //     let damage = Damage::new_reflected_damage(200);
    //     assert_eq!(damage.value(), 200);
    //
    //     assert!(matches!(damage, Damage::Physical(_)));
    //     if let Damage::Physical(inner) = damage {
    //         assert!(matches!(inner, ReflectedDamage(_)));
    //     }
    // }
}