use crate::character::system::Animations;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Character {
    pub name: String,
    pub animations: Animations,
    pub speed: f32,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub character: Character,
    pub model: SceneBundle,
}

pub struct CharacterBundleBuilder<'a> {
    name: Option<String>,
    speed: Option<f32>,
    model: Option<String>,
    animations: Option<Vec<String>>,
    transform: Option<Transform>,
    asset_server: Option<Res<'a, AssetServer>>,
}

impl<'a> CharacterBundleBuilder<'a> {
    pub fn new() -> CharacterBundleBuilder<'a> {
        CharacterBundleBuilder {
            name: None,
            speed: None,
            model: None,
            animations: None,
            transform: None,
            asset_server: None,
        }
    }

    pub fn name(mut self, name: String) -> CharacterBundleBuilder<'a> {
        self.name = Some(name);
        self
    }

    pub fn speed(mut self, speed: f32) -> CharacterBundleBuilder<'a> {
        self.speed = Some(speed);
        self
    }

    pub fn model(mut self, model: String) -> CharacterBundleBuilder<'a> {
        self.model = Some(model);
        self
    }

    pub fn animations(mut self, animations: Vec<String>) -> CharacterBundleBuilder<'a> {
        self.animations = Some(animations);
        self
    }

    pub fn transform(mut self, transform: Transform) -> CharacterBundleBuilder<'a> {
        self.transform = Some(transform);
        self
    }

    pub fn asset_server(
        mut self,
        asset_server: Res<'a, AssetServer>,
    ) -> CharacterBundleBuilder<'a> {
        self.asset_server = Some(asset_server);
        self
    }

    pub fn build(self) -> CharacterBundle {
        let (input_name, input_speed, model, animations, input_transform, asset_server) =
            self.validate_attributes();

        let model_bundle = SceneBundle {
            scene: asset_server.load(model),
            transform: input_transform,
            ..Default::default()
        };

        let mut animation_vector = vec![];
        for animation in animations.iter() {
            animation_vector.push(asset_server.load(animation));
        }

        let input_character = Character {
            name: input_name,
            speed: input_speed,
            animations: Animations(animation_vector),
        };

        CharacterBundle {
            character: input_character,
            model: model_bundle,
        }
    }

    fn validate_attributes(
        self,
    ) -> (
        String,
        f32,
        String,
        Vec<String>,
        Transform,
        Res<'a, AssetServer>,
    ) {
        (
            self.name.expect("Name was not provided"),
            self.speed.expect("Speed was not provided"),
            self.model.expect("Model was not provided"),
            self.animations.expect("Animations were not provided"),
            self.transform.expect("Transform was not provided"),
            self.asset_server.expect("asset_server was not provided"),
        )
    }
}
