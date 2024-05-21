use crate::character::{Animations, Character, CharacterBuilder};
use bevy::prelude::*;

pub struct Simple3D;

impl Plugin for Simple3D {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, setup_scene_once_loaded)
            .add_systems(Update, handle_input);
    }
}

// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    //character animation
    // idle: 6
    // walking: 8
    let mut animationVec = vec![];
    for i in 0..9 {
        animationVec.push(
            asset_server.load("low_poly_character/girl.glb#Animation".to_owned() + &i.to_string()),
        );
    }
    commands.insert_resource(Animations(animationVec.clone()));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 8., 8.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

    // character
    let character: Character = CharacterBuilder::new()
        .name("character".to_string())
        .model(SceneBundle {
            scene: asset_server.load("low_poly_character/girl.glb#Scene0"),
            ..Default::default()
        })
        .animations(Animations(animationVec.clone()))
        .build();

    commands.spawn(character.model);
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[6].clone_weak()).repeat();
    }
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
) {
    for mut player in &mut animation_players {
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            player.play(animations.0[8].clone_weak()).repeat();
        }

        if keyboard_input.just_released(KeyCode::KeyS) {
            player.play(animations.0[6].clone_weak()).repeat();
        }
    }
}
