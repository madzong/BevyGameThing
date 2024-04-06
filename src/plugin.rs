pub mod plugin {
    use bevy::prelude::*;

    pub struct HelloPlugin;

    impl Plugin for HelloPlugin {
        fn build(&self, app: &mut App) {
            app
                .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
                .add_systems(Startup, add_people)
                .add_systems(Update, greet_people);
        }
    }

    #[derive(Component)]
    struct Person;

    #[derive(Component)]
    struct Name(String);

    #[derive(Resource)]
    struct GreetTimer(Timer);

    fn add_people(mut commands: Commands) {
        commands.spawn((Person, Name(String::from("Bomba"))));
    }

    fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
        if timer.0.tick(time.delta()).just_finished() {
            for name in &query {
                println!("Hello {}", name.0);
            }
        }
    }
}