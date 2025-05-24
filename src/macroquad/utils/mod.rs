pub mod utils;
mod draw_card;
mod animated_rectangle;
mod responsive_rectangle;
pub use responsive_rectangle::RespRect;
pub use responsive_rectangle::BaseResolution;
pub use responsive_rectangle::BaseResolution::*;

pub use animated_rectangle::AnimatedRectangle;

pub use utils::*;