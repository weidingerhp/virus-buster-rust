mod virus;

use bevy::{asset::Asset, prelude::*};
use bevy_kira_audio::AudioPlugin;
use virus::VirusPlugin;
use wasm_bindgen::prelude::*;

struct BackgroundRessources {
    background: Handle<ColorMaterial>,
}

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();
    app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor {
        title: "Coderdojo - Virus Buster".to_string(),
        width: 640.0,
        height: 480.0,
        scale_factor_override: Some(1.5),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(bevy_kira_audio::AudioPlugin)
    .add_plugin(VirusPlugin)
    .add_startup_system(game_init.system());


    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}

fn game_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asset_server.load("blue_sky.png").into()),
        ..Default::default()
    });

}