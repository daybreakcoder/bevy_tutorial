use bevy::prelude::*;

#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

//#[derive(Component)]
pub struct Character {
    pub name: Name,
    pub model: SceneBundle,
    pub animations: Animations,
}

impl Character {
    // fn build(&self, app: &mut App) {
    //     app.add_systems(Startup, Self::setup);
    // }
    //
    // fn setup(asset_server: Res<AssetServer>) {}
}

pub struct CharacterBuilder {
    name: Option<Name>,
    model: Option<SceneBundle>,
    animations: Option<Animations>,
}

impl CharacterBuilder {
    pub fn new() -> CharacterBuilder {
        CharacterBuilder {
            name: None,
            model: None,
            animations: None,
        }
    }

    pub fn name(mut self, name: String) -> CharacterBuilder {
        self.name = Some(Name::new(name));
        self
    }

    pub fn model(mut self, model: SceneBundle) -> CharacterBuilder {
        self.model = Some(model);
        self
    }

    pub fn animations(mut self, animations: Animations) -> CharacterBuilder {
        self.animations = Some(animations);
        self
    }

    pub fn build(self) -> Character {
        Character {
            name: self.name.expect("Name was not provided"),
            model: self.model.expect("Model was not provided"),
            animations: self.animations.expect("Animations were not provided "),
        }
    }
}
