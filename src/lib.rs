mod virus;
mod injection;

use bevy::{ecs::component::Component, prelude::*};
use injection::InjectionPlugin;
use virus::VirusPlugin;
use wasm_bindgen::prelude::*;

pub struct GameStatus {
    infected: u16,
    vaccinated: u16,
}

struct VaccinatedText;

struct InfectedText;

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
    .add_plugin(InjectionPlugin)
    .add_startup_system(game_init.system())
    .add_system(update_text_infected.system())
    .add_system(update_text_vaccinated.system());


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

    commands.insert_resource(GameStatus {
        infected: 0,
        vaccinated: 0,
    });

    // At least we want two items that show the actual game status (points)
    let font_handle: Handle<Font> = asset_server.load("ROBOTO-REGULAR.TTF");

    let text_style = TextStyle {
        font: font_handle,
        font_size: 20.,
        color: Color::BLACK,
        ..Default::default()
    };

    create_text(&mut commands, Vec3::new(-310.,230.,2.), "Geimpft: ".to_owned(), TextAlignment {
        horizontal: HorizontalAlign::Right,
        vertical: VerticalAlign::Bottom,
        ..Default::default()
    }, &text_style, VaccinatedText);

    create_text(&mut commands, Vec3::new(310.,230.,2.), "Infiziert: ".to_owned(), TextAlignment {
        horizontal: HorizontalAlign::Left,
        vertical: VerticalAlign::Bottom,
        ..Default::default()
    }, &text_style, InfectedText);
}

fn create_text<C>(commands: &mut Commands, translation:Vec3, title: String, alignment: TextAlignment, text_style: &TextStyle, component: C) where C: Component {
    commands.spawn_bundle(Text2dBundle {
        text: Text {
            alignment: alignment,
            sections: vec![TextSection {
                value: title,
                style: text_style.clone(),
            },
            TextSection {
                value: "0".to_owned(),
                style: text_style.clone(),
            }
            ]
        },
        transform: Transform {
            translation: translation,
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(component)
        .insert(Timer::from_seconds(0.1, true)); // this timer is just to throttle the Graphical updates a little.
}

fn update_text_infected(
    game_status: Res<GameStatus>, 
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut Timer), With<InfectedText>>
) {
    if let Ok((mut text, mut timer)) = query.single_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            text.sections[1].value = game_status.infected.to_string();
        }
    }
}

fn update_text_vaccinated(
    game_status: Res<GameStatus>, 
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut Timer), With<VaccinatedText>>
) {
    if let Ok((mut text, mut timer)) = query.single_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            text.sections[1].value = game_status.vaccinated.to_string();
        }
    }
}