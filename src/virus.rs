use bevy::prelude::*;
use bevy_kira_audio::{AudioSource, Audio};

use crate::GameStatus;

pub struct VirusAssets {
    virus: Handle<TextureAtlas>,
    cough1: Handle<AudioSource>,
    cough2: Handle<AudioSource>,
}

pub struct VirusSpawner;

pub struct VirusParticle {
    is_hit:bool,
    speed_x: f32,
    speed_y: f32,
    rotation: f32,
}

pub struct VirusPlugin;

impl Plugin for VirusPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_virus.system())
            .add_system(spawn_viruses.system())
            .add_system(virus_fly_loop.system());
    }
}

fn init_virus(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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

            // Create new Virus Entity
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: virus_assets.virus.clone(),
                sprite: TextureAtlasSprite {
                    index: 1,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(320., 100., 1.),
                    scale: Vec3::new(0.5, 0.5, 1.),
                    ..Default::default()
                },
                ..Default::default()
            }).insert(VirusParticle {
                is_hit: false,
                speed_y: -0.1,
                speed_x: -1.,
                rotation: 0.02,
            }).insert(Timer::from_seconds(0.02, true));

            audio.play(virus_assets.cough1.clone());
        }

    }
}

fn virus_fly_loop(
    mut commands: Commands,
    time: Res<Time>,
    mut gamestatus: ResMut<GameStatus>,
    mut query: Query<(Entity, &mut Timer, &mut Transform, &VirusParticle), With<VirusParticle>>,

) {
    for (entity, mut timer, mut transform, virus) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            if virus.is_hit {
                transform.translation.y -= 1.;
                if transform.translation.y < -200. {
                    commands.entity(entity).despawn();
                }
            } else {
                transform.translation.x += virus.speed_x;
                transform.translation.y += virus.speed_y;
                transform.rotate(Quat::from_rotation_z(virus.rotation));

                if transform.translation.x < -320. {
                    commands.entity(entity).despawn();
                    gamestatus.infected += 1;
                }
            }
        }
    }
}