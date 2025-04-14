use crate::domain::{Stars, Tier};

#[derive(Debug, Clone, PartialEq)]
pub struct CardLevel(u8);

impl CardLevel {
    pub fn init() -> Self {
        Self(1)
    }

    pub fn level_up(self) -> Self {
        Self(self.0 + 1)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn max_level(stars: &Stars, tier: &Tier) -> Self {
        let level = (stars.value() + tier.value() - 1) * 10;
        if tier.value() == 4 {
            return Self(level + 10)
        }
        Self(level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let level = CardLevel::init();
        assert_eq!(level.value(), 1);
    }

    #[test]
    fn test_level_up() {
        let level = CardLevel::init();
        let level = level.level_up();
        assert_eq!(level.value(), 2);
    }

    #[test]
    fn test_max_level() {
        let stars = Stars::OneStar;
        let tier = Tier::init();
        let level = CardLevel::max_level(&stars, &tier);
        assert_eq!(level.value(), 10);

        let tier_2 = Tier::new(2);
        let level = CardLevel::max_level(&stars, &tier_2);
        assert_eq!(level.value(), 20);

        let tier_4 = Tier::new(4);
        let level = CardLevel::max_level(&Stars::SevenStars, &tier_4);
        assert_eq!(level.value(), 110);
    }
}