use bevy::prelude::*;

pub struct HelloPlugin;

#[derive(Component)]
struct Person {
    name: String,
}

#[derive(Resource)]
struct GreetTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        let timer = GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating));
        app.insert_resource(timer)
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn(Person {
        name: "Foo".to_string(),
    });
    commands.spawn(Person {
        name: "Bar".to_string(),
    });
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Person>) {
    if timer.0.tick(time.delta()).just_finished() {
        for person in &query {
            println!("Hello {}!", person.name);
        }
    }
}
