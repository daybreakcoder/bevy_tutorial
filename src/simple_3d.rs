use crate::character::component::*;
use crate::character::system::{handle_input, on_load};
use bevy::prelude::*;

pub struct Simple3D;

impl Plugin for Simple3D {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(SpawnScene, on_load)
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

    //light
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
    let character: CharacterBundle = CharacterBundleBuilder::new()
        .name("character".to_string())
        .speed(1.)
        .model("low_poly_character/girl.glb#Scene0".to_string())
        .animations(vec![
            "low_poly_character/girl.glb#Animation6".to_string(), // idle
            "low_poly_character/girl.glb#Animation8".to_string(), // walking
        ])
        .transform(Transform::from_xyz(0., 0., 0.))
        .asset_server(asset_server)
        .build();
    commands.spawn((character, Player {}));
}
