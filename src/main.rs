use std::path::PathBuf;
use macroquad::prelude::*;
use heroes_of_camelot::macroquad::presentation_state::PresentationState;
use heroes_of_camelot::macroquad::StackState;
use heroes_of_camelot::macroquad::textures_loader::LocalCardTexturesRepository;

#[macroquad::main(window_conf)]
async fn main() {

    let asset_root = std::env::var("HOC_ASSET_PATH")
        .ok()
        .map(PathBuf::from)
        .expect("Could not determine asset storage path");

    println!("Loading assets from: {:?}", asset_root);

    let repo = LocalCardTexturesRepository::new(asset_root);

    let mut stack_state = StackState::new(repo);
    let presentation_state = PresentationState::new();
    stack_state.push(Box::new(presentation_state));
    loop {
        stack_state.update().await;
        stack_state.draw();

        next_frame().await
    }
}

fn window_conf() -> Conf{
    Conf {
        window_title: "Heroes of Camelot".to_owned(),
        fullscreen: false,
        window_width: 1920,  // Wider than tall
        window_height: 1080,  // Matches a 16:9 landscape layout
        ..Default::default()
    }
}