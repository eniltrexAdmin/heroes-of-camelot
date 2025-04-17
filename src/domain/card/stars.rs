use crate::domain::Tier;

#[derive(Debug, PartialEq)]
pub enum Stars {
    OneStar,
    TwoStars,
    ThreeStars,
    FourStars,
    FiveStars,
    SixStars,
    SevenStars,
}

impl Stars {
    pub fn new(stars: u8) -> Result<Stars, StarError> {
        match stars {
            1 => Ok(Self::OneStar),
            2 => Ok(Self::TwoStars),
            3 => Ok(Self::ThreeStars),
            4 => Ok(Self::FourStars),
            5 => Ok(Self::FiveStars),
            6 => Ok(Self::SixStars),
            7 => Ok(Self::SevenStars),
            _ => Err(StarError::InvalidNumberOfStars)
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Self::OneStar => 1,
            Self::TwoStars => 2,
            Self::ThreeStars => 3,
            Self::FourStars => 4,
            Self::FiveStars => 5,
            Self::SixStars => 6,
            Self::SevenStars => 7,
        }
    }

    pub fn max_tier(&self) -> &Tier {
        match self{
            Stars::OneStar => &Tier::Tier2,
            Stars::TwoStars => &Tier::Tier3,
            _  => &Tier::Tier4,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StarError {
    InvalidNumberOfStars,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_star_constructor() {
        let result = Stars::new(1);
        assert!(result.is_ok());
        assert_eq!(Stars::OneStar, result.unwrap());
    }

    #[test]
    pub fn test_star_constructor_error() {
        let result = Stars::new(20);
        assert!(result.is_err());
        assert_eq!(StarError::InvalidNumberOfStars, result.unwrap_err());
    }


    #[test]
    fn test_max_tier() {
        let stars = Stars::OneStar;
        assert_eq!(stars.max_tier(), &Tier::Tier2);

        let stars = Stars::TwoStars;
        assert_eq!(stars.max_tier(), &Tier::Tier3);

        let stars = Stars::ThreeStars;
        assert_eq!(stars.max_tier(), &Tier::Tier4);

        let stars = Stars::FiveStars;
        assert_eq!(stars.max_tier(), &Tier::Tier4);
    }
}