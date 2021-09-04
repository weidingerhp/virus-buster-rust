use bevy::{input::keyboard::KeyboardInput, prelude::*, sprite::collide_aabb::{self, collide}};
use bevy_kira_audio::{AudioSource, Audio};

use crate::{GameStatus, virus::VirusParticle};

struct InjectionAssets {
    pluck: Handle<AudioSource>,
}

struct Injection;

pub struct InjectionPlugin;

impl Plugin for InjectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init.system())
        .add_system(move_injection.system());
    }
}

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.insert_resource(InjectionAssets {
        pluck: asset_server.load("Pluck.ogg"),
    });

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asset_server.load("injection.png").into()),
        transform: Transform {
            translation: Vec3::new(0., 0., 3.),
            scale: Vec3::new(0.5, 0.5, 1.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Injection);
}

fn move_injection(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    audio: Res<Audio>,
    assets: Res<InjectionAssets>,
    mut game_status: ResMut<GameStatus>,
    mut query: QuerySet<(
        Query<&mut Transform, With<Injection>>, 
        Query<(&Transform, &mut VirusParticle, &mut TextureAtlasSprite), With<VirusParticle>>
    )>
) {
    let mut injection_position: Option<Vec3> = None;
    let mut isInjecting = false;
    if let Ok(mut transform) = query.q0_mut().single_mut() {
        injection_position = Some(transform.translation.clone());
        if kb.just_pressed(KeyCode::Space) {
            audio.play(assets.pluck.clone());
            isInjecting = true;
        }
        if kb.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
        if kb.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if kb.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if kb.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
    }

    for (transform, mut particle, mut sprite) in query.q1_mut().iter_mut() {
        if isInjecting && !particle.is_hit{
            if has_hit(&injection_position.unwrap(), &transform.translation) {
                    sprite.index = 2;
                    particle.is_hit = true;

                    game_status.vaccinated += 1;

                    break; // only one hit counts .... 
            }
        }
    }
}

fn has_hit(
    injection_pos: &Vec3,
    virus_pos: &Vec3
) -> bool {
    let injection_tip_x = injection_pos.x + 45.;
    let injection_tip_y = injection_pos.y + 45.;

    injection_tip_x > virus_pos.x-32. && injection_tip_x < virus_pos.x+32. && injection_tip_y > virus_pos.y-32. && injection_tip_y < virus_pos.y+32. 
}