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
        let mut ten_levels = stars.value() + tier.int_value() - 1;

        if tier == &Tier::Tier4{
            ten_levels = ten_levels + 1
        }
        Self(ten_levels * 10)
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
        let level = CardLevel::max_level(&Stars::OneStar, &Tier::Tier1);
        assert_eq!(level.value(), 10);

        let level = CardLevel::max_level(&Stars::OneStar, &Tier::Tier2);
        assert_eq!(level.value(), 20);

        let level = CardLevel::max_level(&Stars::TwoStars, &Tier::Tier3);
        assert_eq!(level.value(), 40);

        let level = CardLevel::max_level(&Stars::SevenStars, &Tier::Tier4);
        assert_eq!(level.value(), 110);
    }
}