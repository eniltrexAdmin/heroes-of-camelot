use std::path::PathBuf;
use async_trait::async_trait;
use macroquad::miniquad::FilterMode;
use macroquad::prelude::{load_texture, Texture2D};
use slug::slugify;
use crate::domain::{Card, Team};
use crate::macroquad::CardTexturesRepository;
use crate::macroquad::CardTextures;

pub struct LocalCardTexturesRepository {
    asset_root: PathBuf,
    background: Texture2D,
}

impl LocalCardTexturesRepository {
    pub fn new(asset_root: PathBuf) -> LocalCardTexturesRepository {
        let bytes = include_bytes!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/card_background.png")
        );
        let background = Texture2D::from_file_with_format(bytes, None);
        background.set_filter(FilterMode::Linear);
        Self{asset_root, background}
    }

    async fn load_card_texture(&self, card: &Card) -> Texture2D {
        let texture_path = format!("{}/{}_{}.png",
            self.asset_root.to_str().unwrap(),
            slugify(card.name().value()),
            card.tier().int_value()
        );
        match load_texture(texture_path.as_str()).await {
            Ok(texture) => texture,
            Err(err) => {
                println!("Failed to load texture: {}, {}", err, texture_path);
                self.background.clone()
            }
        }
    }


}

#[async_trait]
impl CardTexturesRepository for LocalCardTexturesRepository {
    async fn load_for_team(&self, team: &Team) -> CardTextures {
        let second_card_texture = match team.second() {
            Some(second_card) => { Some(self.load_card_texture(second_card).await)},
            None => None
        };

        let third_card_texture = match team.third() {
            Some(second_card) => { Some(self.load_card_texture(second_card).await)},
            None => None
        };

        let fourth_card_texture = match team.fourth() {
            Some(second_card) => { Some(self.load_card_texture(second_card).await)},
            None => None
        };

       CardTextures{
            background: self.background.clone(),
            captain_template_texture: self.load_card_texture(team.captain()).await,
            second_card_texture,
            third_card_texture,
            fourth_card_texture,
        }
    }
}