use crate::domain::Stars;

#[derive(Debug, PartialEq, Clone, Ord, PartialOrd, Eq)]
pub struct Tier(u8);

impl Tier{
    pub fn init() -> Self{
        Tier(1)
    }

    // I'm just lazy this function shouldn't exist but... it's handy.
    pub fn new(value: u8) -> Self{
        Tier(value)
    }

    pub fn value(&self)->u8 {
        self.0
    }

    pub fn max_tier(stars: &Stars) -> Self {
        match stars{
            Stars::OneStar => Tier(2),
            Stars::TwoStars => Tier(3),
            _  => Tier(4),
        }
    }

    pub fn vec_tier(stars: &Stars) -> Vec<Tier> {
        match stars{
            Stars::OneStar => vec![Tier(1), Tier(2)],
            Stars::TwoStars => vec![Tier(1), Tier(2), Tier(3)],
            _  => vec![Tier(1), Tier(2), Tier(3), Tier(4)]
        }
    }

    pub fn evolve(tier: Tier, tier2: Tier) -> Result<Tier, TierError> {
        if tier != tier2 {
            return Err(TierError::MismatchedTiers)
        }
        Ok(Tier{0: tier.0 + 1})
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
        assert_eq!(tier.value(), 1);
    }

    #[test]
    fn test_max_tier() {
        let stars = Stars::OneStar;
        let tier = Tier::max_tier(&stars);
        assert_eq!(tier.value(), 2);

        let stars = Stars::TwoStars;
        let tier = Tier::max_tier(&stars);
        assert_eq!(tier.value(), 3);

        let stars = Stars::ThreeStars;
        let tier = Tier::max_tier(&stars);
        assert_eq!(tier.value(), 4);

        let stars = Stars::FiveStars;
        let tier = Tier::max_tier(&stars);
        assert_eq!(tier.value(), 4);
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
        assert_eq!(evolved_tier.value(), 2);
    }

    #[test]
    fn cannot_evolve_different_tiers() {
        let tier = Tier::init();
        let tier2 = Tier::init();
        let evolved_tier = Tier::evolve(tier, tier2).unwrap();
        let result = Tier::evolve(Tier::init(), evolved_tier);
        assert_eq!(result, Err(TierError::MismatchedTiers));
    }
}