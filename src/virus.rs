use bevy::prelude::*;
use bevy_kira_audio::{AudioSource, Audio};

pub struct VirusAssets {
    virus: Handle<TextureAtlas>,
    cough1: Handle<AudioSource>,
    cough2: Handle<AudioSource>,
}

pub struct VirusSpawner;

pub struct VirusParticle {
    is_hit:bool,
}

pub struct VirusPlugin;

impl Plugin for VirusPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_virus.system());
        app.add_system(spawn_viruses.system());
    }
}

fn init_virus(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut materialmap: ResMut<Assets<TextureAtlas>>,
) {
    let covid_map = asset_server.load("covid_map.png");

    commands.insert_resource(VirusAssets {
        virus: materialmap.add(TextureAtlas::from_grid(covid_map, Vec2::new(128.0, 128.0), 2, 2)),
        cough1: asset_server.load("cough1.ogg"),
        cough2: asset_server.load("cough2.ogg"),
    });

    commands.spawn()
    .insert(VirusSpawner)
    .insert(Timer::from_seconds(2., false));
}

fn spawn_viruses(
    mut commands: Commands,
    virus_assets: Res<VirusAssets>,
    time: Res<Time>,
    audio: Res<Audio>,
    mut query: Query<(&mut Timer, Entity), With<VirusSpawner>>,
) {

    if let Ok((mut timer, entity)) = query.single_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            commands.entity(entity).insert(Timer::from_seconds(3., false));

            audio.play(virus_assets.cough1.clone());
        }

    }
}