use bevy::prelude::*;

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Tombstone".to_string()));
}

fn hello_world() {
    println!("hello, world!")
}

fn main() {
    App::new().add_system(hello_world).run();
}
