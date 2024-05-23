use super::component::{Character, Player};
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use std::f32::consts::PI;

pub struct Animations(pub Vec<Handle<AnimationClip>>);

pub fn on_load(
    mut events: EventReader<SceneInstanceReady>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut characters: Query<&mut Character>,
) {
    let char = characters.get_single_mut().expect("Character not found");

    for _event in events.read() {
        for mut player in &mut animation_players {
            player.play(char.animations.0[0].clone_weak()).repeat();
        }
    }
}

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut characters: Query<&mut Character>,
    mut transforms: Query<&mut Transform, With<Player>>,
) {
    let char = characters.get_single_mut().expect("Character not found");
    let mut transform = transforms
        .get_single_mut()
        .expect("Character Transform not found");

    for mut player in &mut animation_players {
        let mut direction = Vec3::ZERO;
        let mut new_rotation = transform.rotation;

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0., 0., 1.);
            new_rotation = Quat::from_rotation_y(0.);
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0., 0., -1.);
            new_rotation = Quat::from_rotation_y(PI);
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1., 0., 0.);
            new_rotation = Quat::from_rotation_y(-PI / 2.);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1., 0., 0.);
            new_rotation = Quat::from_rotation_y(PI / 2.);
        }

        // Play walking animation on key pressed
        if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD])
        {
            player.play(char.animations.0[1].clone_weak()).repeat();
        }

        // Play idle animation on key released
        if keyboard_input.any_just_released([
            KeyCode::KeyW,
            KeyCode::KeyA,
            KeyCode::KeyS,
            KeyCode::KeyD,
        ]) {
            player.play(char.animations.0[0].clone_weak()).repeat();
        }

        let new_translation = calculate_transform(char.speed, &time, direction);

        transform.translation += new_translation;
        transform.rotation = new_rotation;
    }
}

fn calculate_transform(speed: f32, time: &Res<Time>, direction: Vec3) -> Vec3 {
    direction * speed * time.delta_seconds()
}
