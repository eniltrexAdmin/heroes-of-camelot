use crate::domain::Stars;

#[derive(Debug, PartialEq, Clone, Ord, PartialOrd, Eq)]
pub enum Tier {
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}

impl Tier {
    pub fn init() -> Self {
        Tier::Tier1
    }

    pub fn int_value(&self) -> u8 {
        match self {
            Tier::Tier1 => 1,
            Tier::Tier2 => 2,
            Tier::Tier3 => 3,
            Tier::Tier4 => 4,
        }
    }

    pub fn vec_tier(stars: &Stars) -> Vec<Tier> {
        match stars {
            Stars::OneStar => vec![Tier::Tier1, Tier::Tier2],
            Stars::TwoStars => vec![Tier::Tier1, Tier::Tier2, Tier::Tier3],
            _ => vec![Tier::Tier1, Tier::Tier2, Tier::Tier3, Tier::Tier4],
        }
    }

    pub fn evolve(tier: Tier, tier2: Tier) -> Result<Tier, TierError> {
        if tier != tier2 {
            return Err(TierError::MismatchedTiers);
        }
        match tier {
            Tier::Tier1 => Ok(Tier::Tier2),
            Tier::Tier2 => Ok(Tier::Tier3),
            Tier::Tier3 => Ok(Tier::Tier4),
            Tier::Tier4 => Err(TierError::MaxTierReached),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TierError {
    MismatchedTiers,
    MaxTierReached,
    InvalidTier(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_tier_is_one() {
        let tier = Tier::init();
        assert_eq!(tier, Tier::Tier1);
    }

    #[test]
    fn test_vec_tier() {
        let stars = Stars::OneStar;
        let tier = Tier::vec_tier(&stars);
        assert_eq!(tier.len(), 2);

        let stars = Stars::TwoStars;
        let tier = Tier::vec_tier(&stars);
        assert_eq!(tier.len(), 3);

        let stars = Stars::ThreeStars;
        let tier = Tier::vec_tier(&stars);
        assert_eq!(tier.len(), 4);

        let stars = Stars::FiveStars;
        let tier = Tier::vec_tier(&stars);
        assert_eq!(tier.len(), 4);
    }

    #[test]
    fn test_evolve() {
        let tier = Tier::init();
        let tier2 = Tier::init();
        let evolved_tier = Tier::evolve(tier, tier2).unwrap();
        assert_eq!(evolved_tier, Tier::Tier2);
    }

    #[test]
    fn cannot_evolve_different_tiers() {
        let result = Tier::evolve(Tier::Tier1, Tier::Tier2);
        assert_eq!(result, Err(TierError::MismatchedTiers));
    }

    #[test]
    fn cannot_evolve_max_tier() {
        let result = Tier::evolve(Tier::Tier4, Tier::Tier4);
        assert_eq!(result, Err(TierError::MaxTierReached));
    }
}
